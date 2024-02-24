

//= Imports


//= Structure and Enumerations

//
pub struct Texture(pub raylib_ffi::Texture);
impl Into<raylib_ffi::Texture> for Texture {
	fn into(self) -> raylib_ffi::Texture {
		self.0
	}
}
impl From<raylib_ffi::Texture> for Texture {
	fn from(value: raylib_ffi::Texture) -> Self {
		Self { 0: value }
	}
}


//= Constants


//= Implementations


//= Procedures
