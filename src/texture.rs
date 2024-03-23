

use crate::{rl_str, color::*, rectangle::*, vectors::*};


/// Wrapper for Texture
#[derive(Debug)]
pub struct Texture(pub TextureRl);
impl Into<TextureRl> for Texture {
	fn into(self) -> raylib_ffi::Texture {
		self.0
	}
}
impl From<TextureRl> for Texture {
	fn from(value: raylib_ffi::Texture) -> Self {
		Self { 0: value }
	}
}

#[derive(Debug)]
pub struct TextureRl {
	pub id:		u32,
    pub width:	i32,
    pub height:	i32,
    pub mipmaps: i32,
    pub format:	i32,
}

pub struct NPatchInfo {
	pub source:	Rectangle,
    pub left:	i32,
    pub top:	i32,
    pub right:	i32,
    pub bottom:	i32,
    pub layout:	i32,
}
impl Into<raylib_ffi::NPatchInfo> for NPatchInfo {
	fn into(self) -> raylib_ffi::NPatchInfo {
		raylib_ffi::NPatchInfo {
			source:	self.source.into(),
			left:	self.left,
			top:	self.top,
			right:	self.right,
			bottom:	self.bottom,
			layout:	self.layout,
		}
	}
}
impl From<raylib_ffi::NPatchInfo> for NPatchInfo {
	fn from(value: raylib_ffi::NPatchInfo) -> Self {
		Self {
			source:	Rectangle::from(value.source),
			left:	value.left,
			top:	value.top,
			right:	value.right,
			bottom:	value.bottom,
			layout:	value.layout,
		}
	}
}


impl Texture {
	
	//=Loading
	/// Wrapper for LoadTexture
	///
	/// Load texture from file into GPU memory (VRAM)
	pub fn load(filename: &str) -> Self {
		unsafe { Self(raylib_ffi::LoadTexture(rl_str!(filename))) }
	}
	/// Wrapper for UnloadTexture
	///
	/// Unload texture from GPU memory (VRAM)
	pub fn unload(&self) {
		unsafe { raylib_ffi::UnloadTexture(self.0) }
	}
	/// Wrapper for IsTextureReady
	///
	/// Check if a texture is ready
	pub fn is_ready(&self) -> bool {
		unsafe { raylib_ffi::IsTextureReady(self.0) }
	}
	/// Wrapper for UpdateTexture
	///
	/// Update GPU texture with new data
	pub fn update(&self, pixels: &mut Vec<Color>) {
		unsafe {
			raylib_ffi::UpdateTexture(self.0, pixels.as_mut_slice().as_mut_ptr().cast())
		}
	}
	/// Wrapper for UpdateTextureRec
	///
	/// Update GPU texture rectangle with new data
	pub fn update_rec(&self, rec: Rectangle, pixels: &mut Vec<Color>) {
		unsafe { raylib_ffi::UpdateTextureRec(self.0, rec.into(), pixels.as_mut_slice().as_mut_ptr().cast()) }
	}

	//= Manipulations
	/// Wrapper for GenTextureMipmaps
	///
	/// Generate GPU mipmaps for a texture
	pub fn gen_mipmaps(&mut self) {
		unsafe { raylib_ffi::GenTextureMipmaps(&mut self.0) }
	}
	/// Wrapper for SetTextureFilter
	///
	/// Set texture scaling filter mode
	pub fn set_filter(&self, filter: i32) {
		unsafe { raylib_ffi::SetTextureFilter(self.0, filter) }
	}
	/// Wrapper for SetTextureWrap
	///
	/// Set texture wrapping mode
	pub fn set_wrap(&self, wrap: i32) {
		unsafe { raylib_ffi::SetTextureWrap(self.0, wrap) }
	}
	/// Wrapper for DrawTexture
	///
	/// Draw a Texture2D
	pub fn draw(&self, pos_x: i32, pos_y: i32, tint: Color) {
		unsafe { raylib_ffi::DrawTexture(self.0, pos_x, pos_y, tint.into()) }
	}
	/// Wrapper for DrawTextureV
	///
	/// Draw a Texture2D with position defined as Vector2
	pub fn draw_v(&self, position: Vector2, tint: Color) {
		unsafe { raylib_ffi::DrawTextureV(self.0, position.into(), tint.into()) }
	}
	/// Wrapper for DrawTextureEx
	///
	/// Draw a Texture2D with extended parameters
	pub fn draw_ex(&self, position: Vector2, rotation: f32, scale: f32, tint: Color) {
		unsafe { raylib_ffi::DrawTextureEx(self.0, position.into(), rotation, scale, tint.into()) }
	}
	/// Wrapper for DrawTextureRec
	///
	/// Draw a part of a texture defined by a rectangle
	pub fn draw_rec(&self, source: Rectangle, position: Vector2, tint: Color) {
		unsafe { raylib_ffi::DrawTextureRec(self.0, source.into(), position.into(), tint.into()) }
	}
	/// Wrapper for DrawTexturePro
	///
	/// raw a part of a texture defined by a rectangle with 'pro' parameters
	pub fn draw_pro(&self, source: Rectangle, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
		unsafe { raylib_ffi::DrawTexturePro(self.0, source.into(), dest.into(), origin.into(), rotation, tint.into()) }
	}
	/// Wrapper for DrawTextureNPatch
	///
	/// Draws a texture (or part of it) that stretches or shrinks nicely
	pub fn draw_npatch(&self, npatch_info: NPatchInfo, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
		unsafe { raylib_ffi::DrawTextureNPatch(self.0, npatch_info.into(), dest.into(), origin.into(), rotation, tint.into()) }
	}

	//= Conversions

}
