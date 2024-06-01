
use crate::{rl_str, structures::{color::*, shader::*, texture::*}};


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
	
	/// #### load_multiple
	/// Wrapper for Raylib::LoadMaterials(filename: *const i8, materialCount: *const i32).
	pub fn load_multiple(filename: &str) -> Vec<Material> {
		unsafe{
			let mut count = [0];
			let mats = LoadMaterials(rl_str!(filename), count.as_mut_ptr());
			
			let mut result = Vec::new();
			for i in 0..count[0] as usize {
				result.push(*mats.wrapping_add(i))
			}
			
			result
		}
	}
	/// #### ready
	/// Wrapper for Raylib::IsMaterialReady(material: Material).
	pub fn ready(&mut self) -> bool {
		unsafe{ IsMaterialReady(*self) }
	}
	/// #### unload
	/// Wrapper for Raylib::UnloadMaterial(material: Material).
	pub fn unload(&mut self) {
		unsafe{ UnloadMaterial(*self) }
	}
	/// #### set_texture
	/// Wrapper for Raylib::SetMaterialTexture(material: Material, map_type: i32, texture: Texture).
	pub fn set_texture(&mut self, map_type: i32, texture: Texture) {
		unsafe{ SetMaterialTexture(self, map_type, texture.0) }
	}
	
}


//= Material loading/unloading functions
extern "C" { fn LoadMaterials(filename: *const i8, materialCount: *const i32) -> *mut Material; }
extern "C" { fn LoadMaterialDefault() -> Material; }
extern "C" { fn IsMaterialReady(material: Material) -> bool; }
extern "C" { fn UnloadMaterial(material: Material); }
extern "C" { fn SetMaterialTexture(material: *mut Material, mapType: i32, texture: TextureRl); }


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MaterialMap {
	pub texture:	TextureRl,
	pub color:		Color,
	pub value:		f32,
}