

//= Imports


//= Structure and Enumerations

/// Rectangle
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}
impl Into<raylib_ffi::Rectangle> for Rectangle {
	fn into(self) -> raylib_ffi::Rectangle {
		raylib_ffi::Rectangle {
			x: self.x,
			y: self.y,
			width: self.width,
			height: self.height,
		}
	}
}
impl From<raylib_ffi::Rectangle> for Rectangle {
	fn from(value: raylib_ffi::Rectangle) -> Self {
		Self {
			x: value.x,
			y: value.y,
			width: value.width,
			height: value.height,
		}
	}
}


//= Constants


//= Implementations

impl Rectangle {

	//


}


//= Procedures
