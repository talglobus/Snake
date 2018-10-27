const SNAKE_INITIAL_LENGTH : i16 = 3;
const SNAKE_ADVANCE_DISTANCE: i16 = 1;

#[derive(Clone, Copy, Debug)]
pub struct Coord {
	pub x: i16,
	pub y: i16
}

//#[derive(Clone, Copy, Deref, Debug)] // Removed as `Vec`s cannot be copied by default
pub struct Snake {
	pub pos: Vec<Coord>,
	pub direction: Direction
}

//#[derive(Clone, Copy)]
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

pub trait Movable {
	// Static method signature; `Self` refers to the implementor type.
	fn new(head_pos: Coord, direction: Direction) -> Self;

	// Instance method signatures; these will return a string.
//	fn name(&self) -> &'static str;
//	fn noise(&self) -> &'static str;
	fn advance(&mut self);

	fn rotate(&mut self, direction: Direction);

	fn grow(&mut self);

	fn score(&self) -> i64;

	// Traits can provide default method definitions.
//	fn talk(&self) {
//		println!("{} says {}", self.name(), self.noise());
//	}
}

impl Movable for Snake {
	/// Creates a `Snake`, building a body of `SNAKE_INITIAL_LENGTH` correctly-positioned segments
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

			for i in 1..SNAKE_INITIAL_LENGTH {
				pos.push(make_segment(i));
			}

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

	fn score(&self) -> i64 {
		self.pos.len() as i64
	}
}

//impl Sheep {
//	fn is_naked(&self) -> bool {
//		self.naked
//	}
//
//	fn shear(&mut self) {
//		if self.is_naked() {
//			// Implementor methods can use the implementor's trait methods.
//			println!("{} is already naked...", self.name());
//		} else {
//			println!("{} gets a haircut!", self.name);
//
//			self.naked = true;
//		}
//	}
//}
//
//// Implement the `Animal` trait for `Sheep`.
//impl Animal for Sheep {
//	// `Self` is the implementor type: `Sheep`.
//	fn new(name: &'static str) -> Sheep {
//		Sheep { name: name, naked: false }
//	}
//
//	fn name(&self) -> &'static str {
//		self.name
//	}
//
//	fn noise(&self) -> &'static str {
//		if self.is_naked() {
//			"baaaaah?"
//		} else {
//			"baaaaah!"
//		}
//	}
//
//	// Default trait methods can be overridden.
//	fn talk(&self) {
//		// For example, we can add some quiet contemplation.
//		println!("{} pauses briefly... {}", self.name, self.noise());
//	}
//}
//
//fn main() {
//	// Type annotation is necessary in this case.
//	let mut dolly: Sheep = Animal::new("Dolly");
//	// TODO ^ Try removing the type annotations.
//
//	dolly.talk();
//	dolly.shear();
//	dolly.talk();
//}
