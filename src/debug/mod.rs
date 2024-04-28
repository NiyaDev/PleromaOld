

use std::fs::OpenOptions;
use std::io::prelude::*;

use chrono::Utc;

use crate::pleroma::Pleroma;


pub static mut DEBUG_LOG: Option<Vec<(String, i32)>> = None;


/// All of the possible errors
pub enum Error {
	TestError,

	LogDoesntExist,

	RenderTextureDoesntExist,
}
impl ToString for Error {
	fn to_string(&self) -> String {
		match self {
			Error::TestError => "[WARNING] - Standard test for debugging system.".to_string(),

			Error::LogDoesntExist => "[WARNING] - Debugging log doesn't exist.".to_string(),

			Error::RenderTextureDoesntExist => "[MAJOR] - RenderTexture not set up while drawing.".to_string(),
		}
	}
}
impl Into<i32> for Error {
	fn into(self) -> i32 {
		match self {
			Error::TestError => 2,

			Error::LogDoesntExist => 2,

			Error::RenderTextureDoesntExist => 0,
		}
	}
}

impl Pleroma {

	/// Starts debug mode and initializes all required variables
	pub fn debug_mode() {
		unsafe { DEBUG_LOG = Some(Vec::new()); }
	}

}


/// Logs error to Console, log.txt, and screen depending on severity
pub fn log(error: Error) {
	let sys_time = Utc::now();
	let error_text = error.to_string();
	let severity: i32 = error.into();

	let formatted_str = format!("{}{}", sys_time.format("[%Y-%m-%e][%T]"), error_text);

	//* Print to console */
	println!("{formatted_str}");

	//* Print to log */
	if severity >= 2 {
		let file = OpenOptions::new()
			.append(true)
			.create(true)
			.open("log.txt");
		if let Err(e) = writeln!(file.unwrap(), "{}", formatted_str) {
			eprintln!("Couldn't write to file: {}", e);
		}
	}

	//* Print to screen */
	unsafe {
		if DEBUG_LOG.is_some() {
			DEBUG_LOG.as_mut().unwrap().push((error_text,100));
		}
	}
}