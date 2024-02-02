

//= Allows


//= Imports


//= Constants
pub static mut LOG_TYPE_LEVEL: TraceLogLevel = TraceLogLevel::LogInfo;


//= Structures & Enumerations

#[derive(Debug, Clone, PartialEq)]
pub enum TraceLogLevel {
	LogAll = 0,			// Display all logs
	LogTrace,			// Trace logging, intended for internal use only
	LogDebug,			// Debug logging, used for internal debugging, it should be disabled on release builds
	LogInfo,			// Info logging, used for program execution info
	LogWarning,			// Warning logging, used on recoverable failures
	LogError,			// Error logging, used on unrecoverable failures
	LogFatal,			// Fatal logging, used to abort program: exit(EXIT_FAILURE)
	LogNone				// Disable logging
}
impl PartialOrd for TraceLogLevel {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		let lhs = self.clone() as i32;
		let rhs = other.clone() as i32;
		
		if lhs <  rhs { return Some(std::cmp::Ordering::Less); }
		if lhs > rhs { return Some(std::cmp::Ordering::Greater); }
		return Some(std::cmp::Ordering::Equal);
	}
}


//= Procedures

/// Set the current threshold (minimum) log level
pub fn set_trace_log_level(log_type: TraceLogLevel) {
	unsafe { LOG_TYPE_LEVEL = log_type; }
}

/// Show trace log messages (LOG_INFO, LOG_WARNING, LOG_ERROR, LOG_DEBUG)
#[macro_export]
macro_rules! tracelog {
	($a:expr,$($arg:tt)*) => {{
		if $a > $crate::logging::LOG_TYPE_LEVEL { println!($($arg)*) }
	}};
}