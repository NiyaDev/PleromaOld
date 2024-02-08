

//= Imports
use glfw;
use current_platform::CURRENT_PLATFORM;

use crate::{flags::WindowFlags::*, logging::TraceLogLevel, *};
use super::CONTEXT;


//= Constants


//= Structures & Enumerations


//= Procedures

/// GLFW3 Error Callback, runs on GLFW3 error
// TODO: Make sure all this works
pub fn error_callback(error: glfw::Error, description: String) {
	unsafe {tracelog!(TraceLogLevel::LogWarning, "GLFW: Error: {} Description: {}", error, description);}
}

/// GLFW3 WindowSize Callback, runs when window is resizedLastFrame
// NOTE: Window resizing not allowed by default
// TODO: Make sure all this works
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
// TODO: Make sure all this works
pub fn window_content_scale_callback(_: &mut glfw::Window, scalex: f32, scaley: f32) {
	unsafe { CORE.window.screen_scale = Matrix::scale(scalex, scaley, 1.0); }
}

/// GLFW3 WindowIconify Callback, runs when window is minimized/restored
// TODO: Make sure all this works
pub fn window_iconify_callback(_: &mut glfw::Window, iconify: bool) {
	unsafe {
		if iconify { CORE.window.flags.set(Minimized.into()) }
		else { CORE.window.flags.clear(Minimized.into()) }
	}
}

/// GLFW3 WindowMaximize Callback, runs when window is maximized/restored
// TODO: Make sure all this works
pub fn window_maximize_callback(_: &mut glfw::Window, maximized: bool) {
	unsafe {
		if maximized { CORE.window.flags.set(Maximized.into()) }
		else { CORE.window.flags.clear(Maximized.into()) }
	}
}

/// GLFW3 WindowFocus Callback, runs when window get/lose focus
// TODO: Make sure all this works
pub fn window_focus_callback(_: &mut glfw::Window, focused: bool) {
	unsafe {
		if focused { CORE.window.flags.clear(Unfocused.into()) }
		else { CORE.window.flags.set(Unfocused.into()) }
	}
}

/// GLFW3 Window Drop Callback, runs when drop files into window
// TODO: Make sure all this works
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
// TODO: Make sure all this works
pub fn key_callback(window: &mut glfw::Window, key: glfw::Key, _: i32, action: glfw::Action, modifiers: glfw::Modifiers) {
	unsafe {
		//if key < 0 { return }
		println!("{:?}",key);

		// WARNING: GLFW could return GLFW_REPEAT, we need to consider it as 1
		// WARNING: to work properly with our implementation (IsKeyDown/IsKeyUp checks)
		if action == glfw::Action::Release { CORE.input.keyboard.current_key_state[key as usize] = Action::Release }
		else if action == glfw::Action::Press { CORE.input.keyboard.current_key_state[key as usize] = Action::Press }
		else if action == glfw::Action::Repeat { CORE.input.keyboard.key_repeat_in_frame[key as usize] = 1 }

		// WARNING: Check if CAPS/NUM key modifiers are enabled and force down state for those keys
		if (key == glfw::Key::CapsLock && modifiers.contains(glfw::Modifiers::CapsLock)) ||
				(key == glfw::Key::NumLock && modifiers.contains(glfw::Modifiers::NumLock)) {
			CORE.input.keyboard.current_key_state[key as usize] = Action::Press
		}

		//* Check if there is space available in the key queue */
		if CORE.input.keyboard.key_press_queue_count < MAX_KEY_PRESSED_QUEUE as i32 && action == glfw::Action::Press {
			//* Add character to the queue */
			CORE.input.keyboard.key_press_queue[CORE.input.keyboard.key_press_queue_count as usize] = key as i32;
			CORE.input.keyboard.key_press_queue_count += 1;
		}

		//* Check the exit key to set close window */
		if CORE.input.keyboard.exit_key == key.into() && action == glfw::Action::Press { window.set_should_close(true) }
	}
}

/// GLFW3 Char Callback, get unicode codepoint value
// TODO: Make sure all this works
pub fn char_callback(_: &mut glfw::Window, codepoint: char) {
	unsafe {
		// NOTE: Registers any key down considering OS keyboard layout but
		// NOTE: does not detect action events, those should be managed by user...

		//* Check if there is space available in the queue */
		if CORE.input.keyboard.char_press_queue_count < MAX_CHAR_PRESSED_QUEUE as i32 {
			//* Add character to the queue */
			CORE.input.keyboard.char_press_queue[CORE.input.keyboard.char_press_queue_count as usize] = codepoint as i32;
			CORE.input.keyboard.char_press_queue_count += 1;
		}
	}
}

/// GLFW3 Mouse Button Callback, runs on mouse button pressed
// TODO: Make sure all this works
pub fn mouse_button_callback(_: &mut glfw::Window, button: glfw::MouseButton, action: glfw::Action, _: glfw::Modifiers) {
	unsafe {
		// WARNING: GLFW could only return GLFW_PRESS (1) or GLFW_RELEASE (0) for now,
		// WARNING: but future releases may add more actions (i.e. GLFW_REPEAT)
		CORE.input.mouse.current_button_state[button as usize] = action as i8;
	}
}

/// GLFW3 Cursor Position Callback, runs on mouse move
// TODO: Make sure all this works
pub fn mouse_cursor_position_callback(_: &mut glfw::Window, x: f64, y: f64) {
	unsafe {
		CORE.input.mouse.current_position = Vector2{x: x as f32, y: y as f32};
	}
}

/// GLFW3 Scrolling Callback, runs on mouse wheel
// TODO: Make sure all this works
pub fn mouse_scroll_callback(_: &mut glfw::Window, xoffset: f64, yoffset: f64) {
	unsafe {
		CORE.input.mouse.current_wheel_move = Vector2{x: xoffset as f32, y: yoffset as f32};
	}
}

/// GLFW3 CursorEnter Callback, when cursor enters the window
// TODO: Make sure all this works
pub fn mouse_cursor_enter_callback(_: &mut glfw::Window, enter: bool) {
	unsafe {
		if enter { CORE.input.mouse.cursor_on_screen = true }
		else { CORE.input.mouse.cursor_on_screen = false }
	}
}

/// GLFW3 Joystick Connected/Disconnected Callback
// TODO: Make sure all this works
pub fn joystick_callback(joystick_id: glfw::JoystickId, event: glfw::JoystickEvent) {
	unsafe {
		if event == glfw::JoystickEvent::Connected {
			let name_str = CONTEXT.as_mut().unwrap().get_joystick(joystick_id).get_gamepad_name().unwrap();
			let name = name_str.as_bytes();
			for i in 0..name.len() {
				if i >= 64 {break}
				CORE.input.gamepad.name[i][joystick_id as usize] = name[i];
			}
		} else if event == glfw::JoystickEvent::Disconnected {
			for i in 0..64 {
				CORE.input.gamepad.name[i][joystick_id as usize] = 0;
			}
		}
	}
}