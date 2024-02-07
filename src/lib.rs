

//= Imports
//use glfw::{fail_on_errors, Context, Glfw, GlfwReceiver, PWindow, WindowEvent, WindowHint};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};


//= Structs and Enums

//pub struct Navia {
//	pub glfw_context: Glfw,
//	pub glfw_window: PWindow,
//	pub glfw_events: GlfwReceiver<(f64, WindowEvent)>,
//}
pub struct Navia {

}


//= Procedures

impl Navia {
	pub fn init() -> Self {
		let sdl_context = sdl2::init().unwrap();
		let video_subsystem = sdl_context.video().unwrap();

		let window = video_subsystem.window("Navia", 1280, 720)
			.position_centered()
			.build()
			.unwrap();

		let mut canvas = window.into_canvas().build().unwrap();

		canvas.set_draw_color(Color::RGB(0,255,255));
		canvas.clear();
		canvas.present();
		let mut event_pump = sdl_context.event_pump().unwrap();
		let mut i = 0;
		'running: loop {
			i = (i + 1) % 255;
			canvas.set_draw_color(Color::RGB(0,64,255 - i));
			canvas.clear();
			for event in event_pump.poll_iter() {
				match event {
					Event::Quit {..} => {
						break 'running
					}
					Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
						break 'running
					}
					_ => {}
				}
			}
			canvas.present();
		}

		Self {}
	}
}

//impl Navia {
//	pub fn init(width: u32, height: u32, flags: u32) -> Self {
//
//		//* Create context */
//		let mut context = glfw::init(fail_on_errors!()).unwrap();
//
//		//* Window hints */
//		context.window_hint(WindowHint::Resizable(false));
//		context.window_hint(WindowHint::ContextVersion(3, 3));
//		
//		//* Create window */
//		let (mut window, events) = context.create_window(width, height, "Navia", glfw::WindowMode::Windowed)
//			.expect("Failed to create GLFW window");
//
//		window.make_current();
//		window.set_key_polling(true);
//
//
//		Self {
//			glfw_context: context,
//			glfw_window:  window,
//			glfw_events:  events,
//		}
//	}
//}