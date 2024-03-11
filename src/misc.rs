

//= Imports
use crate::{color::*, image::*};


//= Procedures

//= General
/// Wrapper for WindowShouldClose
pub fn should_window_close() -> bool {
	unsafe { raylib_ffi::WindowShouldClose() }
}
/// Wrapper for SetWindowIcon
pub fn set_icon(image: Image) {
	unsafe { raylib_ffi::SetWindowIcon(image.into()) }
}
/// Wrapper for SetWindowIcons
pub fn set_icons(images: Vec<Image>) {
	unsafe {
		let mut array: Vec<raylib_ffi::Image> = Vec::new();
		for i in 0..images.len() { array.push(images[i].0) }

		raylib_ffi::SetWindowIcons(array.as_mut_ptr(), images.len() as i32);
	}
}

//= Drawing
/// Wrapper for ClearBackground
pub fn clear_background(color: Color) {
	unsafe { raylib_ffi::ClearBackground(color.into()) }
}