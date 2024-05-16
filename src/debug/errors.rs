

use super::LogLevel;


/// Types of errors that can be a result of systems and functions.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlError {
	Default,
	TestError,
	TestCitical,
	
	EditingRenderSize,
	RenderTextureDoesntExist,
	KeybindCalledAxisInBool,
}
//impl Into<u16> for PlError {
//	fn into(self) -> u16 {
//		match self {
//			PlError::Default => 0,
//			PlError::TestError => 0,
//			PlError::TestCitical => 0
//			,
//			PlError::EditingRenderSize => 1,
//			PlError::RenderTextureDoesntExist => 2,
//			PlError::KeybindCalledAxisInBool => todo!(),
//		}
//	}
//}
impl Into<LogLevel> for PlError {
	fn into(self) -> LogLevel {
		match self {
			PlError::Default => LogLevel::Info,
			PlError::TestError => LogLevel::Error,
			PlError::TestCitical => LogLevel::Critical,
			
			PlError::EditingRenderSize => LogLevel::Error,
			PlError::RenderTextureDoesntExist => LogLevel::Error,
			PlError::KeybindCalledAxisInBool => todo!(),
		}
	}
}
impl ToString for PlError {
	fn to_string(&self) -> String {
		match self {
			PlError::Default => "[INFO] - Info test.".to_string(),
			PlError::TestError => "[ERROR] - Error test.".to_string(),
			PlError::TestCitical => "[CRITICAL] - Critical error test. That's about it, see yah.".to_string(),
			
			PlError::EditingRenderSize => "[ERROR] - Attempted to edit render resolution while drawing to screen.".to_string(),
			PlError::RenderTextureDoesntExist => "[ERROR] - Attempted to start drawing without a RenderTexture.".to_string(),
			PlError::KeybindCalledAxisInBool => "[INFO] - Attempted to use axis in button press.".to_string(),
		}
	}
}