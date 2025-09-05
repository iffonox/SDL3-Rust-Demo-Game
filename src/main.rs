mod game;
mod serialization;
mod game_object;
mod math;
mod util;

use crate::game::Game;
use crate::serialization::game::GameData;
use crate::util::seed_random;
use sdl3::ttf;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::SystemTime;

fn main() {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Could not get current time");

    seed_random(now.as_secs() as u32);

    let path = Path::new("./assets/assets.json");
    let file = File::open(path).expect("Could not open assets.json");
    let reader = BufReader::new(file);
    let game_data: GameData = serde_json::from_reader(reader).expect("Could not parse assets.json");

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

    let mut game: Game = Game::new(
        800,
        600,
        &game_data,
        &sdl_context,
        &mut canvas,
        &texture_creator,
        &ttf_context,
    );

    game.run();
}
