use model::{Coord, Direction};
use std::slice::Iter;

pub trait Playable {
	// Static method signature; `Self` refers to the implementor type.
	fn new(head_pos: Coord, direction: Direction) -> Self;

	fn advance(&mut self);

	fn advance_conditional_grow(&mut self, condition: &impl Fn (&mut Self) -> bool);

	fn rotate(&mut self, direction: Direction);

	fn grow(&mut self);

	fn body_iter_with_head(&self) -> Iter<Coord>;

	fn body_iter_without_head(&self) -> Iter<Coord>;

	fn score(&self) -> i16;
}