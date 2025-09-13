use bitmask_enum::bitmask;
use serde::{Deserialize, Serialize};

#[bitmask(usize)]
#[derive(Deserialize, Serialize)]
pub enum Action {
	NONE = 0x0000,
	QUIT = 0x0001,
	DEBUG = 0x0002,
	FPS_LIMIT = 0x0004,
	MOVE_LEFT = 0x0008,
	MOVE_RIGHT = 0x0010,
	MOVE_UP = 0x0020,
	MOVE_DOWN = 0x0040,
	DUCK = 0x0080,
	JUMP = 0x0100,
	SPRINT = 0x0200,
	ATTACK = 0x0400,
	MENU = 0x0800,
}
