

//= Imports


//= Structure and Enumerations

/// Color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}


//= Procedures

impl Color {

	//= Generation
	/// `(0,0,0,255)`
	pub fn black() -> Self {
		Self {
			r: 0,
			g: 0,
			b: 0,
			a: 255,
		}
	}
	/// `(255,255,255,255)`
	pub fn white() -> Self {
		Self {
			r: 255,
			g: 255,
			b: 255,
			a: 255,
		}
	}
	/// `(48,56,67,255)`
	pub fn palette_30() -> Self {
		Self {
			r: 48,
			g: 56,
			b: 67,
			a: 255,
		}
	}

	//= Conversion
	/// Converting to FFI version
	pub fn into_ffi(&self) -> raylib_ffi::Color {
		raylib_ffi::Color {
			r: self.r,
			g: self.g,
			b: self.b,
			a: self.a,
		}
	}
	/// Converting from FFI version
	pub fn from_ffi(value: raylib_ffi::Color) -> Self {
		Self {
			r: value.r,
			g: value.g,
			b: value.b,
			a: value.a,
		}
	}

}