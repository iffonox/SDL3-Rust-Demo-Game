mod game;
mod serialization;
mod game_object;
mod math;
mod util;
mod gui;
mod actions;
mod mouse;
mod settings;
mod errors;

use std::fs;
use crate::game::Game;
use crate::serialization::game::GameData;
use crate::util::seed_random;
use sdl3::ttf;
use std::fs::File;
use std::io::{BufReader};
use std::path::{Path};
use std::time::SystemTime;
use crate::errors::DataLoadError;
use crate::settings::Settings;

fn main() {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Could not get current time");

    seed_random(now.as_secs() as u32);

    let data_path = Path::new("./assets/assets.json");
    let game_data = load_game_data(data_path).expect("Could not parse assets.json");

	let settings_path = Path::new("./settings.toml");
	let settings = load_game_setting(settings_path).expect("Could not parse assets.json");

    let sdl_context = sdl3::init().unwrap();

	let ttf_context = ttf::init();
    let ttf_context = ttf_context.expect("font context init error");

    let mut game: Game = Game::new(
		settings,
        game_data,
        &sdl_context,
        &ttf_context,
    );

    game.run();
}

fn load_game_data(path: &Path) -> Result<GameData, DataLoadError> {
	let Ok(file) = File::open(path) else {
		return Err(DataLoadError { path: path.to_path_buf() });
	};

	let reader = BufReader::new(file);

	serde_json::from_reader(reader).map_err(|_| DataLoadError { path: path.to_path_buf() })
}

fn load_game_setting(path: &Path) -> Result<Settings, DataLoadError> {
	let data = fs::read_to_string(path).unwrap();

	toml::de::from_str(&data).map_err(|_| DataLoadError { path: path.to_path_buf() })
}
