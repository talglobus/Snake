use std::fmt;
extern crate serde;
extern crate serde_json;
use model::Snake;

#[derive(Debug, Serialize, Deserialize)]
pub enum GameState {
	Init { snake: Snake },
	Running { snake: Snake },
	Win { snake: Snake },
	Lose { snake: Snake },
}

impl GameState {
	fn name(&self) -> &str {
		match self {
			GameState::Init { .. } => "Init",
			GameState::Running { .. } => "Running",
			GameState::Win { .. } => "Result",
			GameState::Lose { .. } => "FalseStart",
		}
	}
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