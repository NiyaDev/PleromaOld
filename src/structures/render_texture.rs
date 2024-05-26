

use crate::structures::texture::*;


/// Texture wrapper
#[derive(Debug, Clone, Copy)]
pub struct RenderTexture(pub RenderTextureRl);
impl Into<RenderTextureRl> for RenderTexture {
	fn into(self) -> RenderTextureRl {
		self.0
	}
}
impl From<RenderTextureRl> for RenderTexture {
	fn from(value: RenderTextureRl) -> Self {
		Self { 0: value }
	}
}

/// Raw raylib structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RenderTextureRl {
	pub id: u32,
    pub texture: TextureRl,
    pub depth: TextureRl,
}

impl RenderTexture {
	
	//=Loading
	/// #### load
	/// Wrapper for Raylib::LoadRenderTexture().
	pub fn load(width: i32, height: i32) -> Self {
		unsafe { Self(LoadRenderTexture(width, height)) }
	}
	/// #### unload
	/// Wrapper for Raylib::UnloadRenderTexture().
	pub fn unload(&self) {
		unsafe { UnloadRenderTexture(self.0) }
	}
	/// #### is_ready
	/// Wrapper for Raylib::IsRenderTextureReady().
	pub fn is_ready(&self) -> bool {
		unsafe { IsRenderTextureReady(self.0) }
	}

	//= Manipulations
	/// #### begin_texture_mode
	/// Wrapper for Raylib::BeginTextureMode().
	pub fn begin_texture_mode(&self) {
		unsafe { BeginTextureMode(self.0) }
	}
	/// #### end_texture_mode
	/// Wrapper for Raylib::EndTextureMode().
	pub fn end_texture_mode(&self) {
		unsafe { EndTextureMode() }
	}

	//= Conversions

}


//= Texture loading functions
extern "C" { fn LoadRenderTexture(width: i32, height: i32) -> RenderTextureRl; }
extern "C" { fn IsRenderTextureReady(target: RenderTextureRl) -> bool; }
extern "C" { fn UnloadRenderTexture(target: RenderTextureRl); }

//= Drawing-related functions
extern "C" { fn BeginTextureMode(target: RenderTextureRl); }
extern "C" { fn EndTextureMode(); }