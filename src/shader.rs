

use std::{collections::HashMap, os::raw::c_void};

use crate::rl_str;

use super::{texture::*, vectors::*};


/// Shader wrapper
#[derive(Debug, Clone)]
pub struct Shader {
	pub shader: ShaderRl,
	pub locations: HashMap <String,i32>,
	pub attributes: HashMap<String,i32>,
}

impl Shader {
	
	/// #### load
	/// Wrapper for Raylib::LoadShader(vsFilename: *const i8, fsFilename: *const i8).
	pub fn load(vs_filename: *const i8, fs_filename: *const i8) -> Self {
		unsafe{
			Self {
				shader: LoadShader(vs_filename, fs_filename),
				locations: HashMap::new(),
				attributes: HashMap::new(),
			}
		}
	}
	/// #### ready
	/// Wrapper for Raylib::IsShaderReady(shader: ShaderRl) -> bool.
	pub fn ready(&mut self) -> bool {
		unsafe{ IsShaderReady(self.shader) }
	}
	/// #### unload
	/// Wrapper for Raylib::UnloadShader(shader: ShaderRl).
	pub fn unload(&mut self) {
		unsafe{ UnloadShader(self.shader) }
	}
	
	/// #### draw
	/// Draws contents using shader.
	pub fn draw(&mut self, draw_contents: impl FnOnce(&mut Shader)) -> &mut Self {
		unsafe{
			BeginShaderMode(self.shader);
			
			let _ = draw_contents(self);
			
			EndShaderMode();
			
			self
		}
	}
	
	/// #### register_location
	/// Gets the location value for location and inserts into hashmap.
	pub fn register_location(&mut self, name: &str) -> &mut Self {
		unsafe{
			let val = GetShaderLocation(self.shader, rl_str!(name));
			self.locations.insert(name.to_string(), val);
		}
		
		self
	}
	/// #### set_value
	/// Wrapper for seting location values.
	pub fn set_value(&mut self, name: &str, value: ShaderValue) -> &mut Self {
		unsafe{
			let loc = self.locations.get(&name.to_string());
			if loc.is_none() { println!("Failed to get shader location.") }
			
			match value {
				ShaderValue::Float(f) => {
					let arr = [f];
					SetShaderValue(self.shader, *loc.unwrap(), arr.as_ptr().cast(), value.to_i32())
				}
				ShaderValue::Vec2(v) => {
					let arr = [v];
					SetShaderValueV(self.shader, *loc.unwrap(), arr.as_ptr().cast(), value.to_i32())
				}
				ShaderValue::Vec3(v) => {
					let arr = [v];
					SetShaderValueV(self.shader, *loc.unwrap(), arr.as_ptr().cast(), value.to_i32())
				}
				ShaderValue::Vec4(v) => {
					let arr = [v];
					SetShaderValueV(self.shader, *loc.unwrap(), arr.as_ptr().cast(), value.to_i32())
				}
				ShaderValue::Int(i) => {
					let arr = [i];
					SetShaderValue(self.shader, *loc.unwrap(), arr.as_ptr().cast(), value.to_i32())
				}
				ShaderValue::IVec2(v) => {
					let arr = [v];
					SetShaderValueV(self.shader, *loc.unwrap(), arr.as_ptr().cast(), value.to_i32())
				}
				ShaderValue::IVec3(v) => {
					let arr = [v];
					SetShaderValueV(self.shader, *loc.unwrap(), arr.as_ptr().cast(), value.to_i32())
				}
				ShaderValue::IVec4(v) => {
					let arr = [v];
					SetShaderValueV(self.shader, *loc.unwrap(), arr.as_ptr().cast(), value.to_i32())
				}
				ShaderValue::Sampler2D(t) => {
					let arr = [t];
					SetShaderValueTexture(self.shader, *loc.unwrap(), arr.as_ptr().cast(), value.to_i32())
				}
			}
		}
		
		self
	}
	
}

//= Drawing-related functions
extern "C" { fn BeginShaderMode(shader: ShaderRl); }
extern "C" { fn EndShaderMode(); }

//= Shader management functions
extern "C" { fn LoadShader(vsFilename: *const i8, fsFilename: *const i8) -> ShaderRl; }
extern "C" { fn IsShaderReady(shader: ShaderRl) -> bool; }
extern "C" { fn GetShaderLocation(shader: ShaderRl, uniformName: *const i8) -> i32; }
//extern "C" { fn GetShaderLocationAttrib(shader: ShaderRl, atttribName: *const i8) -> i32; }
extern "C" { fn SetShaderValue(shader: ShaderRl, locIndex: i32, value: *const c_void, uniformType: i32); }
extern "C" { fn SetShaderValueV(shader: ShaderRl, locIndex: i32, value: *const c_void, uniformType: i32); }
//extern "C" { fn SetShaderValueMatrix(shader: ShaderRl, locIndex: i32, value: *const c_void, uniformType: i32); }
extern "C" { fn SetShaderValueTexture(shader: ShaderRl, locIndex: i32, value: *const c_void, uniformType: i32); }
extern "C" { fn UnloadShader(shader: ShaderRl); }


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ShaderValue {
	Float(f32),
	Vec2(Vector2),
	Vec3(Vector3),
	Vec4(Vector4),
	Int(i32),
	IVec2((i32,i32)),
	IVec3((i32,i32,i32)),
	IVec4((i32,i32,i32,i32)),
	Sampler2D(TextureRl),
}
impl ShaderValue {
	
	//
	pub fn to_i32(self) -> i32 {
		match self {
			ShaderValue::Float(..)		=> 0,
			ShaderValue::Vec2(_)		=> 1,
			ShaderValue::Vec3(_)		=> 2,
			ShaderValue::Vec4(_)		=> 3,
			ShaderValue::Int(_)			=> 4,
			ShaderValue::IVec2(_)		=> 5,
			ShaderValue::IVec3(_)		=> 6,
			ShaderValue::IVec4(_)		=> 7,
			ShaderValue::Sampler2D(_)	=> 8,
		}
	}
	
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ShaderRl {
	pub id: u32,
    pub locs: *mut i32,
}
