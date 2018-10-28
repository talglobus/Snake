#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coord {
	pub x: i16,
	pub y: i16
}

#[derive(Debug)]
pub enum Direction {
	Left,
	Right,
	Up,
	Down,
//	UpRight,
//	UpLeft,
//	DownRight,
//	DownLeft,
}