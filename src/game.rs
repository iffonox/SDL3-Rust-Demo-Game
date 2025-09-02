use crate::game_object::world::World;
use crate::game_object::{Drawable, GameObject, PhysicsVector};
use sdl3::event::Event;
use sdl3::keyboard::{Keycode, Mod};
use sdl3::pixels::{Color};
use sdl3::render::{FPoint, FRect, TextureCreator, WindowCanvas};
use sdl3::surface::Surface;
use sdl3::ttf::{Font, Sdl3TtfContext};
use sdl3::{Sdl};
use std::collections::{HashMap};
use std::fs::read_dir;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use sdl3::libc::{rand, RAND_MAX};
use sdl3::video::WindowContext;
use crate::math::bounds::Bounds;
use bitmask_enum::bitmask;
use crate::game_object::behaviour::controllable::ControllableBehaviour;
use crate::game_object::behaviour::dvd::DvdBehaviour;

static FPS_LIMIT: u64 = 60;
static MIN_FRAME_TIME: u64 = 1_000u64 / FPS_LIMIT;

#[bitmask(u32)]
pub enum Action {
    None = 0,
    Quit = 1,
    Debug = 2,
    FpsLimit = 4,
    MoveLeft = 8,
    MoveRight = 16,
    MoveUp = 32,
    MoveDown = 64,
    Duck = 128,
    Jump = 256,
    Sprint = 512,
    Attack = 1024,
}

pub struct Game<'a> {
    keymap: HashMap<Keycode, Action>,
    actions: Action,
    sdl_context: &'a Sdl,
    texture_creator: &'a TextureCreator<WindowContext>,
    canvas: &'a mut WindowCanvas,
    ttf_context: &'a Sdl3TtfContext,
    font: Font<'a>,
	surfaces: Vec<Surface<'a>>,
    texture_names: Vec<String>,
    keymod: Mod,
    world: World,
    bg_color: Color,
    last_tick: u64,
    frame_time: u64,
    frame_number: u64,
    fps_frame_count: u64,
    fps_last_tick: u64,
    fps: f32,
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
        let mut surfaces = Vec::new();
        let mut texture_names = Vec::new();

        let dir = read_dir(Path::new("./assets/textures/")).expect("readdir error");

        for res in dir {
            if let Ok(entry) = res {
                let path = entry.path();

                if path.extension().unwrap() == "bmp" {
                    let name = path.file_name().expect("file name error");
                    let name1 = name.to_str().expect("name error");
                    let surface = Surface::load_bmp(entry.path()).expect("image load error");

					surfaces.push(surface);
					texture_names.push(String::from(name1));
                }
            }
        }

        Self {
            keymap: HashMap::new(),
            actions: Action::None,
            texture_creator,
            ttf_context,
			surfaces,
            texture_names,
            keymod: Mod::NOMOD,
            world: World::new(width as f32, height as f32),
            sdl_context,
            canvas,
            font: ttf_context
                .load_font(font_path, 12.0)
                .expect("Font loading error"),
            bg_color: Color::RGB(255, 255, 255),
            last_tick: 0,
            frame_time: 0,
            frame_number: 0,
            fps_frame_count: 0,
            fps_last_tick: 0,
            fps: 0.0,
            should_quit: false,
            should_wait_after_frame: true,
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

        let num_textures = self.surfaces.len().clamp(0, 3);
        let (width, height) = self.canvas.output_size().expect("output size error");
        let bounds = FRect {
            x: 0.0,
            y: 0.0,
            w: width as f32,
            h: height as f32,
        };

        for i in 0..10 {
            let pos = i as f32 * 30.0;
            let mut texture_index = 0;

            let mut speed = PhysicsVector::default();

            unsafe {
                speed.x = rand() as f32 / RAND_MAX as f32 * 200.0;
                speed.y = rand() as f32 / RAND_MAX as f32 * 200.0;
            }

            if num_textures > 0 {
                texture_index = i % num_textures;
            }

            let mut game_object = GameObject::new(i as i32);
            game_object.bounds = FRect {
                x: pos,
                y: pos,
                w: 60.0,
                h: 60.0,
            };
            game_object.drawable = Some(Drawable {
                z: i as i32,
                color: Color::RGB(25 * i as u8, 0, 255 - i as u8 * 25),
                texture: Some(texture_index),
				tint_texture: false,
            });
            game_object.behaviours.push(Box::new(DvdBehaviour::new(bounds, speed)));

            self.world.add_game_object(game_object);
        }

        let mut game_object = GameObject::new(100);
        game_object.bounds = FRect {
            x: 0.0,
            y: 0.0,
            w: 60.0,
            h: 60.0,
        };
        game_object.drawable = Some(Drawable {
            z: 100,
            color: Color::MAGENTA,
            texture: Some(3),
			tint_texture: true,
        });
        game_object.behaviours.push(Box::new(ControllableBehaviour::new(bounds, 50.0, 200.0)));

        self.world.add_game_object(game_object);
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
                    self.actions |= *action;
                }
            }
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => {
                if let Some(action) = self.keymap.get(&keycode) {
                    self.actions &= (*action).not();
                }
            }
            Event::Quit { .. } => {
                self.actions |= Action::Quit;
            }
            _ => {}
        }
    }

    fn handle_system_events(&mut self) {
        if self.actions.contains(Action::Quit) {
            self.should_quit = true;
            return;
        }

        if self.actions.contains(Action::FpsLimit) {
            self.actions &= Action::FpsLimit.not();
            self.should_wait_after_frame = !self.should_wait_after_frame
        }

        if self.actions.contains(Action::Debug) {
            self.actions &= Action::Debug.not();
            self.should_show_debug = !self.should_show_debug
        }
    }

    fn render_drawables(&mut self) {
        let drawables = self.world.get_drawables();
        let num_textures = self.surfaces.len();

        for (rect, drawable) in drawables {
			if let Some(texture_index) = drawable.texture && texture_index < self.surfaces.len() {
                let surface = &self.surfaces[texture_index];

				if let Ok(mut texture) = self.texture_creator.create_texture_from_surface(surface) {
					if drawable.tint_texture {
						texture.set_color_mod(drawable.color.r, drawable.color.g, drawable.color.b);
					}

					self.canvas
						.copy(&texture, None, rect)
						.expect("texture error");
				}
            } else {
				self.canvas.set_draw_color(drawable.color);
				self.canvas.fill_rect(rect).expect("draw error");
			}
        }
    }

    fn tick(&mut self) {
        let now = sdl3::timer::ticks();

        if self.last_tick == 0 {
            self.last_tick = now;
            self.fps_last_tick = now;
            return;
        }

        let delta_t = now - self.last_tick;

        self.world.tick(delta_t, self.actions);

        self.canvas.set_draw_color(self.bg_color);
        self.canvas.clear();

        self.render_drawables();

        if self.should_show_debug {
            self.render_debug_msg(delta_t);
        }

        self.canvas.present();

        self.last_tick = now;
        self.keymod = Mod::NOMOD;
        self.frame_number += 1;
        self.fps_frame_count += 1;
        self.frame_time = sdl3::timer::ticks() - now;

        let fps_time = now - self.fps_last_tick;
        if fps_time > 1000 {
            self.fps = self.fps_frame_count as f32 / fps_time as f32 * 1000.0;

            self.fps_last_tick = now;
            self.fps_frame_count = 0;
        }

        if !self.should_wait_after_frame {
            return;
        }

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
        let fn_rect = self.render_msg(&fn_text, FPoint::new(dt_rect.x, dt_rect.bottom()));

        let ft_text = format!("frame_time: {}ms", self.frame_time);
        let ft_rect = self.render_msg(&ft_text, FPoint::new(fn_rect.x, fn_rect.bottom()));

        let fw_text = format!("fps_limit: {}", self.should_wait_after_frame);
        let fw_rect = self.render_msg(&fw_text, FPoint::new(ft_rect.x, ft_rect.bottom()));

        let fps_text = format!("fps: {}", self.fps);
        self.render_msg(&fps_text, FPoint::new(fw_rect.x, fw_rect.bottom()));
    }
}
