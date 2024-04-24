

//use std::time::Instant;

use pleroma::{
	debug::*,
	keybindings::{keyboard::*, *},
	pleroma::*,
	structures::{
		color::*, font::Font, image::Image, misc::*, vectors::*
	}
};


fn main() {
	let mut pleroma: Pleroma = Pleroma::new();
	pleroma.screen
		.init("Pleroma Testing")
		.set_resolution(800, 600)
		.set_render_scale(0.5);
	pleroma.fonts.insert("default".to_string(), Font::default());

	pleroma.keys
		.insert("normal", vec![Binding::KeyboardKey(KeyboardKey::A)])
		.insert(
			"mod",
			vec![
				Binding::KeyboardKey(KeyboardKey::LeftShift),
				Binding::KeyboardKey(KeyboardKey::S),
			],
		);
	pleroma.log(Error::TestError);

	pleroma.textures.insert(
		"gradient".to_string(),
		Image::gen_linear_gradient(64, 64, 1,BLACK, DARKPURPLE).texture(),
	);
	pleroma.textures.insert(
		"perlin".to_string(),
		Image::gen_perlin_noise(64, 64, 0, 0, 1.0).texture(),
	);

	while !should_window_close() {

		if pleroma.keys.key_pressed("normal") {
			pleroma.log(Error::TestError);
		}
		if pleroma.keys.key_pressed("mod") { println!("Mod down") }

		pleroma.screen.start_draw();
		pleroma.textures.get("perlin").unwrap().draw(10, 10);
		pleroma.fonts.get("default").unwrap().draw("Here you go fuck face", Vector2{x: 0.0, y: 20.0}, 20.0, 1.0, BLACK);
		pleroma.screen.end_draw();
	}
}