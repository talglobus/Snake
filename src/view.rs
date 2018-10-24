use piston_window::*;
use side::Side;
use body::Snake;

pub struct View {
	pub text: String,
	pub side: Option<Side>,
//	pub body: Snake,
}

const SQUARE_WIDTH : f64 = 10.0;
const SQUARE_PADDING : f64 = 1.0;
const BOX_SIZE : i16 = 100;

impl View {
	pub fn render(&mut self, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
		// we will scale everything according to the window size
		let view_size = c.get_view_size();
		let w = view_size[0];
		let h = view_size[1];

		if self.text == "Press <Space> to start" {
			let square_width = (w - SQUARE_PADDING) / (BOX_SIZE as f64);
			let square_height = (h - SQUARE_PADDING) / (BOX_SIZE as f64);
			// TODO: Find out if there is a way of staying square beyond leaving out square_height
			clear([0.5, 0.5, 0.5, 1.0], g);
			for x in 0..BOX_SIZE {
				for y in 0..BOX_SIZE {
//					if self.body.pos.x
					rectangle(
						[0.0, 0.0, 0.0, 1.0],
						[
							SQUARE_PADDING + (square_width + SQUARE_PADDING) * y as f64,
							SQUARE_PADDING + (square_height + SQUARE_PADDING) * x as f64,
							square_width,
							square_height
						],
						c.transform,
						g,
					);
				}
			}
		} else {
			// calculate proper font size
			let font_size = (w / 512.0 * 16.0) as u32;	// Change 16.0 back to 32.0 for one line

			// add some padding for a better view
			let padding = w / 512.0 * 20.0;
			// leave some space for text
			let side_top_padding = (font_size as f64) + padding * 2.0;
			let side_height = (h as f64) - side_top_padding - padding;
			let side_width = (w as f64) * 0.5 - padding * 1.5;

			// which rectangle will be brighter
			let left_color_difference = match self.side {
				None => 0.0,
				Some(Side::Left) => 0.125,
				Some(Side::Right) => -0.125,
			};

			// drawing part

			// clear the screen
			clear([0.5, 0.5, 0.5, 1.0], g);

			// draw text
			text::Text::new(font_size)
				.draw(
					&self.text,
					glyphs,
					&c.draw_state,
					c.transform.trans(padding, (font_size as f64) + padding),
					g,
				).unwrap();

			// draw left rectangle
			rectangle(
				[0.5 + left_color_difference, 0.0, 0.0, 1.0],
				[padding, side_top_padding, side_width, side_height],
				c.transform,
				g,
			);

			// draw right rectangle
			rectangle(
				[0.5 - left_color_difference, 0.0, 0.0, 1.0],
				[
					side_width + padding * 2.0,
					side_top_padding,
					side_width,
					side_height,
				],
				c.transform,
				g,
			);
		}
	}
}