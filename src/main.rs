


use std::time::Instant;

fn main() {
	{
		let now = Instant::now();

		navia::window::init_window(1280, 720, "NAVIA");

		let elapsed = now.elapsed();
		println!("\x1b[91mNavia Time\x1b[0m: {:.2?}",elapsed);
	}
	
	{
		let now = Instant::now();

		//unsafe{raylib_ffi::InitWindow(1280, 720, raylib_ffi::rl_str!("RAYLIB"));}

		let elapsed = now.elapsed();
		println!("\x1b[91mRaylib Time\x1b[0m: {:.2?}",elapsed);
	}

	loop {
		
	}
}
