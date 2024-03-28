

//use std::time::Instant;

use pleroma::{color, image::Image, keybindings::{keyboard::*, *}, misc::*, pleroma::*};


fn main() {
	let mut pleroma: Pleroma = Pleroma::new();
	pleroma.screen
		.init("Pleroma Testing")
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

	pleroma.textures.insert(
		"gradient".to_string(),
		Image::gen_linear_gradient(64, 64, 1, color::BLACK, color::DARKPURPLE).texture(),
	);
	pleroma.textures.insert(
		"perlin".to_string(),
		Image::gen_perlin_noise(64, 64, 0, 0, 1.0).texture(),
	);

	while !should_window_close() {

		//if keybindings.key_pressed("up").ok().unwrap() { screen.toggle_fullscreen() }
		if pleroma.keys.key_pressed("normal") {
			pleroma.screen.set_resolution(1000, 800);
		}
		if pleroma.keys.key_pressed("mod") { println!("Mod down") }

		pleroma.screen.start_draw();
		pleroma.textures.get("perlin").unwrap().draw(10, 10, color::WHITE);
		pleroma.screen.end_draw();
	}
}