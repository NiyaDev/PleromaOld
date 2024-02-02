


use std::time::Instant;

fn main() {
	let mut now = Instant::now();

	navia::window::init_window(1280, 720, "TEST");

	let mut elapsed = now.elapsed();
	println!("\x1b[91mTime\x1b[0m: {:.2?}",elapsed);

	now = Instant::now();

	unsafe{raylib_ffi::InitWindow(1280, 720, raylib_ffi::rl_str!("TEST"));}

	elapsed = now.elapsed();
	println!("\x1b[91mTime\x1b[0m: {:.2?}",elapsed);
}
