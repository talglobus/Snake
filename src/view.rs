extern crate rayon;

use piston_window::*;
use model::{Snake, Coord, Playable};
//use self::rayon::*;
//use self::rayon::iter::*;        // TODO: Find a better way of doing this

// TODO: Check whether the lifetime below is appropriate
pub struct View<'a> {
	pub text: String,
//	pub side: Option<Side>,
	pub ref_snake: &'a Snake,
	pub ref_food: &'a Coord,
	pub ref_prev_food: &'a Coord,
	pub is_full_refresh: bool,
}

struct Sides<A> {
	top: A,
	right: A,
	bottom: A,
	left: A,
}

//const SQUARE_WIDTH : f64 = 10.0;
const SQUARE_PADDING : f64 = 0.5;
pub const BOX_SIZE : i16 = 50;
const OUTER_PADDING: Sides<f64> = Sides {
	top: 40.0,
	right: 20.0,
	bottom: 20.0,
	left: 20.0,
};
const FOOD_FILL : [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const SNAKE_FILL : [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const EMPTY_FILL : [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BACKGROUND_FILL : [f32; 4] = [0.5, 0.5, 0.5, 1.0];

fn reset_grid(c: Context, g: &mut G2d) {
	let view_size = c.get_view_size();
	let w = view_size[0];
	let h = view_size[1];

	clear(BACKGROUND_FILL, g);

	let square_width = (w - OUTER_PADDING.left - OUTER_PADDING.right)
		/ BOX_SIZE as f64 - SQUARE_PADDING;
	let square_height = (h - OUTER_PADDING.top - OUTER_PADDING.bottom)
		/ BOX_SIZE as f64 - SQUARE_PADDING;
	// TODO: Find out if there is a way of staying square beyond leaving out square_height

	let make_rectangle_shape = |x: i16, y: i16| {
		[
			OUTER_PADDING.left + (square_width + SQUARE_PADDING) * x as f64,
			OUTER_PADDING.top + (square_height + SQUARE_PADDING) * y as f64,
			square_width,
			square_height
		]
	};

	for x in 0..BOX_SIZE {
		for y in 0..BOX_SIZE {
			rectangle(EMPTY_FILL,
					  make_rectangle_shape(x, y),
					  c.transform,
					  g,);
		}
	}
}

impl<'a> View<'a> {
	pub fn render(&mut self, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
		// we will scale everything according to the window size
		let view_size = c.get_view_size();
		let w = view_size[0];
		let h = view_size[1];

		if self.text == "Press <Space> to start" {
			// calculate proper font size
			let font_size = (w / 512.0 * 16.0) as u32;    // Change 16.0 back to 32.0 for one line

			// add some padding for a better view
			let padding = w / 512.0 * 20.0;
			// leave some space for text
			let side_top_padding = (font_size as f64) + padding * 2.0;
			let side_height = (h as f64) - side_top_padding - padding;
			let side_width = (w as f64) * 0.5 - padding * 1.5;

			// drawing part

			// clear the screen
			reset_grid(c, g);

			// draw text
			text::Text::new(font_size)
				.draw(
					&self.text,
					glyphs,
					&c.draw_state,
					c.transform.trans(padding, (font_size as f64) + padding),
					g,
				).unwrap();
		} else if self.text.contains("Game over!") {
			// calculate proper font size
			let font_size = (w / 512.0 * 16.0) as u32;    // Change 16.0 back to 32.0 for one line

			// add some padding for a better view
			let padding = w / 512.0 * 20.0;
			// leave some space for text
			let side_top_padding = (font_size as f64) + padding * 2.0;
			let side_height = (h as f64) - side_top_padding - padding;
			let side_width = (w as f64) * 0.5 - padding * 1.5;

			// drawing part

			// clear the screen
			reset_grid(c, g);

			// draw text
			text::Text::new(font_size)
				.draw(
					&self.text,
					glyphs,
					&c.draw_state,
					c.transform.trans(padding, (font_size as f64) + padding),
					g,
				).unwrap();
		} else {
			let square_width = (w - OUTER_PADDING.left - OUTER_PADDING.right)
				/ BOX_SIZE as f64 - SQUARE_PADDING;
			let square_height = (h - OUTER_PADDING.top - OUTER_PADDING.bottom)
				/ BOX_SIZE as f64 - SQUARE_PADDING;
			// TODO: Find out if there is a way of staying square beyond leaving out square_height

			// calculate proper font size
			let font_size = (w / 512.0 * 16.0) as u32;	// Change 16.0 back to 32.0 for one line

			// add some padding for a better view
			let padding = w / 512.0 * 20.0;

			rectangle(
				BACKGROUND_FILL,
				[
					0.0,
					0.0,
					w + padding,
					font_size as f64 + padding + 2.0,	// TODO: Fix that `5.0` fudge constant
				],
				c.transform,
				g,
			);

			// draw text
			text::Text::new(font_size)
				.draw(
					&self.text,
					glyphs,
					&c.draw_state,
					c.transform.trans(padding, (font_size as f64) + padding),
					g,
				).unwrap();

			if self.is_full_refresh {
				reset_grid(c, g);
			}

			// TODO: Check why the `mut` below is necessary
			let mut fill_grid = |color: [f32; 4], x: i16, y: i16| {
				rectangle(
					color,
					[
						OUTER_PADDING.left + (square_width + SQUARE_PADDING) * x as f64,
						OUTER_PADDING.top + (square_height + SQUARE_PADDING) * y as f64,
						square_width,
						square_height
					],
					c.transform,
					g,
				);
			};

			// If this square has a snake segment on it, draw the snake segment,...
			// Note in reading the process below that the last segment is not actually part...
			//... of the snake, but just marks where the snake most recently was to refill the grid

			// There are four things that need to be done here:
			// 1) Fill where in the grid the food is now
			// 2) Fill where in the grid the food disappeared
			// 3) Fill where in the grid the snake is now
			// 4) Fill where in the grid the snake just moved off

			// First, (1)
			fill_grid(FOOD_FILL, self.ref_food.x, self.ref_food.y);
			// Then, (2)
			fill_grid(EMPTY_FILL, self.ref_prev_food.x, self.ref_prev_food.y);
			// Next, (3)
			self.ref_snake.body_iter_with_head().for_each(
				|segment| fill_grid(SNAKE_FILL, segment.x, segment.y)
			);
			// Finally, (4)
			match self.ref_snake.pos.last() {
				Some(Coord {x, y}) => fill_grid(EMPTY_FILL, *x, *y),
				_ => ()
			}

		}
	}
}