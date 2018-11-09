use model::{Coord, Direction, Playable, Snake, config};
use rand::Rng;

pub fn init_snake(box_size: i16) -> Snake {
	Snake::new(
		Coord{x: box_size / 2, y: box_size / 2},
		Direction::Left)	// TODO: Randomize this in the future?
}

pub fn pick_locus_within_buffer(box_size: i16, buffer: i16) -> Coord {
	Coord {
		x: rand::thread_rng().gen_range(buffer, box_size-buffer),
		y: rand::thread_rng().gen_range(buffer, box_size-buffer),
	}
}

pub fn pick_locus_off_snake(snake: &Snake, box_size: i16, wall_food_buffer: i16) -> Coord {
	let attempted_location = pick_locus_within_buffer(box_size, wall_food_buffer);
	// If any overlap exists, search again. Otherwise, return the point
	if snake.body_iter_with_head().any(|&pos| {
		attempted_location == pos
	}) {
		pick_locus_off_snake(snake, box_size, wall_food_buffer)
	} else {
		attempted_location
	}
}

pub fn is_body_collision(snake: &Snake) -> bool {
	// If the snake head lies on its model, return false
	match snake.pos.first() {
		Some (some_pos) => {
			snake.pos[1..snake.pos.len()-1].iter().any(|&pos| {
				*some_pos == pos
			})
		}
		None => false
	}

}

pub fn is_head_beyond_bounds(snake: &Snake, box_size: i16) -> bool {
	if let Some(head) = snake.pos.first() {
		head.x >= box_size || head.x < 0 || head.y >= box_size || head.y < 0
	} else {
		false
	}
}