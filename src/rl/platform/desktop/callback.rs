

//= Allows


//= Imports
use glfw;
use platforms;
use current_platform::CURRENT_PLATFORM;

use crate::{flags::WindowFlags::*, logging::TraceLogLevel, *};


//= Constants


//= Structures & Enumerations


//= Procedures

/// GLFW3 Error Callback, runs on GLFW3 error
pub fn error_callback(error: glfw::Error, description: String) {
	unsafe {tracelog!(TraceLogLevel::LogWarning, "GLFW: Error: {} Description: {}", error, description);}
}

/// GLFW3 WindowSize Callback, runs when window is resizedLastFrame
// NOTE: Window resizing not allowed by default
pub fn window_size_callback(_: &mut glfw::Window, width: i32, height: i32) {
	unsafe {
		//* Reset viewport and projection matrix for new size */
		// TODO: SetupViewport(width, height);

		CORE.window.current_fbo.width = width as u32;
		CORE.window.current_fbo.height = height as u32;
		CORE.window.resized_last_frame = true;

		if is_window_fullscreen() { return }

		//* Set current screen size */
		let is_mac = if CURRENT_PLATFORM.contains(&platforms::OS::MacOS.to_string()) { true } else { false };
		if is_mac {
			CORE.window.screen.width = width as u32;
			CORE.window.screen.height = height as u32;
		} else {
			if CORE.window.flags.contains(HighDPI.into()) {
				// TODO: let window_scale_dpi = GetWindowScaleDPI();
				let window_scale_dpi = Vector2{x: 1.0, y: 1.0};

				CORE.window.screen.width = (width / window_scale_dpi.x as i32) as u32;
				CORE.window.screen.height = (height / window_scale_dpi.y as i32) as u32;
			} else {
				CORE.window.screen.width = width as u32;
				CORE.window.screen.height = height as u32;
			}
		}
	}
	// NOTE: Postprocessing texture is not scaled to new size
}

/// GLFW3 ContentScall Callback
pub fn window_content_scale_callback(_: &mut glfw::Window, scalex: f32, scaley: f32) {
	unsafe { CORE.window.screen_scale = Matrix::scale(scalex, scaley, 1.0); }
}

/// GLFW3 WindowIconify Callback, runs when window is minimized/restored
pub fn window_iconify_callback(_: &mut glfw::Window, iconify: bool) {
	unsafe {
		if iconify { CORE.window.flags.set(Minimized.into()) }
		else { CORE.window.flags.clear(Minimized.into()) }
	}
}

/// GLFW3 WindowMaximize Callback, runs when window is maximized/restored
pub fn window_maximize_callback(_: &mut glfw::Window, maximized: bool) {
	unsafe {
		if maximized { CORE.window.flags.set(Maximized.into()) }
		else { CORE.window.flags.clear(Maximized.into()) }
	}
}

/// GLFW3 WindowFocus Callback, runs when window get/lose focus
pub fn window_focus_callback(_: &mut glfw::Window, focused: bool) {
	unsafe {
		if focused { CORE.window.flags.clear(Unfocused.into()) }
		else { CORE.window.flags.set(Unfocused.into()) }
	}
}

/// GLFW3 Window Drop Callback, runs when drop files into window
pub fn window_drop_callback(_: &mut glfw::Window, paths: Vec<std::path::PathBuf>) {
	unsafe {
		if paths.len() > 0 {
			//* In case previous dropped filepaths have not been freed, we free them */
			if CORE.window.dropped_file_count > 0 {
				for i in 0..CORE.window.dropped_file_count { CORE.window.dropped_filepaths[i as usize] = [0;64] }
            	CORE.window.dropped_file_count = 0;
			}

			// WARNING: Paths are freed by GLFW when the callback returns, we must keep an internal copy
			CORE.window.dropped_file_count = paths.len() as u32;
			for i in 0..paths.len() {
				let str = paths[i].to_str().unwrap().as_bytes();
				for l in 0..str.len() {
					if l >= 64 { break }
					CORE.window.dropped_filepaths[l][i] = str[l];
				}
			}
		}
	}
}

/// GLFW3 Keyboard Callback, runs on key pressed
pub fn key_callback(_: &mut glfw::Window, key: glfw::Key, scancode: i32, action: glfw::Action, modifiers: glfw::Modifiers) {
	unsafe {
		//if key < 0 { return }

		// WARNING: GLFW could return GLFW_REPEAT, we need to consider it as 1
		// WARNING: to work properly with our implementation (IsKeyDown/IsKeyUp checks)
		if action == glfw::Action::Release { CORE.input.keyboard.current_key_state[key as usize] = 0 }
		else if action == glfw::Action::Press { CORE.input.keyboard.current_key_state[key as usize] = 1 }
		else if action == glfw::Action::Repeat { CORE.input.keyboard.key_repeat_in_frame[key as usize] = 1 }

		// WARNING: Check if CAPS/NUM key modifiers are enabled and force down state for those keys
		if (key == glfw::Key::CapsLock && modifiers.contains(glfw::Modifiers::CapsLock)) ||
				(key == glfw::Key::NumLock && modifiers.contains(glfw::Modifiers::NumLock)) {
			CORE.input.keyboard.current_key_state[key as usize] = 1
		}

		//* Check if there is space available in the key queue */
		if CORE.input.keyboard.key_press_queue_count < MAX_KEY_PRESSED_QUEUE as i32 && action == glfw::Action::Press {
			//* Add character to the queue */
			CORE.input.keyboard.key_press_queue[CORE.input.keyboard.key_press_queue_count as usize] = key.into();
			CORE.input.keyboard.key_press_queue_count += 1;
		}
	}
}
