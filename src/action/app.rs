use model::{Snake, Playable, Direction, DirectionKey, Coord, is_body_collision,
			is_head_beyond_bounds, pick_locus_off_snake, pick_locus_within_buffer, GameState};
use std::mem;
use piston_window::keyboard::Key;
use view::{View, BOX_SIZE};

// TODO: Move the following to a config file
pub const TICK_DURATION: f64 = 0.04;	// TODO: Make this decrease over time, from 0.04
const WALL_FOOD_BUFFER: i16 = 0;
// Set number of updates between every full refresh to 10% of complete re-render equivalent
const FULL_REFRESH_ROUNDS: i32 = (BOX_SIZE as i32 / 10 as i32) * BOX_SIZE as i32;

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

impl App {
	pub fn new() -> Self {
		App {
			game_state: GameState::Init {
				snake: Snake { ..Default::default() }
			},
			time_since_update: 0.0,
			updates_since_full_refresh: 0,		// So bumping to `0` triggers refresh on first load
			last_pressed: DirectionKey::None,
			food_location: pick_locus_within_buffer(BOX_SIZE, WALL_FOOD_BUFFER),
			prev_food_location: pick_locus_within_buffer(BOX_SIZE, WALL_FOOD_BUFFER),		// Unused before value change
			newly_ended: true,
			next_state: None,
		}
	}

	pub fn update(&mut self, dt: f64) {
//		println!("{}", self.game_state);    // Debugging line to identify current state

		if let Some(game_state) = &mut self.next_state {
			println!("Replacing state to next state");
			self.game_state = mem::replace(game_state, GameState::Init {
				snake: Snake { ..Default::default() }
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
//					println!("Updating, last updated before {:?}", self.time_since_update);
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

					{
						// The following line is an unfortunate attempt to keep the compiler happy
						let food_location = self.food_location.clone();

						snake.advance_conditional_grow(
							&|snake| snake.pos[0].x == food_location.x
								&& snake.pos[0].y == food_location.y);
					}

					// If the snake head lies on its model or escapes its bounds, lose
					//... TODO: If `SNAKE_ADVANCE_DISTANCE` is over 1 this will not always work
					if is_body_collision(snake) || is_head_beyond_bounds(snake, BOX_SIZE) {
						println!("Changing state to loss");
						self.newly_ended = true;
						let movable_snake = mem::replace(snake, Snake { ..Default::default() });
						mem::replace(&mut self.next_state,
									 Some(GameState::Lose { snake: movable_snake }));
					}

					// If the snake has just eaten (landed on) the food, reposition the food....
					//... TODO: If `SNAKE_ADVANCE_DISTANCE` is over 1 this will not always work
					if snake.pos[0].x == self.food_location.x
						&& snake.pos[0].y == self.food_location.y {
						mem::replace(&mut self.prev_food_location, self.food_location);
						self.food_location = pick_locus_off_snake(
							&snake,
							BOX_SIZE,
							WALL_FOOD_BUFFER);
						println!("Old food location: {:?}", self.prev_food_location);
						println!("New food location: {:?}", self.food_location);
					}
				}
			}
			GameState::Init { .. } => {

			}
			GameState::Win { .. } => {}
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
					snake: Snake { ..Default::default() },
				}
			}
			(&GameState::Win { .. }, Key::Space) |
			(&GameState::Lose { .. }, Key::Space) => {
				self.last_pressed = DirectionKey::None;
				self.newly_ended = true;
				self.food_location = pick_locus_within_buffer(BOX_SIZE, WALL_FOOD_BUFFER);
				self.game_state = GameState::Init {
					snake: Snake { ..Default::default() }
				};
			}
			_ => (),
		}
	}

	pub fn view(&mut self) -> View {
//		{
//			let this_self = &self;
//			push_state(&this_self.game_state);
//		}

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
				text: format!("Game over! Score: {:.*}", 2, snake.score()),
				ref_snake: &snake,
				ref_food: &self.food_location,
				ref_prev_food: &self.prev_food_location,
				is_full_refresh: true,
			},
		}
	}

	pub fn is_continue(&self) -> bool {
		match self.game_state {
			GameState::Lose { .. } | GameState::Win { .. } => false,
			_ => true
		}
	}
}