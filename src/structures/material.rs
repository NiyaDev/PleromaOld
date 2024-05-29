
use crate::structures::{shader::*, texture::*, color::*};


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Material {
	pub shader:	ShaderRl,
	pub maps:	*mut MaterialMap,
	pub params:	[f32; 4],
}
impl Default for Material {
	fn default() -> Self {
		unsafe{ LoadMaterialDefault() }
	}
}

impl Material {
	
	
	
}


//= Material loading/unloading functions
extern "C" { fn LoadMaterialDefault() -> Material; }


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MaterialMap {
	pub texture:	TextureRl,
	pub color:		Color,
	pub value:		f32,
}