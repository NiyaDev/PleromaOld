

//= Imports
use super::{color::Color, texture::Texture};


//= Structure and Enumerations

//
pub struct MaterialMap {
	pub texture:	Texture,
    pub color:		Color,
    pub value:		f32,
}

//
#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub shader:		raylib_ffi::Shader,
    pub maps:		*mut raylib_ffi::MaterialMap,
    pub params:		[f32; 4],
}
impl PartialEq for Material {
	fn eq(&self, other: &Self) -> bool {
		//self.shader == other.shader &&
		self.maps == other.maps &&
			self.params == other.params
	}
}


//= Procedures

impl Material {
	
	//= Conversion
	/// Converting to FFI version
	pub fn into_ffi(&self) -> raylib_ffi::Material {
		raylib_ffi::Material {
			shader:	self.shader,
			maps:	self.maps,
			params:	self.params,
		}
	}
	/// Converting from FFI version
	pub fn from_ffi(value: raylib_ffi::Material) -> Self {
		Self {
			shader:	value.shader,
			maps:	value.maps,
			params:	value.params,
		}
	}

}