

//= Imports


//= Structure and Enumerations

//
pub struct Font(pub raylib_ffi::Font);
impl Into<raylib_ffi::Font> for Font {
	fn into(self) -> raylib_ffi::Font {
		self.0
	}
}
impl From<raylib_ffi::Font> for Font {
	fn from(value: raylib_ffi::Font) -> Self {
		Self { 0: value }
	}
}


//= Constants


//= Implementations


//= Procedures
