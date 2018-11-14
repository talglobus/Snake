//extern crate model;

use std::io::{self, Write, Stdout};
use serde;
use serde_json;
use model::GameState;
//use std::sync::Mutex;
use std::thread;
use std::sync::mpsc;

pub fn get_command() -> String {
	let mut input = String::new();
	match io::stdin().read_line(&mut input) {
		Ok(_bytes) => {
			input
		}
		Err(error) => panic!("error: {}", error),
	}
}

pub struct Commander {
	listener: mpsc::Receiver<String>,
	terminator: mpsc::Sender<i16>,
	thread: thread::JoinHandle<()>,
}

// TODO: Make this dynamic type-wise, perhaps call it `unblock` or something like that, `unblocker`
// TODO: Potential problems, if multiple messages on channel before `receive`, only first is read
impl Commander {
	pub fn new() -> Commander {
		let (data_tx, data_rx) = mpsc::channel();
		let (term_tx, term_rx) = mpsc::channel();

		let handle = thread::spawn(move || {
			loop {
				let mut input = String::new();
				match io::stdin().read_line(&mut input) {
					Ok(_) => {	// TODO: Possibly use the pseudo-exit-code received here
						data_tx.send(input).unwrap();
					}
					Err(error) => panic!("error: {}", error),
				}

				match term_rx.try_recv() {
					// Note, a disconnect from the sender side is really equivalent to `terminate`
					Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
						()	// Return from the thread, which I think should work
					},
					Err(mpsc::TryRecvError::Empty) => {},
				}
			}

		});

		Commander {
			listener: data_rx,
			terminator: term_tx,
			thread: handle,
		}
	}

	// TODO: Termination has THE SAME notification-blocked problem as threading was meant to solve
	pub fn receive(&mut self) -> Option<String> {
		match self.listener.try_recv() {
			Ok(new_command) => {
				Some(new_command)
			},
			Err(mpsc::TryRecvError::Empty) => {
				None
			},
			Err(mpsc::TryRecvError::Disconnected) => {
				panic!("The listening channel disconnected from the tx end");
//				None	// Logically unreachable, but type-wise really should be here
			}
		}
	}

//	fn terminate(&mut self) {				// It seems this is going to go unused
//		self.terminator.send(0).unwrap();
////		self.thread.join().unwrap();	// Technically necessary, but apparently not possible
//	}
}

pub fn push_state(state: &GameState) -> io::Result<()> {
	let stdout: &Stdout = &io::stdout();
	let mut handle = stdout.lock();

	handle.write((serde_json::to_string(&state)? + "\n").as_bytes())?;

	Ok(())
}