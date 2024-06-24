

//use std::collections::HashMap;

use pleroma::{
	camera::CameraMode, debug::*, files, font::*, g_3d::models::*, keybinds::{
		keyboard::KeyboardKey,
		Device,
	}, pleroma::*, vectors::*
};



fn main() {
	let mut pleroma = Pleroma::default();
	pleroma
		.set_log_level(LogLevel::Info)
		.set_title("Pleroma Test")
		.set_font(Font::load_ex("data/Pixeboy.ttf", 8, Vec::new()))
		.add_keybind("left",	Device::Keyboard, [0,KeyboardKey::A.into()])
		.add_keybind("right",	Device::Keyboard, [0,KeyboardKey::D.into()])
		.add_keybind("up",	Device::Keyboard, [0,KeyboardKey::W.into()])
		.add_keybind("down",	Device::Keyboard, [0,KeyboardKey::S.into()])
		.add_keybind("rot_l", Device::Keyboard, [0,KeyboardKey::Q.into()])
		.add_keybind("rot_r", Device::Keyboard, [0,KeyboardKey::E.into()])
		.add_keybind("anim",	Device::Keyboard, [0,KeyboardKey::R.into()])
		.camera.mode(CameraMode::Mode3D);
	
	let _bulk = files::Bulk::load("data/test.blk");
	
	
	pleroma.audio
		.load_song("data/sounds/new_bark_town.wav")
		.load_sfx("data/sounds/ugh.wav", "ugh");
	
	let mut model = Model::load("data/glTF/robot.glb");
	model.set_animation("Robot_Idle");
	
	while !pleroma.should_close() {
		if pleroma.is_down("up")		{ pleroma.camera.pan(Vector3{x:  0.0, y:  0.2, z: 0.0}); }
		if pleroma.is_down("down")	{ pleroma.camera.pan(Vector3{x:  0.0, y: -0.2, z: 0.0}); }
		if pleroma.is_down("rot_l")	{ pleroma.camera.rotate(-5.0); }
		if pleroma.is_down("rot_r")	{ pleroma.camera.rotate( 5.0); }
		if pleroma.is_down("anim")	{ model.set_animation("Robot_Walking"); }
		
		if pleroma.is_down("left")	{ model.rotation_angle += 10.0; }
		if pleroma.is_down("right")	{ model.rotation_angle -= 10.0; }
		
		pleroma.draw( |_ctx| {
			//texture.draw(Vector2{x:100.0,y:10.0});
			model.draw();
		});
	}
	
	pleroma.close(); // TODO: Unload everything
}
