

use super::LogLevel;


/// Types of errors that can be a result of systems and functions.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlError {
	Default,
	
	EditingRenderSize,
	RenderTextureDoesntExist,
}
impl Into<u16> for PlError {
	fn into(self) -> u16 {
		match self {
			PlError::Default => 0,
			PlError::EditingRenderSize => 1,
			PlError::RenderTextureDoesntExist => 2,
		}
	}
}
impl Into<LogLevel> for PlError {
	fn into(self) -> LogLevel {
		match self {
			PlError::Default => LogLevel::Info,
			PlError::EditingRenderSize => LogLevel::Error,
			PlError::RenderTextureDoesntExist => LogLevel::Error,
		}
	}
}
impl ToString for PlError {
	fn to_string(&self) -> String {
		match self {
			PlError::Default => "[INFO] - Default error test.".to_string(),
			PlError::EditingRenderSize => "[ERROR] - Attempted to edit render resolution while drawing to screen.".to_string(),
			PlError::RenderTextureDoesntExist => "[ERROR] - Attempted to start drawing without a RenderTexture.".to_string(),
		}
	}
}