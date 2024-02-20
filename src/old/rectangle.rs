

//= Imports


//= Structure and Enumerations

//
pub struct Rectangle {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}


//= Procedures

impl Rectangle {
	
	//= Conversion
	/// Converting to FFI version
	pub fn into_ffi(&self) -> raylib_ffi::Rectangle {
		raylib_ffi::Rectangle {
			x: self.x,
			y: self.y,
			width: self.width,
			height: self.height,
		}
	}
	/// Converting from FFI version
	pub fn from_ffi(value: raylib_ffi::Rectangle) -> Self {
		Self {
			x: value.x,
			y: value.y,
			width: value.width,
			height: value.height,
		}
	}

}