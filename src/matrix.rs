

//= Imports


//= Structure and Enumerations

/// Matrix type (OpenGL style 4x4 - right handed, column major)
#[derive(Debug, PartialEq)]
pub struct Matrix {
	pub m0: f32, pub m4: f32, pub  m8: f32, pub m12: f32,
	pub m1: f32, pub m5: f32, pub  m9: f32, pub m13: f32,
	pub m2: f32, pub m6: f32, pub m10: f32, pub m14: f32,
	pub m3: f32, pub m7: f32, pub m11: f32, pub m15: f32,
}


//= Procedures

impl Matrix {
	
	//= Conversion
	/// Converting to FFI version
	pub fn into_ffi(&self) -> raylib_ffi::Matrix {
		raylib_ffi::Matrix {
			m0: self.m0, m4: self.m4,  m8: self.m8,  m12: self.m12,
			m1: self.m1, m5: self.m5,  m9: self.m9,  m13: self.m13,
			m2: self.m2, m6: self.m6, m10: self.m10, m14: self.m14,
			m3: self.m3, m7: self.m7, m11: self.m11, m15: self.m15,
		}
	}
	/// Converting from FFI version
	pub fn from_ffi(value: raylib_ffi::Matrix) -> Self {
		Self {
			m0: value.m0, m4: value.m4,  m8: value.m8,  m12: value.m12,
			m1: value.m1, m5: value.m5,  m9: value.m9,  m13: value.m13,
			m2: value.m2, m6: value.m6, m10: value.m10, m14: value.m14,
			m3: value.m3, m7: value.m7, m11: value.m11, m15: value.m15,
		}
	}

}