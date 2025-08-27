mod geometry;

use crate::geometry::{ellipse};

extern crate sdl3;

use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::time::Duration;
use sdl3::render::{FPoint, WindowCanvas};

trait Ellipse {
	fn draw_ellipse(&mut self, x: f32, y: f32, dx: f32, dy: f32);
}

impl Ellipse for WindowCanvas {
	fn draw_ellipse(&mut self, x: f32, y: f32, dx: f32, dy: f32) {
		let e = ellipse(dx as u32, dy as u32);

		if e.len() == 0 {
			return;
		}

		let slice =  e.iter()
			.map(|&p| FPoint::new(x + p.0 as f32, y + p.2 as f32))
			.collect::<Vec<FPoint>>()
			.into_boxed_slice();

		let (chunks, remainder) = slice.as_chunks::<10>();

		for points in chunks {
			self.draw_points(points.as_ref()).expect("TODO: panic message");
		}

		self.draw_points(remainder).expect("TODO: panic message");
	}
}

fn main() {
	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("rust-sdl3 demo", 800, 600)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas();

	canvas.set_draw_color(Color::RGB(0, 255, 255));
	canvas.clear();
	canvas.present();
	let mut event_pump = sdl_context.event_pump().unwrap();
	let mut i = 0;

	let mut x: f32 = 0.0;
	let mut y: f32 = 0.0;
	let mut increment: f32 = 1.0;

	'running: loop {
		i = (i + 1) % 255;
		canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
		canvas.clear();


		canvas.set_draw_color(Color::RGB(255, 255, 255));

		canvas.draw_ellipse(x, y, 160.0, 80.0);

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'running
				},
				Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
					y -= increment;
				},
				Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
					y += increment;
				},
				Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
					x -= increment;
				},
				Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
					x += increment;
				},
				Event::KeyDown { keycode: Some(Keycode::LShift), .. } => {
					increment = 10.0;
				},
				Event::KeyUp { keycode: Some(Keycode::LShift), .. } => {
					increment = 1.0;
				},
				_ => {}
			}
		}
		// The rest of the game loop goes here...

		canvas.present();

		::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
	}
}
