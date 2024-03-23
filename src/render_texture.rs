

use crate::texture::*;


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
	/// Wrapper for LoadRenderTexture
	///
	/// Load texture for rendering (framebuffer)
	pub fn load(width: i32, height: i32) -> Self {
		unsafe { Self(LoadRenderTexture(width, height)) }
	}
	/// Wrapper for UnloadRenderTexture
	///
	/// Unload render texture from GPU memory (VRAM)
	pub fn unload(&self) {
		unsafe { UnloadRenderTexture(self.0) }
	}
	/// Wrapper for IsRenderTextureReady
	///
	/// Check if a render texture is ready
	pub fn is_ready(&self) -> bool {
		unsafe { IsRenderTextureReady(self.0) }
	}

	//= Manipulations
	/// Wrapper for BeginTextureMode
	///
	/// Begin drawing to render texture
	pub fn begin_texture_mode(&self) {
		unsafe { BeginTextureMode(self.0) }
	}
	/// Wrapper for EndTextureMode
	///
	/// Ends drawing to render texture
	pub fn end_texture_mode(&self) {
		unsafe { EndTextureMode() }
	}

	//= Conversions

}


extern "C" { fn LoadRenderTexture(width: i32, height: i32) -> RenderTextureRl; }
extern "C" { fn UnloadRenderTexture(target: RenderTextureRl); }
extern "C" { fn IsRenderTextureReady(target: RenderTextureRl) -> bool; }
extern "C" { fn BeginTextureMode(target: RenderTextureRl); }
extern "C" { fn EndTextureMode(); }