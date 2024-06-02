

use pleroma::{
	camera::CameraMode, debug::*, keybinds::{keyboard::KeyboardKey, Device}, pleroma::*, structures::{
		color::*, font::*, image::*, model::*, vectors::Vector3
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
		.add_keybind("down", Device::Keyboard, [0,KeyboardKey::S.into()])
		.add_keybind("rot_l", Device::Keyboard, [0,KeyboardKey::Q.into()])
		.add_keybind("rot_r", Device::Keyboard, [0,KeyboardKey::E.into()]);
	pleroma.camera.camera_mode = CameraMode::Mode3D; // TODO: Change this
	
	pleroma.audio
		.load_song("data/sounds/new_bark_town.wav")
		.load_sfx("data/sounds/ugh.wav", "ugh");
	
	let texture = Image::gen_linear_gradient(64, 64, 1,BLACK, DARKPURPLE).texture();
	let mut model = Model::load("data/test/map_06.obj");
	
	while !pleroma.should_close() {
		if pleroma.is_down("left") { pleroma.camera.pan(Vector3{x: -0.2, y:  0.0, z: 0.0}); }
		if pleroma.is_down("right"){ pleroma.camera.pan(Vector3{x:  0.2, y:  0.0, z: 0.0}); }
		if pleroma.is_down("up")	 { pleroma.camera.pan(Vector3{x:  0.0, y:  0.2, z: 0.0}); }
		if pleroma.is_down("down") { pleroma.camera.pan(Vector3{x:  0.0, y: -0.2, z: 0.0}); }
		if pleroma.is_down("rot_l") { pleroma.camera.rotate(-1.0); }
		if pleroma.is_down("rot_r") { pleroma.camera.rotate( 1.0); }
		
		pleroma.draw( |_ctx| {
			texture.draw(100, 10);
			model.draw(Vector3 { x: 0.0, y: 0.0, z: -100.0 }, 1.0, WHITE);
		});
	}
	
	pleroma.close(); // TODO: Unload everything
}
