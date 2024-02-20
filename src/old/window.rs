

//= Imports
use super::color::Color;


//= Structure and Enumerations


//= Procedures

/// Wrapper for InitWindow
pub fn init_window(width: i32, height: i32, title: &str) {
	unsafe { raylib_ffi::InitWindow(width, height, raylib_ffi::rl_str!(title)) }
}

/// Wrapper for CloseWindow
pub fn close_window() {
	unsafe { raylib_ffi::CloseWindow() }
}

/// Wrapper for WindowShouldClose
pub fn window_should_close() -> bool {
	unsafe { return raylib_ffi::WindowShouldClose() }
}

/// Wrapper for BeginDrawing
pub fn begin_drawing() {
	unsafe { raylib_ffi::BeginDrawing() }
}

/// Wrapper for EndDrawing
pub fn end_drawing() {
	unsafe { raylib_ffi::EndDrawing() }
}

/// Wrapper for 
pub fn clear_background(color: Color) {
	unsafe { raylib_ffi::ClearBackground(color.into_ffi()) }
}