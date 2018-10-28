extern crate piston_window;
extern crate rand;

mod view;
mod body;

use view::{View, BOX_SIZE};
use body::{Snake, Movable, Direction, Coord};
use piston_window::keyboard::Key;
use rand::Rng;
use std::fmt;
use std::time::{Instant, Duration};
use std::mem;

//const F: f64 = 5.28;
const TICK_DURATION: f64 = 0.04;	// TODO: Make this decrease over time
const WALL_FOOD_BUFFER: i16 = 2;
const FULL_REFRESH_ROUNDS: i32 = 1000;		// Number of updates between every full refresh

#[derive(Debug)]
pub enum GameState {			// TODO: Dehackify this working with `main.rs`
	Init { snake: Snake },
	Running { snake: Snake },
	Win { snake: Snake },
	Lose { snake: Snake },
}

// In order to use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for GameState {
	// This trait requires `fmt` with this exact signature.
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Write strictly the first element into the supplied output
		// stream: `f`. Returns `fmt::Result` which indicates whether the
		// operation succeeded or failed. Note that `write!` uses syntax which
		// is very similar to `println!`.
//		write!(f, "{}", match self {
//			GameState::Init => "Init",
//			GameState::Preparing { .. } => "Preparing",
//			GameState::Running { .. } => "Running",
//			GameState::Result { .. } => "Result",
//			GameState::FalseStart => "FalseStart",
//		})

		write!(f, "{}", match self {
			GameState::Init { .. } => "Init",
			GameState::Running { .. } => "Running",
			GameState::Win { .. } => "Result",
			GameState::Lose { .. } => "FalseStart",
		})
	}
}

#[derive(PartialEq, Debug)]
enum DirectionKey {		// TODO: Possibly unify this with `Direction` in `body.rs`
	Up,
	Down,
	Left,
	Right,
	None,
}

pub struct App {
	game_state: GameState,
	time_since_update: f64,
	updates_since_full_refresh: i32,
	last_pressed: DirectionKey,
	food_location: Coord,
	prev_food_location: Coord,
	newly_ended: bool,
	next_state: Option<GameState>,
}

fn init_snake() -> Snake {
	Snake::new(
		Coord{x: BOX_SIZE / 2, y: BOX_SIZE / 2},
		Direction::Left)	// TODO: Randomize this in the future?
}

fn pick_locus_random() -> Coord {
	Coord {
		x: rand::thread_rng().gen_range(WALL_FOOD_BUFFER, BOX_SIZE-WALL_FOOD_BUFFER+1),
		y: rand::thread_rng().gen_range(WALL_FOOD_BUFFER, BOX_SIZE-WALL_FOOD_BUFFER+1),
	}
}

fn is_body_collision(snake: &Snake) -> bool {
	// If the snake head lies on its body, lose
	snake.pos[1..].iter().any(|&pos| {
		match snake.pos.first() {
			Some (some_pos) => (*some_pos == pos),
			None => false
		}
	})
}

fn is_head_beyond_bounds(snake: &Snake) -> bool {
	if let Some(head) = snake.pos.first() {
		head.x >= BOX_SIZE || head.x < 0 || head.y >= BOX_SIZE || head.y < 0
	} else {
		false
	}
}

impl App {
	pub fn new() -> Self {
		App {
			game_state: GameState::Init {
				snake: init_snake()
			},
			time_since_update: 0.0,
			updates_since_full_refresh: 0,		// So bumping to `0` triggers refresh on first load
			last_pressed: DirectionKey::None,
			food_location: pick_locus_random(),
			prev_food_location: pick_locus_random(),		// Unused before value change
			newly_ended: true,
			next_state: None,
		}
	}

	pub fn update(&mut self, dt: f64) {
//		println!("{}", self.game_state);    // Debugging line to identify current state

		if let Some(game_state) = &mut self.next_state {
			println!("Replacing state to next state");
			self.game_state = mem::replace(game_state, GameState::Init {
				snake: init_snake()
			});
		}

		// This next line would be more efficient if it were only run after a state change, i.e....
		//... in the above `if` block. However, that breaks borrowing rules, so here it goes
		self.next_state = None;

		match &mut self.game_state {
			GameState::Running { snake } => {
				self.time_since_update += dt;
				// If at least one tick has gone by, change state corresponding to game action
				if self.time_since_update >= TICK_DURATION {
					println!("Updating, last updated before {:?}", self.time_since_update);
					self.time_since_update = 0.0;
					self.updates_since_full_refresh
						= if self.updates_since_full_refresh >= FULL_REFRESH_ROUNDS { 0 }
						else { self.updates_since_full_refresh + 1 };

					match (&self.last_pressed, &snake.direction) {
						(DirectionKey::None, _) => {},
						(DirectionKey::Left, Direction::Right)
						| (DirectionKey::Up, Direction::Down)
						| (DirectionKey::Right, Direction::Left)
						| (DirectionKey::Down, Direction::Up) => {},
						(DirectionKey::Left, _) => snake.rotate(Direction::Left),
						(DirectionKey::Up, _) => snake.rotate(Direction::Up),
						(DirectionKey::Right, _) => snake.rotate(Direction::Right),
						(DirectionKey::Down, _) => snake.rotate(Direction::Down),
					}

					println!("Advancing snake toward {:?}! {:?}", self.last_pressed, snake.pos);
					snake.advance();	// Advance the snake one tick

					// If the snake head lies on its body, lose
					if is_body_collision(snake) || is_head_beyond_bounds(snake) {
						println!("Changing state");
						self.newly_ended = true;
						let movable_snake = mem::replace(snake, init_snake());
						mem::replace(&mut self.next_state,
									 Some(GameState::Lose { snake: movable_snake }));
					}

					// If the snake eats the food, cause the snake to grow and reposition the food
					if snake.pos[0].x == self.food_location.x
						&& snake.pos[0].y == self.food_location.y {
						snake.grow();
						self.prev_food_location = self.food_location.clone();
						self.food_location = pick_locus_random();
						println!("Old food location: {:?}", self.prev_food_location);
						println!("New food location: {:?}", self.food_location);

					}
				}
			}
			GameState::Init { .. } => {

			}
			GameState::Win { snake } => {}
			GameState::Lose { .. } => {}
		}
	}

	pub fn key(&mut self, key: Key) {
		match (&self.game_state, key) {
			(&GameState::Running { .. }, Key::Left) => {
				self.last_pressed = DirectionKey::Left;
			},
			(&GameState::Running { .. }, Key::Up) => {
				self.last_pressed = DirectionKey::Up;
			},
			(&GameState::Running { .. }, Key::Right) => {
				self.last_pressed = DirectionKey::Right;
			},
			(&GameState::Running { .. }, Key::Down) => {
				self.last_pressed = DirectionKey::Down;
			},
			(&GameState::Init { .. }, Key::Space) => {
				self.game_state = GameState::Running {
					snake: init_snake(),
				}
			}
			(&GameState::Win { .. }, Key::Space) |
			(&GameState::Lose { .. }, Key::Space) => {
				self.last_pressed = DirectionKey::None;
				self.newly_ended = true;
				self.food_location = pick_locus_random();
				self.game_state = GameState::Init {
					snake: init_snake()
				};
			}
			_ => (),
		}
	}

	pub fn view(&mut self) -> View {
		match &self.game_state {
			GameState::Init { snake } => View {
				text: String::from("Press <Space> to start"),
				ref_snake: &snake,
				ref_food: &self.food_location,
				ref_prev_food: &self.prev_food_location,
				is_full_refresh: true,
			},
			GameState::Running { snake, .. } => View {
				text: format!("Score: {:.*}", 2, snake.score()),
				ref_snake: &snake,
				ref_food: &self.food_location,
				ref_prev_food: &self.prev_food_location,
				is_full_refresh: self.updates_since_full_refresh == 0,
			},
			GameState::Win { snake } => View {
				text: format!("You win! Score: {:.*}", 2, snake.score()),
				ref_snake: &snake,
				ref_food: &self.food_location,
				ref_prev_food: &self.prev_food_location,
				is_full_refresh: true,
			},
			GameState::Lose { snake } => View {
				text: format!("You lose! Score: {:.*}", 2, snake.score()),
				ref_snake: &snake,
				ref_food: &self.food_location,
				ref_prev_food: &self.prev_food_location,
				is_full_refresh: true,
			},
		}
	}
}