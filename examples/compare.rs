

//use std::time::Instant;

//use navia::{self, window::init_window};

use navia::{color, image::Image, misc::*, screen::Screen};


fn main() {
	let mut screen = Screen::new();

	screen.init("Testing");
	screen.set_render_scale(0.5);

	let texture = Image::gen_linear_gradient(32, 32, 1, color::BLACK, color::DARKPURPLE).texture();

	while !should_window_close() {

		screen.start_draw();
		clear_background(color::BLUE);
		texture.draw(10, 10, color::WHITE);
		screen.end_draw();
	}
}