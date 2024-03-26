

//use std::time::Instant;

use navia::{color, image::Image, keybindings::{keyboard::*, *}, misc::*, pleroma::Pleroma, screen::Screen};


fn main() {
	let mut pleroma: Pleroma = Pleroma::new();
	pleroma.screen
		.init("Testing")
		.set_resolution(800, 600)
		.set_render_scale(0.5);

	pleroma.keys
		.insert("normal", vec![Binding::KeyboardKey(KeyboardKey::A)])
		.insert(
			"mod",
			vec![
				Binding::KeyboardKey(KeyboardKey::LeftShift),
				Binding::KeyboardKey(KeyboardKey::S),
			],
		);


	let texture = Image::gen_linear_gradient(64, 64, 1, color::BLACK, color::DARKPURPLE).texture();

	while !should_window_close() {

		//if keybindings.key_pressed("up").ok().unwrap() { screen.toggle_fullscreen() }
		if pleroma.keys.key_pressed("normal") { println!("Normal down") }
		if pleroma.keys.key_pressed("mod") { println!("Mod down") }
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

		pleroma.screen.start_draw();
		texture.draw(10, 10, color::WHITE);
		pleroma.screen.end_draw();
	}
}