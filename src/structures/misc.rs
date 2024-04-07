

use crate::structures::{
	color::*,
	image::*,
};


//= General
/// Wrapper for WindowShouldClose
pub fn should_window_close() -> bool {
	unsafe { WindowShouldClose() }
}
/// Wrapper for SetWindowIcon
pub fn set_icon(image: Image) {
	unsafe { SetWindowIcon(image.0) }
}
/// Wrapper for SetWindowIcons
pub fn set_icons(images: Vec<Image>) {
	unsafe {
		let mut array: Vec<ImageRl> = Vec::new();
		for i in 0..images.len() { array.push(images[i].0) }

		SetWindowIcons(array.as_mut_ptr(), images.len() as i32);
	}
}

//= Drawing
/// Wrapper for ClearBackground
pub fn clear_background(color: Color) {
	unsafe { ClearBackground(color) }
}


extern "C" { fn WindowShouldClose() -> bool; }
extern "C" { fn SetWindowIcon(image: ImageRl); }
extern "C" { fn SetWindowIcons(images: *mut ImageRl, count: i32); }
extern "C" { fn ClearBackground(color: Color); }