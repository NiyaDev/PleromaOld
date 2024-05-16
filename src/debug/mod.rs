

pub mod errors;
use crate::pleroma::Pleroma;
use std::fs::OpenOptions;
use std::io::prelude::*;
use bitflags::bitflags;
use chrono::Local;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LogLevel {
	Info,
	Error,
	Critical,
	None,
}
impl Into<u8> for LogLevel {
	fn into(self) -> u8 {
		match self {
			LogLevel::Info => 0,
			LogLevel::Error => 1,
			LogLevel::Critical => 2,
			LogLevel::None => 3,
		}
	}
}

bitflags! {
	/// ### DebugFlags
	/// Flags designating what the debug system should do.
	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
	pub struct DebugFlags: u8 {
		const LOG_ENABLE	= 0b00000001;
		const SCRN_ENABLE	= 0b00000010;
		const INFO_ENABLE	= 0b00000100;
		const DEBUG_ENABLE	= 0b10000000;
		
		const _ = !0;
	}
}

impl Pleroma {
	
	/// ### log
	/// Prints a message to Pleroma's log system.
	/// 
	/// ##### Includes:
	/// - Console
	/// - Log file (if enabled)
	/// - Screen (if enabled)
	pub fn log(&mut self, message: errors::PlError) {
		let level_value: u8 = self.get_log_level().into();
		let message_level: LogLevel = message.into();
		let message_level_value: u8 = message_level.into();
		
		let sys_time = Local::now();
		let formatted_message = format!("{}{}", sys_time.format("[%Y-%m-%e][%T]"), message.to_string());
		
		if level_value >= message_level_value {
			//* Print to console */
			println!("{}", formatted_message);
			
			//* Print to log */
			if self.get_debug_setting(DebugFlags::LOG_ENABLE) {
				let file = OpenOptions::new()
					.append(true)
					.create(true)
					.open("log.txt");
				if let Err(e) = writeln!(file.unwrap(), "{}", formatted_message) {
					eprintln!("Couldn't write to file: {}", e);
				}
			}
			
			//* Print to screen */
			if self.get_debug_setting(DebugFlags::SCRN_ENABLE) {
				self.push_message(formatted_message);
			}
		}
	}
	/// ### Draw_debug_info
	/// Draws debug info, including current FPS / Target FPS, number of drawn models and textures, etc.
	/// 
	/// TODO
	pub fn draw_debug_info(&self) {}
	
}