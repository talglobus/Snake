extern crate piston_window;
extern crate rand;

mod side;
mod view;
mod body;

use side::Side;
use view::{View, BOX_SIZE};
use body::{Snake, Movable, Direction, Coord};
use piston_window::keyboard::Key;
use rand::Rng;
use std::fmt;
use std::time::{Instant, Duration};

const F: f64 = 5.28;
const TICK_DURATION: Duration = Duration::from_millis(250);	// TODO: Make this increase

//enum GameState {
//	Init,
//	Preparing { time_to_start: f64 },
//	Running { elapsed_time: f64, side: Side },
//	Result { elapsed_time: f64, is_correct: bool },
//	FalseStart,
//}

enum GameState {
	Init { snake: Snake },
	Running { snake: Snake, last_updated: Instant },
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

//pub struct Cumulatives {
//	correct_rounds: i64,
//	total_rounds: i64,
//	time_elapsed: f64,
//}

#[derive(PartialEq)]
enum DirectionKey {		// TODO: Possibly unify this with `Direction` in `body.rs`
	Up,
	Down,
	Left,
	Right,
	None,
}

pub struct App {
	game_state: GameState,
	last_pressed: DirectionKey,
	newly_ended: bool,
}

fn init_snake() -> Snake {
	Snake::new(
		Coord{x: BOX_SIZE / 2, y: BOX_SIZE / 2},
		Direction::Left)	// TODO: Randomize this in the future?
}

impl App {
	pub fn new() -> Self {
		App {
			game_state: GameState::Init {
				snake: init_snake()
			},
			last_pressed: DirectionKey::None,
//			cumulative: Cumulatives {
//				correct_rounds: 0,
//				total_rounds: 0,
//				time_elapsed: 0.0,
//			},
			newly_ended: true,
		}
	}

	pub fn update(&mut self, dt: f64) {
		println!("{}", self.game_state);	// Debugging line to identify current state
//		match self.game_state {
//			GameState::Preparing { time_to_start } => {
//				self.newly_ended = true;
//
//				let time_to_start = time_to_start - dt;
//
//				if time_to_start < 0.0 {
//					self.game_state = GameState::Running {
//						elapsed_time: 0.0,
//						side: if rand::thread_rng().gen() {
//							Side::Left
//						} else {
//							Side::Right
//						},
//					}
//				} else {
//					self.game_state = GameState::Preparing {
//						time_to_start: time_to_start,
//					}
//				}
//			}
//			GameState::Running { elapsed_time, side } => {
//				self.game_state = GameState::Running {
//					elapsed_time: elapsed_time + dt,
//					side: side,
//				}
//			}
//			GameState::Init { .. } => {
//
//			}
//			GameState::Result { elapsed_time, is_correct } => {
//				if self.newly_ended {
//					self.newly_ended = false;
//					self.cumulative.correct_rounds += if is_correct { 1 } else { 0 };
//					self.cumulative.total_rounds += 1;
//					self.cumulative.time_elapsed += elapsed_time;
//				}
//			}
//			_ => (),
//		}

		match &mut self.game_state {
			GameState::Running { snake, last_updated } => {
				let new_last_updated = Instant::now();

				// If at least one tick has gone by, change state corresponding to game action
				if new_last_updated.duration_since(*last_updated) >= TICK_DURATION {
					match self.last_pressed {
						DirectionKey::None => {},
						DirectionKey::Left => snake.rotate(Direction::Left),
						DirectionKey::Up => snake.rotate(Direction::Up),
						DirectionKey::Right => snake.rotate(Direction::Right),
						DirectionKey::Down => snake.rotate(Direction::Down),
					}
				}

				snake.advance();	// Advance the snake one tick
				// Always return the same `.game_state` at the end, which may even be unnecessary
				// NOTE: In this case it seems it was avoidable, but if it weren't mutability...
				//... cause issues with outputting a mutable reference where a struct is expected
//				self.game_state = GameState::Running {
//					snake, last_updated,
//				}
			}
			GameState::Init { .. } => {

			}
			GameState::Win { snake } => {
//				if self.newly_ended {
//					self.newly_ended = false;
//					self.cumulative.correct_rounds += if is_correct { 1 } else { 0 };
//					self.cumulative.total_rounds += 1;
//					self.cumulative.time_elapsed += elapsed_time;
//				}
			}
			GameState::Lose { snake } => {
//				if self.newly_ended {
//					self.newly_ended = false;
//					self.cumulative.correct_rounds += if is_correct { 1 } else { 0 };
//					self.cumulative.total_rounds += 1;
//					self.cumulative.time_elapsed += elapsed_time;
//				}
			}
//			_ => (),
		}
	}

	pub fn key(&mut self, key: Key) {
		match (&self.game_state, key) {
//			(&GameState::Preparing { .. }, _) => self.game_state = GameState::FalseStart,
			(&GameState::Running { .. }, Key::Left) => {
				self.last_pressed = DirectionKey::Left;
//				self.game_state = GameState::Running {
//					snake, last_updated
//				}
			},
			(&GameState::Running { .. }, Key::Up) => {
				self.last_pressed = DirectionKey::Up;
//				self.game_state = GameState::Running {
//					snake, last_updated
//				}
			},
			(&GameState::Running { .. }, Key::Right) => {
				self.last_pressed = DirectionKey::Right;
//				self.game_state = GameState::Running {
//					snake, last_updated
//				}
			},
			(&GameState::Running { .. }, Key::Down) => {
				self.last_pressed = DirectionKey::Down;
//				self.game_state = GameState::Running {
//					snake, last_updated
//				}
//				self.game_state = GameState::Result {
//					elapsed_time: elapsed_time,
//					is_correct: match (key, side) {
//						(Key::Left, Side::Left) | (Key::Right, Side::Right) => true,
//						_ => false,
//					},
//				}
			},
			(&GameState::Init { .. }, Key::Space) => {
				self.game_state = GameState::Running {
					snake: init_snake(),	// TODO: This should really use the previous `snake`
					last_updated: Instant::now()
				}
			}
			(&GameState::Win { .. }, Key::Space) |
			(&GameState::Lose { .. }, Key::Space) => {
				self.last_pressed = DirectionKey::None;
				self.newly_ended = true;
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
			},
//			GameState::Preparing { time_to_start } => View {
//				text: format!("time to start: {:.*}", 2, time_to_start),
//				side: None,
//			},
			GameState::Running { last_updated, snake } => View {
//				text: format!("elapsed time: {:.*}", 2, elapsed_time),
				text: format!("Score: {:.*}", 2, snake.score()),
				ref_snake: &snake,
			},
			GameState::Win { snake } => View {
				text: format!("You win! Score: {:.*}", 2, snake.score()),
				ref_snake: &snake,
			},
			GameState::Lose { snake } => View {
				text: format!("You win! Score: {:.*}", 2, snake.score()),
				ref_snake: &snake,
			},
		}
	}
}