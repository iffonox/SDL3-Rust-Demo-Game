use std::collections::HashSet;
use crate::game_object::world::World;
use crate::gui::{Align, ElementType, TextFormat, UiElement};
use crate::math::bounds::Bounds;
use sdl3::Sdl;
use sdl3::event::Event;
use sdl3::mouse::MouseButton;
use sdl3::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl3::render::{FPoint, FRect, SurfaceCanvas, TextureCreator, WindowCanvas};
use sdl3::surface::{Surface, SurfaceContext};
use sdl3::timer::performance_frequency;
use sdl3::ttf::{Sdl3TtfContext};
use sdl3::video::WindowContext;
use std::thread::sleep;
use std::time::Duration;
use crate::actions::Action;
use crate::game_assets::GameAssets;
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
    actions: HashSet<Action>,
    sdl_context: &'a Sdl,
    main_texture_creator: TextureCreator<WindowContext>,
    menu_texture_creator: TextureCreator<SurfaceContext<'a>>,
    main_canvas: WindowCanvas,
    menu_canvas: SurfaceCanvas<'a>,
    world: World,
	performance_frequency: f64,
	settings: Settings,
	assets: GameAssets<'a>,
	frame_data: FrameData,
    window_bounds: FRect,
	mouse: Mouse,
	system_state: SystemState,
	min_frame_time: MilliSeconds
}

impl<'a> Game<'a> {
    pub fn new(
        settings: Settings,
        sdl_context: &'a Sdl,
        ttf_context: &'a Sdl3TtfContext,
    ) -> Self {
		let assets = GameAssets::new(&settings.asset_file, ttf_context);

        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(WINDOW_TITLE, settings.width as u32, settings.height as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas();

        let menu_canvas = SurfaceCanvas::from_surface(
            Surface::new(settings.width as u32, settings.height as u32, PixelFormat::from(PixelFormatEnum::ARGB8888))
                .expect("Surface creation error"),
        )
        .expect("Surface creation error");

        let main_texture_creator = canvas.texture_creator();

        let menu_texture_creator = menu_canvas.texture_creator();

		let min_frame_time: MilliSeconds =  1_000u64 / settings.frame_limit as u64;

        Self {
            actions: HashSet::new(),
            main_texture_creator,
            menu_texture_creator,
            world: World::new(settings.width as f32, settings.height as f32),
            sdl_context,
            main_canvas: canvas,
            menu_canvas,
            performance_frequency: performance_frequency() as f64,
			frame_data: FrameData::default(),
            window_bounds: FRect {
                x: 0.0,
                y: 0.0,
                w: settings.width as f32,
                h: settings.height as f32,
            },
			settings,
			assets,
			mouse: Mouse::default(),
			system_state: SystemState {
				should_quit: false,
				should_show_debug: false,
				menu_open: false,
			},
			min_frame_time
        }
    }

    fn init(&mut self) {
        let level = self.assets.level_data.get(0).expect("no level data available");
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
		self.mouse.buttons = MouseButtonState::NONE;

        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                repeat: false,
                ..
            } => {
                if let Some(action) = self.assets.keymap.get(&keycode) {
                    self.actions.insert(*action);
                }
            }
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => {
                if let Some(action) = self.assets.keymap.get(&keycode) {
                    self.actions.remove(action);
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
				self.actions.insert(Action::Quit);
            }
            _ => {}
        }
    }

    fn handle_system_events(&mut self) {
        if self.actions.contains(&Action::Quit) {
            self.system_state.should_quit = true;
            return;
        }

        if self.actions.contains(&Action::Menu) {
			self.actions.remove(&Action::Menu);
            self.system_state.menu_open = !self.system_state.menu_open;
            self.frame_data.last_tick = 0;
        }

        if self.actions.contains(&Action::FpsLimit) {
			self.actions.remove(&Action::FpsLimit);
            self.settings.frame_limit_active = !self.settings.frame_limit_active
        }

        if self.actions.contains(&Action::Debug) {
			self.actions.remove(&Action::Debug);
            self.system_state.should_show_debug = !self.system_state.should_show_debug
        }

        if self.system_state.menu_open {
            self.handle_ui_events();
        }
    }

    fn handle_ui_events(&mut self) {
		if self.assets.gui_data.len() == 0 {
			return;
		}

		let main_menu = &mut self.assets.gui_data[0];

		let Some(action) = main_menu.handle_event(self.mouse) else {
			return;
		};

		self.actions.insert(action);
	}

    fn render_drawables(&mut self) {
        let drawables = self.world.get_drawables();

        for (rect, drawable) in drawables {
            if let Some(texture_index) = drawable.texture_id
                && let Some(surface) = self.assets.surfaces.get(&texture_index)
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

		bounds = element.bounds;

		// Offset the position by the parents positions; the elements bounds are always relative to their parent
		bounds.x += parent_bounds.x;
		bounds.y += parent_bounds.y;

		self.menu_canvas.set_draw_color(element.bg);
		self.menu_canvas
			.fill_rect(bounds)
			.expect("Failed to fill rect");

        match &element.element_type {
			ElementType::Label { text, format } => {
                let surface = self.build_text_surface(text, format);
                let surface_width = surface.width() as f32;
                let surface_height = surface.height() as f32;

                let texture = self
                    .menu_texture_creator
                    .create_texture_from_surface(surface)
                    .expect("texture creation panic");

                let x = match format.justify {
                    Align::Start => 0.0,
                    Align::Center => (bounds.w - surface_width) / 2.0,
                    Align::End => bounds.w - surface_width,
                } + bounds.x;

                let y = match format.align {
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
			_ => {}
        }

        let children = &element.children;

        for i in 0..children.len() {
            let child = &children[i];

            self.render_ui_element(child, bounds);
        }
    }

    fn render_menu(&mut self) {
        self.menu_canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        self.menu_canvas.clear();

        if let Some(main_menu) = self.assets.gui_data.get(0) {
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

        self.main_canvas.set_draw_color(Color::WHITE);
        self.main_canvas.clear();

        if !self.system_state.menu_open {
            self.world.tick(delta_t_sec, &self.actions);
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

    fn build_text_surface(&self, text: &String, format: &TextFormat) -> Surface<'_> {
        let font = self
			.assets
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
		let format = TextFormat {
			font_id: self.assets.game_data.debug_font_id,
			color: Color::WHITE,
			justify: Align::Start,
			align: Align::Start,
		};

        let surface = self.build_text_surface(msg, &format);

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
