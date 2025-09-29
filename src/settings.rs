use crate::actions::{Action};
use sdl3::keyboard::Keycode;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::marker::PhantomData;
use serde::de::{MapAccess, Visitor};

pub type Pixels = u16;
pub type Fps = u16;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub width: Pixels,
    pub height: Pixels,
    pub frame_limit_active: bool,
    pub frame_limit: Fps,
    pub asset_file: String,
    #[serde(
        default,
        deserialize_with = "_de_key_code_map",
        serialize_with = "_ser_key_code_map"
    )]
    pub keymap: HashMap<Keycode, Action>,
}

struct KeyMapVisitor {
	marker: PhantomData<fn() -> HashMap<Keycode, Action>>
}

impl KeyMapVisitor {
	fn new() -> Self {
		Self {
			marker: PhantomData
		}
	}
}

impl<'de> Visitor<'de> for KeyMapVisitor {
	type Value = HashMap<Keycode, Action>;

	fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
		formatter.write_str("didn't expect that")
	}

	fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
	where
		A: MapAccess<'de>,
	{
		let mut map = HashMap::with_capacity(access.size_hint().unwrap_or(0));

		while let Some((key, value)) = access.next_entry()? {
			let Some(keycode) = Keycode::from_name(key) else {
				continue;
			};

			map.insert(keycode, value);
		}

		Ok(map)
	}
}

fn _de_key_code_map<'de, D>(deserializer: D) -> Result<HashMap<Keycode, Action>, D::Error>
where
    D: Deserializer<'de>,
{
	let visitor = KeyMapVisitor::new();

	deserializer.deserialize_map(visitor)
}

fn _ser_key_code_map<S>(map: &HashMap<Keycode, Action>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut ser = s
        .serialize_map(Some(map.len()))
        .expect("error while serializing key_code_map");

    map.iter().for_each(|(code, action)| {
		let key = code.to_string();

        ser.serialize_entry(&key, action)
            .expect("error while serializing key")
    });

    ser.end()
}
