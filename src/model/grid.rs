use serde;
use serde_json;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Coord {
	pub x: i16,
	pub y: i16
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Direction {
	Left,
	Right,
	Up,
	Down,
}

#[derive(PartialEq, Debug)]
pub enum DirectionKey {		// TODO: Possibly unify this with `Direction` in `model.rs`
	Up,
	Down,
	Left,
	Right,
	None,
}