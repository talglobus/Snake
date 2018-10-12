const SNAKE_INITIAL_LENGTH : int16 = 3;

#[derive(Clone, Copy)]
pub struct Coord {
	x: i16,
	y: i16
}

#[derive(Clone, Copy)]
pub struct Snake {
	pos: Vec<Coord>,
	direction: Direction
}

enum Direction {
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
	fn advance(&self) -> Self;

	fn rotate(&self, direction: Direction) -> Self;

	fn grow(&self) -> Self;

	fn score(&self) -> i64;

	// Traits can provide default method definitions.
//	fn talk(&self) {
//		println!("{} says {}", self.name(), self.noise());
//	}
}

impl Movable for Snake {
	/// Creates a `Snake`, building a body of `SNAKE_INITIAL_LENGTH` correctly-positioned segments
	fn new(head_pos: Coord, direction: Direction) -> Snake {
		let pos = vec![head_pos];

		let make_segment: fn(i16) -> Coord = match direction {
			Direction::Left => |i: i16| Coord { x: head_pos.x + i, y: head_pos.y},
			Direction::Right => |i: i16| Coord { x: head_pos.x - i, y: head_pos.y},
			Direction::Up => |i: i16| Coord { x: head_pos.x, y: head_pos.y - i},
			Direction::Down => |i: i16| Coord { x: head_pos.x, y: head_pos.y + i},
		};

		for i in 1..SNAKE_INITIAL_LENGTH {
			pos.push(make_segment(i));
		}

		Snake {
			pos, direction
		}
	}

	fn advance(&self) -> Self {
		let make_head_segment: fn(i16) -> Coord = match self.direction {
			Direction::Left => |i: i16| Coord { x: self.head_pos.x - i, y: self.head_pos.y},
			Direction::Right => |i: i16| Coord { x: self.head_pos.x + i, y: self.head_pos.y},
			Direction::Up => |i: i16| Coord { x: self.head_pos.x, y: self.head_pos.y + i},
			Direction::Down => |i: i16| Coord { x: self.head_pos.x, y: self.head_pos.y - i},
		};

		self.pos.insert(0, make_head_segment(1));
		self.pos.pop();	// TODO: Check if this can be combined with the previous line via chaining

		return *self;	// TODO: Check if this can be shortened
	}

	fn rotate(&self, direction: Direction) -> Self {
		Snake {
			pos: self.pos,
			direction
		}
	}

	fn grow(&self) -> Self {
		let make_segment: fn(i16) -> Coord = match direction {
			Direction::Left => |i: i16| Coord { x: self.pos[-1].x + i, y: self.pos[-1].y},
			Direction::Right => |i: i16| Coord { x: self.pos[-1].x - i, y: self.pos[-1].y},
			Direction::Up => |i: i16| Coord { x: self.pos[-1].x, y: self.pos[-1].y - i},
			Direction::Down => |i: i16| Coord { x: self.pos[-1].x, y: self.pos[-1].y + i},
		};

		self.pos.push(make_segment(1))
	}

	fn score(&self) -> i64 {
		self.pos.len()
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
