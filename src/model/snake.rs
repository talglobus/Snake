use serde_json;
use model::{Coord, Direction, Playable};
use std::slice::Iter;

#[derive(Debug, Serialize, Deserialize)] // Removed as `Vec`s cannot be copied by default
pub struct Snake {
	pub pos: Vec<Coord>,
	pub direction: Direction
}

const SNAKE_INITIAL_LENGTH : i16 = 3;	// Note that this includes an extra segment
const SNAKE_ADVANCE_DISTANCE: i16 = 1;

impl Default for Snake {
	fn default() -> Snake {
		Snake {
			pos: vec![
				Coord { x: 50 / 2, y: 50 / 2 },
				Coord { x: 50 / 2 + 1, y: 50 / 2 },
				Coord { x: 50 / 2 + 2, y: 50 / 2 }
			],		// TODO: Change `50` to config box size
			direction: Direction::Left
		}
	}
}

impl Playable for Snake {
	/// Creates a `Snake`, building a model of `SNAKE_INITIAL_LENGTH` correctly-positioned segments
	fn new(head_pos: Coord, direction: Direction) -> Snake {
		let mut pos: Vec<Coord> = vec![head_pos];

		// This isolates the borrow of `direction` to prevent borrow checker errors on the return
		pos = {
			let make_segment = |i: i16| {
				match direction {
					Direction::Left => Coord { x: head_pos.x + i, y: head_pos.y},
					Direction::Right => Coord { x: head_pos.x - i, y: head_pos.y},
					Direction::Up => Coord { x: head_pos.x, y: head_pos.y + i},
					Direction::Down => Coord { x: head_pos.x, y: head_pos.y - i},
				}
			};

			(1..SNAKE_INITIAL_LENGTH).for_each(|i| pos.push(make_segment(i)));

			pos
		};

		Snake {
			pos, direction
		}
	}

	fn advance(&mut self) {
		self.advance_conditional_grow(&|_| false);
	}

	fn advance_conditional_grow(&mut self, condition: &impl Fn(&mut Self) -> bool) {
		let make_segment
		= |pos: &Vec<Coord>, direction: &Direction, i: i16| {
			match direction {
				Direction::Left => Coord { x: pos[0].x - i, y: pos[0].y},
				Direction::Right => Coord { x: pos[0].x + i, y: pos[0].y},
				Direction::Up => Coord { x: pos[0].x, y: pos[0].y - i},
				Direction::Down => Coord { x: pos[0].x, y: pos[0].y + i},
			}
		};

		// Note that the checking of condition is once per step, which should also give the...
		//... appropriate behavior with a `SNAKE_ADVANCE_DISTANCE` greater than 1,...
		//... though be sure that the "appropriate" behavior is the desired behavior
		for i in 1..SNAKE_ADVANCE_DISTANCE + 1 {
			let new_segment = make_segment(&self.pos, &self.direction,i);
			self.pos.insert(0, new_segment);
			if !condition(self) {
				self.pos.pop();
			}
		}
	}

	fn rotate(&mut self, direction: Direction) {
		self.direction = direction;
	}

	fn body_iter_with_head(& self) -> Iter<Coord> {
		self.pos[..self.pos.len()-1].iter()
	}

	fn body_iter_without_head(& self) -> Iter<Coord> {
		self.pos[1..self.pos.len()-1].iter()
	}

	fn score(&self) -> i16 {
		self.pos.len() as i16
	}
}