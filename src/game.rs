use crate::game_object::world::World;
use crate::gui::{Align, TextFormat, UiElement};
use crate::math::bounds::Bounds;
use crate::serialization::font::FontDefinition;
use crate::serialization::game::{AssetDefinition, GameData, TextureDefinition};
use crate::serialization::level::LevelData;
use crate::serialization::{AssetId};
use sdl3::Sdl;
use sdl3::event::Event;
use sdl3::keyboard::{Keycode};
use sdl3::mouse::MouseButton;
use sdl3::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl3::render::{FPoint, FRect, SurfaceCanvas, TextureCreator, WindowCanvas};
use sdl3::surface::{Surface, SurfaceContext};
use sdl3::timer::performance_frequency;
use sdl3::ttf::{Font, Sdl3TtfContext};
use sdl3::video::WindowContext;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use crate::actions::Action;
use crate::mouse::{Mouse, MouseButtonState};
use crate::settings::{Settings};

type MilliSeconds = u64;

static WINDOW_TITLE: &str = "rust-sdl3 demo";

#[derive(Debug, Copy, Clone, Default)]
struct FrameData {
	pub last_tick: u64,
	pub frame_time: u64,
	pub frame_number: u64,
	pub fps_frame_count: u64,
	pub fps_last_tick: u64,
	pub fps: f32,
}

#[derive(Debug, Copy, Clone, Default)]
struct SystemState {
	should_quit: bool,
	should_show_debug: bool,
	menu_open: bool,
}

pub struct Game<'a> {
    keymap: HashMap<Keycode, Action>,
    actions: Action,
    game_data: GameData,
    sdl_context: &'a Sdl,
    main_texture_creator: TextureCreator<WindowContext>,
    menu_texture_creator: TextureCreator<SurfaceContext<'a>>,
    main_canvas: WindowCanvas,
    menu_canvas: SurfaceCanvas<'a>,
    fonts: HashMap<AssetId, Font<'a>>,
    surfaces: HashMap<AssetId, Surface<'a>>,
    world: World,
    bg_color: Color,
    level_data: Vec<LevelData>,
    gui_data: Vec<UiElement>,
	performance_frequency: f64,
	settings: Settings,
	frame_data: FrameData,
    window_bounds: FRect,
	mouse: Mouse,
	system_state: SystemState,
	min_frame_time: MilliSeconds
}

impl<'a> Game<'a> {
    pub fn new(
        settings: Settings,
        game_data: GameData,
        sdl_context: &'a Sdl,
        ttf_context: &'a Sdl3TtfContext,
    ) -> Self {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(WINDOW_TITLE, settings.width as u32, settings.height as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas();

        let fonts = Self::load_fonts(&game_data.fonts, ttf_context);

        let surfaces = Self::load_surfaces(&game_data.textures);

        let keymap = Self::load_keymap();

        let level_data = Self::load_definitions(&game_data.levels);
        let gui_data = Self::load_definitions(&game_data.guis);

        let menu_canvas = SurfaceCanvas::from_surface(
            Surface::new(settings.width as u32, settings.height as u32, PixelFormat::from(PixelFormatEnum::ARGB8888))
                .expect("Surface creation error"),
        )
        .expect("Surface creation error");

        let main_texture_creator = canvas.texture_creator();

        let menu_texture_creator = menu_canvas.texture_creator();

		let min_frame_time: MilliSeconds =  1_000u64 / settings.frame_limit as u64;

        Self {
            keymap,
            actions: Action::NONE,
            game_data,
            level_data,
            gui_data,
            main_texture_creator,
            menu_texture_creator,
            surfaces,
            world: World::new(settings.width as f32, settings.height as f32),
            sdl_context,
            main_canvas: canvas,
            menu_canvas,
            fonts,
            performance_frequency: performance_frequency() as f64,
            bg_color: Color::WHITE,
			frame_data: FrameData::default(),
			settings,
            window_bounds: FRect {
                x: 0.0,
                y: 0.0,
                w: settings.width as f32,
                h: settings.height as f32,
            },
			mouse: Mouse {
				buttons:  MouseButtonState::NONE,
				pos: FPoint { x: 0.0, y: 0.0 },
			},
			system_state: SystemState {
				should_quit: false,
				should_show_debug: false,
				menu_open: false,
			},
			min_frame_time
        }
    }

    fn load_surfaces(texture_definitions: &[TextureDefinition]) -> HashMap<AssetId, Surface<'a>> {
        let mut surfaces = HashMap::with_capacity(texture_definitions.len());

        for i in 0..texture_definitions.len() {
            let texture_definition = &texture_definitions[i];
            let path = Path::new(&texture_definition.path);
            let surface = Surface::load_bmp(path).expect("image load error");

            surfaces.insert(texture_definition.id, surface);
        }

        surfaces
    }

    fn load_fonts(
        font_definitions: &[FontDefinition],
        ttf_context: &Sdl3TtfContext,
    ) -> HashMap<AssetId, Font<'a>> {
        let mut fonts = HashMap::with_capacity(font_definitions.len());

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

        keymap.insert(Keycode::Escape, Action::MENU);
        keymap.insert(Keycode::F2, Action::DEBUG);
        keymap.insert(Keycode::F3, Action::FPS_LIMIT);
        keymap.insert(Keycode::W, Action::MOVE_UP);
        keymap.insert(Keycode::A, Action::MOVE_LEFT);
        keymap.insert(Keycode::S, Action::MOVE_DOWN);
        keymap.insert(Keycode::D, Action::MOVE_RIGHT);
        keymap.insert(Keycode::Space, Action::JUMP);
        keymap.insert(Keycode::LShift, Action::SPRINT);
        keymap.insert(Keycode::LCtrl, Action::DUCK);
        keymap.insert(Keycode::RCtrl, Action::ATTACK);

        keymap
    }

    fn load_definitions<T>(definitions: &[AssetDefinition]) -> Vec<T>
    where
        T: DeserializeOwned,
    {
        let mut results = Vec::with_capacity(definitions.len());

        for i in 0..definitions.len() {
            let definition = &definitions[i];
            let path = Path::new(&definition.path);

            let mut path_error = String::from("Could not open asset json: ");
            path_error.push_str(&definition.path);

            let file = File::open(path).expect(&path_error);
            let reader = BufReader::new(file);

            let mut parse_error = String::from("Could not parse asset json: ");
            parse_error.push_str(&definition.path);

            let asset = serde_json::from_reader(reader).expect(&parse_error);

            results.push(asset)
        }

        results
    }

    fn init(&mut self) {
        let level = self.level_data.get(0).expect("no level data available");
        let mut title = WINDOW_TITLE.to_owned();
        title.push_str(" - ");
        title.push_str(&level.name);

        self.main_canvas
            .window_mut()
            .set_title(&title)
            .expect("setting window title failed");

        self.world.load_level(level);
    }

    pub fn run(&mut self) {
        self.init();

        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'running: loop {
            if self.system_state.should_quit {
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
			Event::MouseMotion { x, y, .. } => {
				self.mouse.pos = FPoint { x, y }
			}
            Event::MouseButtonDown {
                mouse_btn,
                ..
            } => {
                self.mouse.buttons |= match mouse_btn {
					MouseButton::Left => MouseButtonState::LEFT_BUTTON,
					MouseButton::Middle => MouseButtonState::MIDDLE_BUTTON,
					MouseButton::Right => MouseButtonState::RIGHT_BUTTON,
					_ => MouseButtonState::NONE,
				}
            }
            Event::MouseButtonUp {
                mouse_btn,
                ..
            } => {
				self.mouse.buttons &= match mouse_btn {
					MouseButton::Left => MouseButtonState::LEFT_BUTTON,
					MouseButton::Middle => MouseButtonState::MIDDLE_BUTTON,
					MouseButton::Right => MouseButtonState::RIGHT_BUTTON,
					_ => MouseButtonState::NONE,
				}.not()
            }
            Event::Quit { .. } => {
                self.actions |= Action::QUIT;
            }
            _ => {}
        }
    }

    fn handle_system_events(&mut self) {
        if self.actions.contains(Action::QUIT) {
            self.system_state.should_quit = true;
            return;
        }

        if self.actions.contains(Action::MENU) {
            self.actions &= Action::MENU.not();
            self.system_state.menu_open = !self.system_state.menu_open;
            self.frame_data.last_tick = 0;
        }

        if self.actions.contains(Action::FPS_LIMIT) {
            self.actions &= Action::FPS_LIMIT.not();
            self.settings.frame_limit_active = !self.settings.frame_limit_active
        }

        if self.actions.contains(Action::DEBUG) {
            self.actions &= Action::DEBUG.not();
            self.system_state.should_show_debug = !self.system_state.should_show_debug
        }

        if self.system_state.menu_open {
            self.handle_ui_events();
        }
    }

    fn handle_ui_events(&mut self) {
	}

    fn render_drawables(&mut self) {
        let drawables = self.world.get_drawables();

        for (rect, drawable) in drawables {
            if let Some(texture_index) = drawable.texture_id
                && let Some(surface) = self.surfaces.get(&texture_index)
            {
                if let Ok(mut texture) = self
                    .main_texture_creator
                    .create_texture_from_surface(surface)
                {
                    if let Some(color) = drawable.color
                        && drawable.tint_texture
                    {
                        texture.set_color_mod(color.r, color.g, color.b);
                    }

                    self.main_canvas
                        .copy(&texture, None, rect)
                        .expect("texture error");
                }
            } else if let Some(color) = drawable.color {
                self.main_canvas.set_draw_color(color);
                self.main_canvas.fill_rect(rect).expect("draw error");
            }

            if self.system_state.should_show_debug {
                self.main_canvas.set_draw_color(Color::MAGENTA);
                self.main_canvas
                    .draw_rect(FRect {
                        x: rect.x - 1.0,
                        y: rect.y - 1.0,
                        w: rect.w + 2.0,
                        h: rect.h + 2.0,
                    })
                    .expect("draw error");
            }
        }
    }

    fn render_ui_element(&mut self, element: &UiElement, parent_bounds: FRect) {
		let mut bounds;

        match element {
            UiElement::Box(e) => {
				bounds = e.bounds;

				// Offset the position by the parents positions; the elements bounds are always relative to their parent
				bounds.x += parent_bounds.x;
				bounds.y += parent_bounds.y;

                self.menu_canvas.set_draw_color(e.bg);
                self.menu_canvas
                    .fill_rect(bounds)
                    .expect("Failed to fill rect");
            }
            UiElement::Label(e) => {
				bounds = e.bounds;

				// Offset the position by the parents positions; the elements bounds are always relative to their parent
				bounds.x += parent_bounds.x;
				bounds.y += parent_bounds.y;

                self.menu_canvas.set_draw_color(e.bg);
                self.menu_canvas
                    .fill_rect(bounds)
                    .expect("Failed to fill rect");

                let surface = self.build_text_surface(&e.text, e.format);
                let surface_width = surface.width() as f32;
                let surface_height = surface.height() as f32;

                let texture = self
                    .menu_texture_creator
                    .create_texture_from_surface(surface)
                    .expect("texture creation panic");

                let x = match e.format.justify {
                    Align::Start => 0.0,
                    Align::Center => (bounds.w - surface_width) / 2.0,
                    Align::End => bounds.w - surface_width,
                } + bounds.x;

                let y = match e.format.align {
                    Align::Start => 0.0,
                    Align::Center => (bounds.h - surface_height) / 2.0,
                    Align::End => bounds.h - surface_height,
                } + bounds.y;

                let text_rect = FRect {
                    x,
                    y,
                    w: surface_width,
                    h: surface_height,
                };

                self.menu_canvas
                    .copy(&texture, None, Some(text_rect))
                    .expect("debug message panic");
            }
        }

        let children = element.get_children();

        for i in 0..children.len() {
            let child = &children[i];

            self.render_ui_element(child, bounds);
        }
    }

    fn render_menu(&mut self) {
        self.menu_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        self.menu_canvas.clear();

        if let Some(main_menu) = self.gui_data.get(0) {
            let e = main_menu.clone();

            self.render_ui_element(&e, self.window_bounds);
        }

        self.menu_canvas.present();

        let menu_surface = self.menu_canvas.surface();

        let menu_surface = self
            .main_texture_creator
            .create_texture_from_surface(menu_surface)
            .expect("texture creation panic");

        self.main_canvas
            .copy(&menu_surface, None, Some(self.window_bounds))
            .expect("menu render error");
    }

    fn tick(&mut self) {
        let now = sdl3::timer::performance_counter();

        if self.frame_data.last_tick == 0 {
            self.frame_data.last_tick = now;
            self.frame_data.fps_last_tick = now;
            return;
        }

        let delta_t = now - self.frame_data.last_tick;
        let delta_t_sec = delta_t as f64 / self.performance_frequency;

        self.main_canvas.set_draw_color(self.bg_color);
        self.main_canvas.clear();

        if !self.system_state.menu_open {
            self.world.tick(delta_t_sec, self.actions);
        }

        self.render_drawables();

        if self.system_state.menu_open {
            self.render_menu();
        }

        if self.system_state.should_show_debug {
            self.render_debug_msg(delta_t_sec);
        }

        self.main_canvas.present();

        self.frame_data.last_tick = now;
        self.frame_data.frame_number += 1;
        self.frame_data.fps_frame_count += 1;
        self.frame_data.frame_time = ((sdl3::timer::performance_counter() - now) as f64
            / self.performance_frequency
            * 1_000.0) as u64;

        let fps_time = (now - self.frame_data.fps_last_tick) as f64 / self.performance_frequency;
        if fps_time > 1.0 {
            self.frame_data.fps = self.frame_data.fps_frame_count as f32 / fps_time as f32;

            self.frame_data.fps_last_tick = now;
            self.frame_data.fps_frame_count = 0;
        }

        if !self.settings.frame_limit_active {
            return;
        }

        if self.frame_data.frame_time < self.min_frame_time {
            let wait_time = (self.min_frame_time - self.frame_data.frame_time) * 1_000_000;

            sleep(Duration::new(0, wait_time as u32));
        }
    }

    fn build_text_surface(&self, text: &String, format: TextFormat) -> Surface<'_> {
        let font = self
            .fonts
            .get(&format.font_id)
            .expect("Invalid debug font id");
        let rendered_text = font.render(text);
        let surface = rendered_text
            .blended(format.color)
            .expect("text render panic");

        surface
    }

    fn render_msg(&mut self, msg: &String, pos: FPoint) -> FRect {
        let surface = self.build_text_surface(
            msg,
            TextFormat {
                font_id: self.game_data.debug_font_id,
                color: Color::WHITE,
                justify: Align::Start,
                align: Align::Start,
            },
        );

        let surface_width = surface.width() as f32;
        let surface_height = surface.height() as f32;

        let texture = self
            .main_texture_creator
            .create_texture_from_surface(surface)
            .expect("texture creation panic");

        let text_rect = FRect::new(pos.x + 5.0, pos.y + 2.0, surface_width, surface_height);

        let bg_rect = FRect::new(pos.x, pos.y, surface_width + 10.0, surface_height + 4.0);

        self.main_canvas.set_draw_color(Color::BLACK);
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

        let fn_text = format!("frame_count: {}", self.frame_data.frame_number);
        let fn_rect = self.render_msg(&fn_text, FPoint::new(dt_rect.x, dt_rect.bottom()));

        let ft_text = format!("frame_time: {0:.2}ms", self.frame_data.frame_time);
        let ft_rect = self.render_msg(&ft_text, FPoint::new(fn_rect.x, fn_rect.bottom()));

        let fw_text = format!("fps_limit: {}", self.settings.frame_limit_active);
        let fw_rect = self.render_msg(&fw_text, FPoint::new(ft_rect.x, ft_rect.bottom()));

        let fps_text = format!("fps: {0:.2}", self.frame_data.fps);
        self.render_msg(&fps_text, FPoint::new(fw_rect.x, fw_rect.bottom()));
    }
}
