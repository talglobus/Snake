use model::{Coord, Direction};

pub trait Playable {
	// Static method signature; `Self` refers to the implementor type.
	fn new(head_pos: Coord, direction: Direction) -> Self;

	fn advance(&mut self);

	fn rotate(&mut self, direction: Direction);

	fn grow(&mut self);

	fn score(&self) -> i16;
}