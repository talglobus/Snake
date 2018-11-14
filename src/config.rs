//#[macro_use]
//extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use outerwear::{IS_GUI, IS_VISIBLE, IS_VERBOSE};

/// Deserialization object definition for /config/main.json. Must be kept synchronized as such
//#[derive(Serialize, Deserialize, Debug)]
//pub struct MainConfig {
//	box_size: i16,
//	wall_buffer: i16,
//	tick_duration: f64,
//}

#[derive(Debug)]
pub struct ExportConfig {
	pub is_gui: bool,
	pub is_visible: bool,
	pub is_verbose: bool,
//	pub input_string_up: String,
//	pub input_string_down: String,
//	pub input_string_left: String,
//	pub input_string_right: String,
}

lazy_static! {
	pub static ref CONFIG: ExportConfig = ExportConfig {
		is_gui: *IS_GUI,
		is_visible: *IS_VISIBLE,
		is_verbose: *IS_VERBOSE,
//		input_string_up: "up".to_string(),
//		input_string_down: "down".to_string(),
//		input_string_left: "left".to_string(),
//		input_string_right: "right".to_string()
	};
}