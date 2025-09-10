use crate::game_object::world::World;
use crate::math::bounds::Bounds;
use crate::serialization::font::FontDefinition;
use crate::serialization::game::{GameData, LevelDefinition};
use crate::serialization::level::LevelData;
use crate::serialization::texture::TextureDefinition;
use crate::serialization::{Action, AssetId};
use sdl3::Sdl;
use sdl3::event::Event;
use sdl3::keyboard::{Keycode, Mod};
use sdl3::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl3::render::{FPoint, FRect, SurfaceCanvas, TextureCreator, WindowCanvas};
use sdl3::surface::{Surface};
use sdl3::timer::performance_frequency;
use sdl3::ttf::{Font, Sdl3TtfContext};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use sdl3::mouse::MouseButton;
use sdl3::video::WindowContext;

static FPS_LIMIT: u64 = 60;
static MIN_FRAME_TIME: u64 = 1_000u64 / FPS_LIMIT;

pub struct Game<'a> {
    keymap: HashMap<Keycode, Action>,
    actions: Action,
    game_data: GameData,
    sdl_context: &'a Sdl,
	main_texture_creator: TextureCreator<WindowContext>,
    main_canvas: WindowCanvas,
    menu_canvas: SurfaceCanvas<'a>,
    fonts: HashMap<AssetId, Font<'a>>,
    surfaces: HashMap<AssetId, Surface<'a>>,
    performance_frequency: f64,
    keymod: Mod,
    world: World,
    bg_color: Color,
    level_data: Vec<LevelData>,
    last_tick: u64,
    frame_time: u64,
    frame_number: u64,
    fps_frame_count: u64,
    fps_last_tick: u64,
    fps: f32,
	window_bounds: FRect,
	click: Option<FPoint>,
    should_quit: bool,
    should_wait_after_frame: bool,
    should_show_debug: bool,
	menu_open: bool,
}

impl<'a> Game<'a> {
    pub fn new(
        width: u32,
        height: u32,
        game_data: GameData,
        sdl_context: &'a Sdl,
        ttf_context: &'a Sdl3TtfContext,
    ) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl3 demo", width, height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas();

        let fonts = Self::load_fonts(&game_data.fonts, ttf_context);

        let surfaces = Self::load_surfaces(&game_data.textures);

        let keymap = Self::load_keymap();

        let level_data = Self::load_levels(&game_data.levels);

        let menu_canvas = SurfaceCanvas::from_surface(
            Surface::new(width, height, PixelFormat::from(PixelFormatEnum::ARGB8888))
                .expect("Surface creation error"),
        )
        .expect("Surface creation error");

		let main_texture_creator = canvas.texture_creator();

        Self {
            keymap,
            actions: Action::None,
            game_data,
            level_data,
			main_texture_creator,
            surfaces,
            keymod: Mod::NOMOD,
            world: World::new(width as f32, height as f32),
            sdl_context,
            main_canvas: canvas,
            menu_canvas,
            fonts,
            performance_frequency: performance_frequency() as f64,
            bg_color: Color::WHITE,
            last_tick: 0,
            frame_time: 0,
            frame_number: 0,
            fps_frame_count: 0,
            fps_last_tick: 0,
            fps: 0.0,
			window_bounds: FRect { x: 0.0, y: 0.0, w: width as f32, h: height as f32},
			click: None,
            should_quit: false,
            should_wait_after_frame: true,
            should_show_debug: false,
			menu_open: false
        }
    }

    fn load_surfaces(
        texture_definitions: &Vec<TextureDefinition>,
    ) -> HashMap<AssetId, Surface<'a>> {
        let mut surfaces = HashMap::new();

        for i in 0..texture_definitions.len() {
            let texture_definition = &texture_definitions[i];
            let path = Path::new(&texture_definition.path);
            let surface = Surface::load_bmp(path).expect("image load error");

            surfaces.insert(texture_definition.id, surface);
        }

        surfaces
    }

    fn load_fonts(
        font_definitions: &Vec<FontDefinition>,
        ttf_context: &Sdl3TtfContext,
    ) -> HashMap<AssetId, Font<'a>> {
        let mut fonts = HashMap::new();

        for i in 0..font_definitions.len() {
            let font_definition = &font_definitions[i];
            let path = Path::new(&font_definition.path);
            let surface = ttf_context
                .load_font(path, font_definition.size)
                .expect("Font loading error");

            fonts.insert(font_definition.id, surface);
        }

        fonts
    }

    fn load_keymap() -> HashMap<Keycode, Action> {
        let mut keymap = HashMap::new();

        // Load keymap, will later be loaded from a config file

        keymap.insert(Keycode::Escape, Action::Menu);
        keymap.insert(Keycode::F2, Action::Debug);
        keymap.insert(Keycode::F3, Action::FpsLimit);
        keymap.insert(Keycode::W, Action::MoveUp);
        keymap.insert(Keycode::A, Action::MoveLeft);
        keymap.insert(Keycode::S, Action::MoveDown);
        keymap.insert(Keycode::D, Action::MoveRight);
        keymap.insert(Keycode::Space, Action::Jump);
        keymap.insert(Keycode::LShift, Action::Sprint);
        keymap.insert(Keycode::LCtrl, Action::Duck);
        keymap.insert(Keycode::RCtrl, Action::Attack);

        keymap
    }

    fn load_levels(level_definitions: &Vec<LevelDefinition>) -> Vec<LevelData> {
        let mut levels = Vec::new();

        for i in 0..level_definitions.len() {
            let level_definition = &level_definitions[i];
            let path = Path::new(&level_definition.path);
            let file = File::open(path).expect("Could not open level json");
            let reader = BufReader::new(file);
            let level = serde_json::from_reader(reader).expect("Could not parse level json");

            levels.push(level)
        }

        levels
    }

    fn init(&mut self) {
        self.world
            .load_level(&self.level_data.get(0).expect("no level data available"));
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
			Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
				self.click = Some(FPoint { x, y });
			}
			Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {
				self.click = None;
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

		if self.actions.contains(Action::Menu) {
			self.actions &= Action::Menu.not();
			self.menu_open = !self.menu_open;
			self.last_tick = 0;
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

        for (rect, drawable) in drawables {
            if let Some(texture_index) = drawable.texture
                && let Some(surface) = self.surfaces.get(&texture_index)
            {
                if let Ok(mut texture) = self.main_texture_creator.create_texture_from_surface(surface) {
                    if drawable.tint_texture {
                        texture.set_color_mod(drawable.color.r, drawable.color.g, drawable.color.b);
                    }

                    self.main_canvas
                        .copy(&texture, None, rect)
                        .expect("texture error");
                }
            } else {
                self.main_canvas.set_draw_color(drawable.color);
                self.main_canvas.fill_rect(rect).expect("draw error");
            }

			if self.should_show_debug {
				self.main_canvas.set_draw_color(Color::MAGENTA);
				self.main_canvas.draw_rect(FRect { x: rect.x - 1.0, y: rect.y - 1.0, w: rect.w + 2.0, h: rect.h + 2.0 }).expect("draw error");
			}
        }
    }

	fn render_menu(&mut self) {
		self.menu_canvas.set_draw_color(Color::RGBA(0, 0, 0, 127));
		self.menu_canvas.clear();



		self.menu_canvas.present();

		let menu_surface = self.menu_canvas.surface();

		let menu_surface = self
			.main_texture_creator
			.create_texture_from_surface(menu_surface)
			.expect("texture creation panic");

		self.main_canvas.copy(&menu_surface, None, Some(self.window_bounds)).expect("menu render error");
	}

    fn tick(&mut self) {
        let now = sdl3::timer::performance_counter();

        if self.last_tick == 0 {
            self.last_tick = now;
            self.fps_last_tick = now;
            return;
        }

        let delta_t = now - self.last_tick;
        let delta_t_sec = delta_t as f64 / self.performance_frequency;

		self.main_canvas.set_draw_color(self.bg_color);
		self.main_canvas.clear();

		if !self.menu_open {
			self.world.tick(delta_t_sec, self.actions);
		}

		self.render_drawables();

		if self.menu_open {
			self.render_menu();
		}

		if self.should_show_debug {
			self.render_debug_msg(delta_t_sec);
		}

        self.main_canvas.present();

        self.last_tick = now;
        self.keymod = Mod::NOMOD;
        self.frame_number += 1;
        self.fps_frame_count += 1;
        self.frame_time = ((sdl3::timer::performance_counter() - now) as f64
            / self.performance_frequency
            * 1_000.0) as u64;

        let fps_time = (now - self.fps_last_tick) as f64 / self.performance_frequency;
        if fps_time > 1.0 {
            self.fps = self.fps_frame_count as f32 / fps_time as f32;

            self.fps_last_tick = now;
            self.fps_frame_count = 0;
        }

        if !self.should_wait_after_frame {
            return;
        }

        if self.frame_time < MIN_FRAME_TIME {
            let wait_time = (MIN_FRAME_TIME - self.frame_time) * 1_000_000;

            sleep(Duration::new(0, wait_time as u32));
        }
    }

    fn render_msg(&mut self, msg: &String, pos: FPoint) -> FRect {
        let font = self
            .fonts
            .get(&self.game_data.debug_font_id)
            .expect("Invalid debug font id");
        let rendered_text = font.render(msg);
        let surface = rendered_text
            .blended(Color::RGB(255, 255, 255))
            .expect("text render panic");

        let surface_width = surface.width() as f32;
        let surface_height = surface.height() as f32;

        let texture = self
            .main_texture_creator
            .create_texture_from_surface(surface)
            .expect("texture creation panic");

        let text_rect = FRect::new(pos.x + 5.0, pos.y + 2.0, surface_width, surface_height);

        let bg_rect = FRect::new(pos.x, pos.y, surface_width + 10.0, surface_height + 4.0);

        self.main_canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.main_canvas
            .fill_rect(bg_rect)
            .expect("debug message panic");
        self.main_canvas
            .copy(&texture, None, Some(text_rect))
            .expect("debug message panic");

        bg_rect
    }

    fn render_debug_msg(&mut self, delta_t: f64) {
        let sec = delta_t * 1000.0;

        let dt_text = format!("delta_t: {sec:.2}ms");
        let dt_rect = self.render_msg(&dt_text, FPoint::new(0.0, 0.0));

        let fn_text = format!("frame_count: {}", self.frame_number);
        let fn_rect = self.render_msg(&fn_text, FPoint::new(dt_rect.x, dt_rect.bottom()));

        let ft_text = format!("frame_time: {0:.2}ms", self.frame_time);
        let ft_rect = self.render_msg(&ft_text, FPoint::new(fn_rect.x, fn_rect.bottom()));

        let fw_text = format!("fps_limit: {}", self.should_wait_after_frame);
        let fw_rect = self.render_msg(&fw_text, FPoint::new(ft_rect.x, ft_rect.bottom()));

        let fps_text = format!("fps: {0:.2}", self.fps);
        self.render_msg(&fps_text, FPoint::new(fw_rect.x, fw_rect.bottom()));
    }
}
