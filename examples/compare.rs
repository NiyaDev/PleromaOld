

use pleroma::{
	debug::{errors::PlError, *},
	keybinds::Device,
	pleroma::*,
	structures::{
		color::*, font::*, image::*
	}
};



fn main() {
	let mut pleroma = Pleroma::default();
	pleroma
		.set_log_level(LogLevel::Info)
		.set_title("Pleroma Test")
		.set_font(Font::load_ex("data/Pixeboy.ttf", 8, Vec::new()))
		.add_keybind("A", Device::Keyboard, [0,65])
		.add_keybind("gp1-a", Device::Gamepad(0), [0,6]);
	
	//pleroma.audio.load_track("data/doom.json", 0);
	pleroma.audio.load_song("data/sounds/new_bark_town.wav");
	
	let texture = Image::gen_linear_gradient(64, 64, 1,BLACK, DARKPURPLE).texture();
	
	while !pleroma.should_close() {
		//if pleroma.keys.key_pressed("normal") {
		//	log(Error::TestError);
		//}
		//unsafe {if IsKeyPressed(65) { pleroma.log(PlError::Default) }}
		//unsafe {if IsKeyPressed(66) { pleroma.log(PlError::TestError) }}
		//unsafe {if IsKeyPressed(67) { pleroma.log(PlError::TestCitical) }}
		//if pleroma.keys.key_pressed("mod") { println!("Mod down") }
		if pleroma.is_pressed("A") { pleroma.log(PlError::Default) }
		pleroma.audio.update();
		
		pleroma.start_draw();
		texture.draw(0, 0);
		pleroma.end_draw();
	}
	
	pleroma.close();
}



//use std::time::Instant;

//use pleroma::{
//	debug::*,
//	keybindings::{keyboard::*, *},
//	pleroma::*,
//	structures::{
//		color::*,
//		font::Font,
//		image::Image,
//		misc::*,
//	}
//};


//fn main() {
//	let mut pleroma: Pleroma = Pleroma::new();
//	pleroma.screen
//		.init("Pleroma Testing")
//		.set_resolution(800, 600)
//		.set_render_scale(0.5);
//	pleroma.fonts.insert("default".to_string(), Font::load_ex("data/EarlyGameBoy.ttf", 8, Vec::new()));
//	pleroma.screen.def_font = Font::load_ex("data/EarlyGameBoy.ttf", 8, Vec::new());
//
//	pleroma.keys
//		.insert("normal", vec![Binding::KeyboardKey(KeyboardKey::A)])
//		.insert(
//			"mod",
//			vec![
//				Binding::KeyboardKey(KeyboardKey::LeftShift),
//				Binding::KeyboardKey(KeyboardKey::S),
//			],
//		);
//	Pleroma::debug_mode();
//
//	pleroma.textures.insert(
//		"gradient".to_string(),
//		Image::gen_linear_gradient(64, 64, 1,BLACK, DARKPURPLE).texture(),
//	);
//	pleroma.textures.insert(
//		"perlin".to_string(),
//		Image::gen_perlin_noise(64, 64, 0, 0, 1.0).texture(),
//	);
//
//	while !should_window_close() {
//
//		if pleroma.keys.key_pressed("normal") {
//			log(Error::TestError);
//		}
//		if pleroma.keys.key_pressed("mod") { println!("Mod down") }
//
//		pleroma.screen.start_draw();
//		pleroma.textures.get("perlin").unwrap().draw(10, 10);
//		pleroma.screen.end_draw();
//	}
//}