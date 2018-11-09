use model::{Coord, Direction, Playable};
use std::slice::Iter;

#[derive(Debug)] // Removed as `Vec`s cannot be copied by default
pub struct Snake {
	pub pos: Vec<Coord>,
	pub direction: Direction
}

const SNAKE_INITIAL_LENGTH : i16 = 3;	// Note that this includes an extra segment
const SNAKE_ADVANCE_DISTANCE: i16 = 1;

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

	fn grow(&mut self) {
		let make_segment: fn(&Vec<Coord>, &Direction, i16) -> Coord =
			if self.pos.len() < 3 { // If the length isn't long enough for tail-based, use old method
				|body: &Vec<Coord>, direction: &Direction, i: i16| {
					match body.last() {
						Some (last_seg) => match direction {
							Direction::Left => Coord { x: last_seg.x + i, y: last_seg.y },
							Direction::Right => Coord { x: last_seg.x - i, y: last_seg.y },
							Direction::Up => Coord { x: last_seg.x, y: last_seg.y + i },
							Direction::Down => Coord { x: last_seg.x, y: last_seg.y - i },
						},
						None => Coord { x: 0, y: 0 }
					}
				}
			} else {
				|body: &Vec<Coord>, direction: &Direction, i: i16| {
					let tail = body
						.get(body.len()-3..)		// TODO: Check this for off-by-one errors
						.unwrap();

					let last = tail[1];
					let sec_last = tail[0];

					if tail[0].x == last.x && sec_last.y > last.y {			// Add segment below
						Coord { x: last.x, y: last.y + i }
					} else if sec_last.x == last.x {						// Add segment above
						Coord { x: last.x, y: last.y - i }
					} else if sec_last.y == last.y && sec_last.x > last.x {	// Add segment to left
						Coord { x: last.x - i, y: last.y }
					} else {												// Add segment to right
						Coord { x: last.x + i, y: last.y }
					}
				}
			};

		let new_segment = make_segment(&self.pos, &self.direction, 1);
		self.pos.push(new_segment);

		// TODO: Replace the above with "advance-and-grow", moving onto the food with a new segment
	}

//	fn old_grow(&mut self) {	// TODO: Change this from `direction`-based to tail-direction-based
//		let make_segment: fn(&Option<&Coord>, &Direction, i16) -> Coord
//		= |last_segment: &Option<&Coord>, direction: &Direction, i: i16| {
//			match last_segment {
//				Some (last_seg) => match direction {
//					Direction::Left => Coord { x: last_seg.x + i, y: last_seg.y},
//					Direction::Right => Coord { x: last_seg.x - i, y: last_seg.y},
//					Direction::Up => Coord { x: last_seg.x, y: last_seg.y + i},
//					Direction::Down => Coord { x: last_seg.x, y: last_seg.y - i},
//				},
//				None => Coord { x: 0, y: 0 }
//			}
//		};
//
//		let new_segment = make_segment(&self.pos.last(), &self.direction, 1);
//		self.pos.push(new_segment);
//	}

	fn body_iter_with_head(& self) -> Iter<Coord> {
		self.pos.iter()
	}

	fn body_iter_without_head(& self) -> Iter<Coord> {
		self.pos[1..].iter()
	}

	fn score(&self) -> i16 {
		self.pos.len() as i16
	}
}