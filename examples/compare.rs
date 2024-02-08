

use std::time::Instant;

//use glfw::{Action, Context, Key};
use navia;


fn main() {

	if false {
		let now = Instant::now();

		navia::window::init_window(1280, 720, "NAVIA");

		println!("\x1b[91mNavia Time\x1b[0m: {:.2?}",now.elapsed());

		//while ! {
		//	
		//}
	}
	
	if true {
		let mut now = Instant::now();

		unsafe{raylib_ffi::InitWindow(1280, 720, raylib_ffi::rl_str!("Raylib"));}

		println!("\x1b[91mRaylib Time\x1b[0m: {:.2?}", now.elapsed());

		unsafe {
			while !raylib_ffi::WindowShouldClose() {
				now = Instant::now();
				raylib_ffi::BeginDrawing();
				raylib_ffi::ClearBackground(raylib_ffi::colors::BLACK);
				raylib_ffi::EndDrawing();
			}
			println!("\x1b[91mFrame Time\x1b[0m: {:.2?}", now.elapsed());
		}

	}
}
