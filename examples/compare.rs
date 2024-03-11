

//use std::time::Instant;

use navia::{color, image::Image, keybindings::{*, keyboard_keys::*}, misc::*, screen::Screen};


fn main() {
	let mut screen = Screen::new();
	screen.set_resolution(800, 600);
	screen.init("Testing");
	screen.set_render_scale(0.5);

	let mut keybindings = Keybindings::new();
	// TODO cleaner adding
	keybindings.insert("up".to_string(), Keybind::Keyboard { key: KeyboardKey::Up });


	let texture = Image::gen_linear_gradient(64, 64, 1, color::BLACK, color::DARKPURPLE).texture();

	while !should_window_close() {

		if keybindings.key_pressed("up".to_string()).ok().unwrap() { screen.toggle_fullscreen() }
		/*
		unsafe {
			if raylib_ffi::IsKeyPressed(raylib_ffi::enums::KeyboardKey::F as i32) {
				screen.toggle_fullscreen();
			}
			if raylib_ffi::IsKeyPressed(raylib_ffi::enums::KeyboardKey::B as i32) {
				screen.toggle_borderless();
			}
		}
		*/

		screen.start_draw();
		texture.draw(10, 10, color::WHITE);
		screen.end_draw();
	}
}