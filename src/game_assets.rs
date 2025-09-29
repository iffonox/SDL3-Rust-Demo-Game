use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use sdl3::keyboard::Keycode;
use sdl3::surface::Surface;
use sdl3::ttf::{Font, Sdl3TtfContext};
use serde::de::DeserializeOwned;
use crate::actions::Action;
use crate::errors::DataLoadError;
use crate::gui::UiElement;
use crate::serialization::AssetId;
use crate::serialization::font::FontDefinition;
use crate::serialization::game::{AssetDefinition, GameData, TextureDefinition};
use crate::serialization::level::LevelData;

pub struct GameAssets<'a> {
	pub game_data: GameData,
	pub fonts: HashMap<AssetId, Font<'a>>,
	pub surfaces: HashMap<AssetId, Surface<'a>>,
	pub level_data: Vec<LevelData>,
	pub gui_data: Vec<UiElement>,
	pub keymap: HashMap<Keycode, Action>,
}

impl<'a> GameAssets<'_> {
	pub fn new (asset_file: &str, ttf_context: &'a Sdl3TtfContext) -> Self {
		let data_path = Path::new(asset_file);
		let game_data = load_game_data(data_path).expect("Could not parse assets.json");

		let fonts = load_fonts(&game_data.fonts, ttf_context);

		let surfaces = load_surfaces(&game_data.textures);

		let level_data = load_definitions(&game_data.levels);

		let gui_data = load_definitions(&game_data.guis);

		let keymap = load_keymap();

		Self {
			game_data,
			fonts,
			surfaces,
			level_data,
			gui_data,
			keymap,
		}
	}
}

fn load_surfaces<'a>(texture_definitions: &[TextureDefinition]) -> HashMap<AssetId, Surface<'a>> {
	let mut surfaces = HashMap::with_capacity(texture_definitions.len());

	for i in 0..texture_definitions.len() {
		let texture_definition = &texture_definitions[i];
		let path = Path::new(&texture_definition.path);
		let surface = Surface::load_bmp(path).expect("image load error");

		surfaces.insert(texture_definition.id, surface);
	}

	surfaces
}

fn load_fonts<'a>(
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

fn load_game_data(path: &Path) -> Result<GameData, DataLoadError> {
	let Ok(file) = File::open(path) else {
		return Err(DataLoadError { path: path.to_path_buf() });
	};

	let reader = BufReader::new(file);

	serde_json::from_reader(reader).map_err(|_| DataLoadError { path: path.to_path_buf() })
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
