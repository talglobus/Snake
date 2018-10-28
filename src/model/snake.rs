use model::{Coord, Direction, Playable};

#[derive(Debug)] // Removed as `Vec`s cannot be copied by default
pub struct Snake {
	pub pos: Vec<Coord>,
	pub direction: Direction
}

impl IntoIterator for Snake {
	type Item = Coord;
	type IntoIter = ::std::vec::IntoIter<Coord>;

	fn into_iter(self) -> Self::IntoIter {
		self.pos.into_iter()
	}
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

		let make_segment
		= |pos: &Vec<Coord>, direction: &Direction, i: i16| {
			match direction {
				Direction::Left => Coord { x: pos[0].x - i, y: pos[0].y},
				Direction::Right => Coord { x: pos[0].x + i, y: pos[0].y},
				Direction::Up => Coord { x: pos[0].x, y: pos[0].y - i},
				Direction::Down => Coord { x: pos[0].x, y: pos[0].y + i},
			}
		};

		for i in 1..SNAKE_ADVANCE_DISTANCE + 1 {
			let new_segment = make_segment(&self.pos, &self.direction,i);
			self.pos.insert(0, new_segment);
			self.pos.pop();
		}
	}

	fn rotate(&mut self, direction: Direction) {
		self.direction = direction;
	}

	fn grow(&mut self) {	// TODO: Change this from `direction`-based to tail-direction-based
		let make_segment
		= |last_segment: &Option<&Coord>, direction: &Direction, i: i16| {
			match last_segment {
				Some (last_seg) => match direction {
					Direction::Left => Coord { x: last_seg.x + i, y: last_seg.y},
					Direction::Right => Coord { x: last_seg.x - i, y: last_seg.y},
					Direction::Up => Coord { x: last_seg.x, y: last_seg.y + i},
					Direction::Down => Coord { x: last_seg.x, y: last_seg.y - i},
				},
				None => Coord { x: 0, y: 0 }
			}
		};

		let new_segment = make_segment(&self.pos.last(), &self.direction, 1);
		self.pos.push(new_segment);
	}

	fn score(&self) -> i16 {
		self.pos.len() as i16
	}
}