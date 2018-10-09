extern crate piston_window;
extern crate rand;

mod side;
mod view;

use side::Side;
use view::View;
use piston_window::keyboard::Key;
use rand::Rng;
use std::fmt;

const F: f64 = 5.28;

enum GameState {
	Init,
	Preparing { time_to_start: f64 },
	Running { elapsed_time: f64, side: Side },
	Result { elapsed_time: f64, is_correct: bool },
	FalseStart,
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
		write!(f, "{}", match self {
			GameState::Init => "Init",
			GameState::Preparing { .. } => "Preparing",
			GameState::Running { .. } => "Running",
			GameState::Result { .. } => "Result",
			GameState::FalseStart => "FalseStart",
		})
	}
}

pub struct Cumulatives {
	correct_rounds: i64,
	total_rounds: i64,
	time_elapsed: f64,
}

pub struct App {
	game_state: GameState,
	cumulative: Cumulatives,
	newly_ended: bool,
}

impl App {
	pub fn new() -> Self {
		App {
			game_state: GameState::Init,
			cumulative: Cumulatives {
				correct_rounds: 0,
				total_rounds: 0,
				time_elapsed: 0.0,
			},
			newly_ended: true,
		}
	}

	pub fn update(&mut self, dt: f64) {
//		println!("{}", self.game_state);	// Debugging line to identify current state
		match self.game_state {
			GameState::Preparing { time_to_start } => {
				self.newly_ended = true;

				let time_to_start = time_to_start - dt;

				if time_to_start < 0.0 {
					self.game_state = GameState::Running {
						elapsed_time: 0.0,
						side: if rand::thread_rng().gen() {
							Side::Left
						} else {
							Side::Right
						},
					}
				} else {
					self.game_state = GameState::Preparing {
						time_to_start: time_to_start,
					}
				}
			}
			GameState::Running { elapsed_time, side } => {
				self.game_state = GameState::Running {
					elapsed_time: elapsed_time + dt,
					side: side,
				}
			}
			GameState::Init { .. } => {

			}
			GameState::Result { elapsed_time, is_correct } => {
				if self.newly_ended {
					self.newly_ended = false;
					self.cumulative.correct_rounds += if is_correct { 1 } else { 0 };
					self.cumulative.total_rounds += 1;
					self.cumulative.time_elapsed += elapsed_time;
				}
			}
			_ => (),
		}
	}

	pub fn key(&mut self, key: Key) {
		match (&self.game_state, key) {
			(&GameState::Preparing { .. }, _) => self.game_state = GameState::FalseStart,
			(&GameState::Running { elapsed_time, side }, Key::Left) |
			(&GameState::Running { elapsed_time, side }, Key::Right) => {
				self.game_state = GameState::Result {
					elapsed_time: elapsed_time,
					is_correct: match (key, side) {
						(Key::Left, Side::Left) | (Key::Right, Side::Right) => true,
						_ => false,
					},
				}
			}
			(&GameState::Init { .. }, Key::Space) |
			(&GameState::Result { .. }, Key::Space) |
			(&GameState::FalseStart, Key::Space) => {
				self.game_state = GameState::Preparing { time_to_start: 1.0 }
			}
			_ => (),
		}
	}

	pub fn view(&mut self) -> View {
		match self.game_state {
			GameState::Init => View {
				text: String::from("Press <Space> to start"),
				side: None,
			},
			GameState::Preparing { time_to_start } => View {
				text: format!("time to start: {:.*}", 2, time_to_start),
				side: None,
			},
			GameState::Running { elapsed_time, side } => View {
				text: format!("elapsed time: {:.*}", 2, elapsed_time),
				side: Some(side),
			},
			GameState::Result {
				elapsed_time,
				is_correct,
			} => View {
				text: format!(
					"You {}! Elapsed time: {:.*}. {:.*}% correct, {:.*}s per round",
					if is_correct { "win" } else { "lose" },
					2,
					elapsed_time,
					2,
					(self.cumulative.correct_rounds as f64) / (self.cumulative.total_rounds as f64) * 100.0,
					2,
					self.cumulative.time_elapsed / (self.cumulative.total_rounds as f64),
				),
				side: None,
			},
			GameState::FalseStart => View {
				text: String::from("False start!"),
				side: None,
			},
		}
	}
}