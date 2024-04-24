

use std::{fs::{self, File}, io::Write};

use chrono::Utc;

use crate::pleroma::Pleroma;


/// All of the possible errors
pub enum Error {
	TestError,

	LogDoesntExist,

	FileLoadingFailed(String),
}
impl ToString for Error {
	fn to_string(&self) -> String {
		match self {
			Error::TestError => "[WARNING] - Standard test for debugging system.".to_string(),

			Error::LogDoesntExist => "[WARNING] - Debugging log doesn't exist.".to_string(),

			Error::FileLoadingFailed ( name ) => format!("[WARNING] - File ({}) failed to load.", name),
		}
	}
}
impl Into<i32> for Error {
	fn into(self) -> i32 {
		match self {
			Error::TestError => 2,

			Error::LogDoesntExist => 2,

			Error::FileLoadingFailed(..) => 3,
		}
	}
}

impl Pleroma {
	/// Logs error to Console, log.txt, and screen depending on severity
	pub fn log(&mut self, error: Error) {
		let sys_time = Utc::now();
		let error_text = error.to_string();
		let severity: i32 = error.into();

		let formatted_str = format!("{}{}", sys_time.format("[%Y-%m-%e][%T]"), error_text);

		//* Print to console */
		println!("{formatted_str}");

		//* Print to log */
		if severity >= 2 {
			let mut file: File;
			if fs::metadata("log.txt").is_ok() {
				let result = File::open("log.txt");
				if result.is_err() { self.log(Error::FileLoadingFailed("log.txt".to_string())) }
				file = result.ok().unwrap();
			} else {
				let result = File::create("log.txt");
				if result.is_err() { self.log(Error::FileLoadingFailed("log.txt".to_string())) }
				self.log(Error::LogDoesntExist);
				file = result.ok().unwrap();
			}
			let _ = file.write(formatted_str.as_bytes());
		}

		//* Print to screen */
		// TODO Do this after fonts
		if self.screen.raylib_init {
			self.screen.error_log.push((formatted_str,100));
		}
	}	
}