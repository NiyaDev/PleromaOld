

//use std::time::Instant;

use navia::{color, image::Image, keybindings2::{keyboard::*, *}, misc::*, screen::Screen};


fn main() {
	let mut screen = Screen::new();
	screen.set_resolution(800, 600);
	screen.init("Testing");
	screen.set_render_scale(0.5);

	//let mut keybindings = Keybindings::new();
	// TODO cleaner adding
	//keybindings.insert("up", Keybind::Keyboard { key: KeyboardKey::Up });
	let mut keybindings = Keybindings::new();
	keybindings.insert("normal", vec![Binding::KeyboardKey(KeyboardKey::A)]);
	keybindings.insert(
		"mod",
		vec![
			Binding::KeyboardKey(KeyboardKey::LeftShift),
			Binding::KeyboardKey(KeyboardKey::S),
		],
	);


	let texture = Image::gen_linear_gradient(64, 64, 1, color::BLACK, color::DARKPURPLE).texture();

	while !should_window_close() {

		//if keybindings.key_pressed("up").ok().unwrap() { screen.toggle_fullscreen() }
		if keybindings.key_pressed("normal") { println!("Normal down") }
		if keybindings.key_pressed("mod") { println!("Mod down") }
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