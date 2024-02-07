

use std::time::Instant;

//use glfw::{Action, Context, Key};
use navia::Navia;


fn main() {

	let now = Instant::now();

	//let mut context = glfw::init(fail_on_errors!()).unwrap();
	//context.window_hint(WindowHint::Resizable(false));
	//let (mut window, events) = context.create_window(1280, 720, "Navia", glfw::WindowMode::Windowed)
	//	.expect("Failed to create GLFW window");
	//window.make_current();
	//window.set_key_polling(true);
	//let mut navia = Navia::init(1280, 720, 0);
//
	//while !navia.glfw_window.should_close() {
	//	navia.glfw_window.
	//	navia.glfw_window.swap_buffers();
//
	//	navia.glfw_context.poll_events();
	//	for (_, event) in glfw::flush_messages(&navia.glfw_events) {
	//		println!("{:?}", event);
	//		match event {
	//			glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
	//				navia.glfw_window.set_should_close(true);
	//			}
	//			_ => {}
	//		}
	//	}
	//}
	let _navia = Navia::init();


	let elapsed = now.elapsed();
	println!("\x1b[91mNavia Time\x1b[0m: {:.2?}",elapsed);


	//{
	//	let now = Instant::now();
//
	//	navia::window::init_window(1280, 720, "NAVIA");
//
	//	let elapsed = now.elapsed();
	//	println!("\x1b[91mNavia Time\x1b[0m: {:.2?}",elapsed);
	//}
	//
	//{
	//	let now = Instant::now();
//
	//	//unsafe{raylib_ffi::InitWindow(1280, 720, raylib_ffi::rl_str!("RAYLIB"));}
//
	//	let elapsed = now.elapsed();
	//	println!("\x1b[91mRaylib Time\x1b[0m: {:.2?}",elapsed);
	//}
//
	//loop {
	//	
	//}
}
