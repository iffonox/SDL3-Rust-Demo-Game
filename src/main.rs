mod game_object;
mod game;
mod math;

use sdl3::ttf;
use crate::game::Game;

fn main() {
	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let ttf_context = ttf::init();

	let window = video_subsystem
		.window("rust-sdl3 demo", 800, 600)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas();

	let texture_creator = canvas.texture_creator();

	let ttf_context = ttf_context.expect("font context init error");

	let mut game: Game = Game::new(800, 600, &sdl_context, &mut canvas, &texture_creator, &ttf_context);

	game.run();
}
