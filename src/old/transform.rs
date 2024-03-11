

//= Imports
use super::vector::*;


//= Structure and Enumerations

//
pub struct Transform {
	pub translation:	Vector3,
    pub rotation:		Vector4,
    pub scale:			Vector3,
}


//= Procedures

impl Transform {
	
	//= Conversion
	/// Converting to FFI version
	pub fn into_ffi(&self) -> raylib_ffi::Transform {
		raylib_ffi::Transform {
			translation:	self.translation.into_ffi(),
			rotation:		self.rotation.into_ffi(),
			scale:			self.scale.into_ffi(),
		}
	}
	/// Converting from FFI version
	pub fn from_ffi(value: raylib_ffi::Transform) -> Self {
		Self {
			translation:	Vector3::from_ffi(value.translation),
			rotation:		Vector4::from_ffi(value.rotation),
			scale:			Vector3::from_ffi(value.scale),
		}
	}

}