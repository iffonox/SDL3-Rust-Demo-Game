use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialOrd, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Action {
	Quit,
	Debug,
	FpsLimit,
	MoveLeft,
	MoveRight,
	MoveUp,
	MoveDown,
	Duck,
	Jump,
	Sprint,
	Attack,
	Menu,
}
