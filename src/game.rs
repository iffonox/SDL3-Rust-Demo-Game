use crate::game_object::world::World;
use crate::game_object::{Drawable, GameObject};
use sdl3::event::Event;
use sdl3::keyboard::{Keycode, Mod};
use sdl3::pixels::Color;
use sdl3::render::{FPoint, FRect, Texture, TextureCreator, WindowCanvas};
use sdl3::surface::Surface;
use sdl3::ttf::{Font, Sdl3TtfContext};
use sdl3::{Sdl, ttf};
use std::collections::{HashMap, HashSet};
use std::fs::read_dir;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use sdl3::video::WindowContext;

static FPS_LIMIT: u64 = 60;
static MIN_FRAME_TIME: u64 = 1_000u64 / FPS_LIMIT;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(i32)]
pub enum Action {
    Quit,
    Debug,
    FpsLimit,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Duck,
    Jump,
    Sprint,
    Attack,
}

pub struct Game<'a> {
    keymap: HashMap<Keycode, Action>,
    actions: HashSet<Action>,
    sdl_context: &'a Sdl,
    texture_creator: &'a TextureCreator<WindowContext>,
    canvas: &'a mut WindowCanvas,
    ttf_context: &'a Sdl3TtfContext,
    font: Font<'a>,
    textures: Vec<Texture<'a>>,
    keymod: Mod,
    last_tick: u64,
    world: World,
    frame_time: u64,
    frame_number: u64,
    bg_color: Color,
    should_quit: bool,
    should_wait_after_frame: bool,
    should_show_debug: bool,
}

impl<'a> Game<'a> {
    pub fn new(
        width: u32,
        height: u32,
        sdl_context: &'a Sdl,
        canvas: &'a mut WindowCanvas,
        texture_creator: &'a TextureCreator<WindowContext>,
        ttf_context: &'a Sdl3TtfContext
    ) -> Self {

        let font_path = Path::new("./assets/fonts/static/JetBrainsMono-Medium.ttf");

        // Load textures
        let mut textures = Vec::new();

        let dir = read_dir(Path::new("./assets/textures/")).expect("readdir error");

        for res in dir {
            if let Ok(entry) = res {
                let file_name = entry.file_name();

                if file_name.into_string().unwrap().contains(".bmp") {
                    let surface = Surface::load_bmp(entry.path()).expect("image load error");

                    if let Ok(texture) = texture_creator.create_texture_from_surface(surface) {
                        textures.push(texture);
                    }
                }
            }
        }

        Self {
            keymap: HashMap::new(),
            actions: HashSet::new(),
            texture_creator,
            ttf_context,
            textures,
            keymod: Mod::NOMOD,
            last_tick: 0,
            world: World::new(width as f32, height as f32),
            sdl_context,
            canvas,
            frame_time: 0,
            frame_number: 0,
            font: ttf_context
                .load_font(font_path, 12.0)
                .expect("Font loading error"),
            bg_color: Color::RGB(255, 255, 255),
            should_quit: false,
            should_wait_after_frame: false,
            should_show_debug: false,
        }
    }

    fn init(&mut self) {
        // Load keymap, will later be loaded from a save file

        self.keymap.insert(Keycode::Escape, Action::Quit);
        self.keymap.insert(Keycode::F2, Action::Debug);
        self.keymap.insert(Keycode::F3, Action::FpsLimit);
        self.keymap.insert(Keycode::W, Action::MoveUp);
        self.keymap.insert(Keycode::A, Action::MoveLeft);
        self.keymap.insert(Keycode::S, Action::MoveDown);
        self.keymap.insert(Keycode::D, Action::MoveRight);
        self.keymap.insert(Keycode::Space, Action::Jump);
        self.keymap.insert(Keycode::LShift, Action::Sprint);
        self.keymap.insert(Keycode::LCtrl, Action::Duck);
        self.keymap.insert(Keycode::RCtrl, Action::Attack);

        // Load game objects, will later be loaded from level file modified by a save file

        let num_textures = self.textures.len();

        for i in 0..10 {
            let pos = i as f32 * 30.0;
            let mut texture_index = 0;

            if num_textures > 0 {
                texture_index = i % num_textures;
            }

            self.world.add_game_object(GameObject {
                id: 1,
                bounds: FRect {
                    x: pos,
                    y: pos,
                    w: 60.0,
                    h: 60.0,
                },
                drawable: Some(Drawable {
                    z: i as i32,
                    color: Color::RGB(25 * i as u8, 0, 255 - i as u8 * 25),
                    texture: texture_index,
                }),
                physics_body: None,
                action_handler: None,
            })
        }
    }

    pub fn run(&mut self) {
        self.init();

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'running: loop {
            if self.should_quit {
                break 'running;
            }

            for event in event_pump.poll_iter() {
                self.register_events(event);
            }

            self.handle_system_events();

            self.tick();
        }
    }

    fn register_events(&mut self, event: Event) {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                repeat: false,
                ..
            } => {
                if let Some(action) = self.keymap.get(&keycode) {
                    self.actions.insert(action.clone());
                }
            }
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => {
                if let Some(action) = self.keymap.get(&keycode) {
                    self.actions.remove(action);
                }
            }
            Event::Quit { .. } => {
                self.actions.insert(Action::Quit);
            }
            _ => {}
        }
    }

    fn handle_system_events(&mut self) {
        if self.actions.contains(&Action::Quit) {
            self.should_quit = true;
            return;
        }

        if self.actions.contains(&Action::FpsLimit) {
            self.actions.remove(&Action::FpsLimit);
            self.should_wait_after_frame = !self.should_wait_after_frame
        }

        if self.actions.contains(&Action::Debug) {
            self.actions.remove(&Action::Debug);
            self.should_show_debug = !self.should_show_debug
        }
    }

    fn render_drawables(&mut self, delta_t: u64) {
        let drawables = self.world.get_drawables(delta_t).clone();
        let num_textures = self.textures.len();

        for (rect, drawable) in drawables {
            self.canvas.set_draw_color(drawable.color);
            self.canvas.fill_rect(rect).expect("draw error");

            if drawable.texture < num_textures {
                let texture = &self.textures[drawable.texture];

                self.canvas
                    .copy(texture, None, rect)
                    .expect("texture error");
            }
        }
    }

    fn tick(&mut self) {
        let now = sdl3::timer::ticks();

        if self.last_tick == 0 {
            self.last_tick = now;
            return;
        }

        let delta_t = now - self.last_tick;

        self.world.tick_physics(delta_t);

        self.canvas.set_draw_color(self.bg_color);
        self.canvas.clear();

        self.render_drawables(delta_t);

        if self.should_show_debug {
            self.render_debug_msg(delta_t);
        }

        self.canvas.present();

        self.last_tick = now;
        self.keymod = Mod::NOMOD;
        self.frame_number += 1;
        self.frame_time = sdl3::timer::ticks() - now;

        if !self.should_wait_after_frame {
            return;
        }

        // limit to FPS_LIMIT
        if self.frame_time < MIN_FRAME_TIME {
            let wait_time = (MIN_FRAME_TIME - self.frame_time) * 1000_000;

            sleep(Duration::new(0, wait_time as u32));
        }
    }

    fn render_msg(&mut self, msg: &String, pos: FPoint) -> FRect {
        let rendered_text = self.font.render(msg);
        let surface = rendered_text
            .blended(Color::RGB(255, 255, 255))
            .expect("text render panic");

        let surface_width = surface.width() as f32;
        let surface_height = surface.height() as f32;

        let texture = self.texture_creator
            .create_texture_from_surface(surface)
            .expect("texture creation panic");

        let text_rect = FRect::new(pos.x + 5.0, pos.y + 2.0, surface_width, surface_height);

        let bg_rect = FRect::new(pos.x, pos.y, surface_width + 10.0, surface_height + 4.0);

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.fill_rect(bg_rect).expect("debug message panic");
        self.canvas
            .copy(&texture, None, Some(text_rect))
            .expect("debug message panic");

        bg_rect
    }

    fn render_debug_msg(&mut self, delta_t: u64) {
        let dt_text = format!("delta_t: {delta_t}ms");
        let dt_rect = self.render_msg(&dt_text, FPoint::new(0.0, 0.0));

        let fn_text = format!("frame_count: {}", self.frame_number);
        let fn_rect = self.render_msg(&fn_text, FPoint::new(dt_rect.x, dt_rect.y + dt_rect.h));

        let ft_text = format!("frame_time: {}ms", self.frame_time);
        let ft_rect = self.render_msg(&ft_text, FPoint::new(fn_rect.x, fn_rect.y + fn_rect.h));

        let fw_text = format!("fps_limit: {}", self.should_wait_after_frame);
        self.render_msg(&fw_text, FPoint::new(ft_rect.x, ft_rect.y + ft_rect.h));
    }
}
