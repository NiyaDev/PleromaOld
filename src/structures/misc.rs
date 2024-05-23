

use crate::structures::image::*;


//= General
/// #### set_icon
/// Wrapper for SetWindowIcon
/// TODO Move to pleroma
pub fn set_icon(image: Image) {
	unsafe { SetWindowIcon(image.0) }
}
/// #### set_icons
/// Wrapper for SetWindowIcons
/// TODO Move to pleroma
pub fn set_icons(images: Vec<Image>) {
	unsafe {
		let mut array: Vec<ImageRl> = Vec::new();
		for i in 0..images.len() { array.push(images[i].0) }

		SetWindowIcons(array.as_mut_ptr(), images.len() as i32);
	}
}


extern "C" { fn SetWindowIcon(image: ImageRl); }
extern "C" { fn SetWindowIcons(images: *mut ImageRl, count: i32); }