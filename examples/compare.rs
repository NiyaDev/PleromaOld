

use pleroma::{
	camera::CameraMode, debug::*, keybinds::{keyboard::KeyboardKey, Device}, pleroma::*, structures::{
		color::*, font::*, image::*, material::*, matrix::*, model::*, vectors::Vector3
	}
};



fn main() {
	let mut pleroma = Pleroma::default();
	pleroma
		.set_log_level(LogLevel::Info)
		.set_title("Pleroma Test")
		.set_font(Font::load_ex("data/Pixeboy.ttf", 8, Vec::new()))
		.add_keybind("left", Device::Keyboard, [0,KeyboardKey::A.into()])
		.add_keybind("right", Device::Keyboard, [0,KeyboardKey::D.into()])
		.add_keybind("up", Device::Keyboard, [0,KeyboardKey::W.into()])
		.add_keybind("down", Device::Keyboard, [0,KeyboardKey::S.into()]);
	pleroma.camera.camera_mode = CameraMode::Mode3D; // TODO: Change this
	
	pleroma.audio
		.load_song("data/sounds/new_bark_town.wav")
		.load_sfx("data/sounds/ugh.wav", "ugh");
	
	let texture = Image::gen_linear_gradient(64, 64, 1,BLACK, DARKPURPLE).texture();
	let mut mesh = Mesh::gen_cube(1.0, 1.0, 1.0);
	let mut transforms = Vec::new();
	transforms.push(IDENTITY);
	transforms.push(Matrix::translate(Vector3{x: 2.0, y: 0.0, z: 0.0}));
	transforms.push(Matrix::translate(Vector3{x: 0.0, y: 2.0, z: 0.0}));
	let trans = transforms.as_slice();
	
	while !pleroma.should_close() {
		if pleroma.is_pressed("left") { pleroma.camera.pan(Vector3{x: -1.0, y:  0.0, z: 0.0}); }
		if pleroma.is_pressed("right"){ pleroma.camera.pan(Vector3{x:  1.0, y:  0.0, z: 0.0}); }
		if pleroma.is_pressed("up")	{ pleroma.camera.pan(Vector3{x:  0.0, y:  1.0, z: 0.0}); }
		if pleroma.is_pressed("down") { pleroma.camera.pan(Vector3{x:  0.0, y: -1.0, z: 0.0}); }
		
		pleroma.draw( |_ctx| {
			texture.draw(100, 10);
			//mesh.draw(Material::default(), IDENTITY);
			mesh.draw_instanced(Material::default(), trans);
		});
	}
	
	pleroma.close(); // TODO: Unload everything
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