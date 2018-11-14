#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

extern crate piston_window;
extern crate find_folder;
extern crate serde;
extern crate serde_json;
extern crate rand;

mod view;
mod model;
mod config;
mod outerwear;
mod action;
mod io;

use piston_window::*;

use action::{App, TICK_DURATION};
use config::CONFIG;
use io::Commander;
//use model::DirectionKey;

fn main() {
	println!("{}", CONFIG.is_gui);

	let mut app = App::new();

	if CONFIG.is_gui || CONFIG.is_visible {
		let mut window: PistonWindow
			= WindowSettings::new("Snake", [512; 2])
			.controllers(false).build().unwrap();	// Controllers should disable keyboard

		let assets = find_folder::Search::ParentsThenKids(3, 3)
			.for_folder("assets")
			.unwrap();

		let font = &assets.join("FiraSans-Regular.ttf");
		let factory = window.factory.clone();
		let texture_settings = TextureSettings::new();

		let mut glyphs
			= Glyphs::new(font, factory, texture_settings).unwrap();
//		window.set_max_fps((1.0 / TICK_DURATION) as u64 + 1);
//		window.set_ups((1.0 / TICK_DURATION) as u64 + 1);

		if !CONFIG.is_gui {
			let mut commander = io::Commander::new();
			println!("Hit <enter> to begin");
			app.update(0.00000001);		// TODO: Check if this needs to run once at the beginning
			while let Some(e) = window.next() {
				if !app.is_continue() { break; }

				let received_command = commander.receive();
				let command = received_command.as_ref().map(|s| s.as_str());
				if command != None {
					println!("{:?}", command);
				}

				// TODO: Fix the arbitrary huge `dt` here
				match command {
					Some("up\n") => {
						println!("Going up!");
						app.key(Key::Up);
						app.update(100.);
					},
					Some("down\n") => {
						println!("Going down!");
						app.key(Key::Down);
						app.update(100.);
					},
					Some("left\n") => {
						println!("Going left!");
						app.key(Key::Left);
						app.update(100.);
					},
					Some("right\n") => {
						println!("Going right!");
						app.key(Key::Right);
						app.update(100.);
					},
					Some("\n") => {
						println!("Just advancing!");
						app.key(Key::Space);
						app.update(100.);	// Essentially an "ack" to advance the snake
					},
					_ => ()
				}
				window.draw_2d(&e, |c, g| app.view().render(c, g, &mut glyphs));
			}
			println!("Game is over!");
//			while let Some(e) = window.next() {
//				e.update(|args| app.update(args.dt));
//				match commander.receive().as_ref().map(|s| s.as_str()) {
//					Some("up\n") => { println!("Going up!"); app.key(Key::Up); },
//					Some("down\n") => { println!("Going down!"); app.key(Key::Down); },
//					Some("left\n") => { println!("Going left!"); app.key(Key::Left); },
//					Some("right\n") => { println!("Going right!"); app.key(Key::Right); },
//					Some("\n") => app.key(Key::Space),
//					_ => ()
//				}
//				window.draw_2d(&e, |c, g| app.view().render(c, g, &mut glyphs));
//			}
		} else {
			while let Some(e) = window.next() {
				e.update(|args| app.update(args.dt));
				e.press(|button| if let Button::Keyboard(key) = button {
					app.key(key)
				});
				window.draw_2d(&e, |c, g| app.view().render(c, g, &mut glyphs));
			}
		}
	} else {
		// Note that this means of operation entirely avoids using piston, so... much faster
		let mut commander = io::Commander::new();
		println!("Hit <enter> to begin");
		app.update(0.00000001);		// TODO: Check if this needs to run once at the beginning
		while app.is_continue() {
			let received_command = commander.receive();
			let command = received_command.as_ref().map(|s| s.as_str());
			if command != None {
				println!("{:?}", command);
			}

			// TODO: Fix the arbitrary huge advance here
			match command {
				Some("up\n") => {
					println!("Going up!");
					app.key(Key::Up);
					app.update(100.);
				},
				Some("down\n") => {
					println!("Going down!");
					app.key(Key::Down);
					app.update(100.);
				},
				Some("left\n") => {
					println!("Going left!");
					app.key(Key::Left);
					app.update(100.);
				},
				Some("right\n") => {
					println!("Going right!");
					app.key(Key::Right);
					app.update(100.);
				},
				Some("\n") => {
					println!("Just advancing!");
					app.key(Key::Space);
					app.update(100.);	// Essentially an "ack" to advance the snake
				},
				_ => ()
			}
		}
		println!("Game is over!");
	}
}