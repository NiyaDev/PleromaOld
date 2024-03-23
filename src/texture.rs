

use crate::{rl_str, color::*, rectangle::*, vectors::*};


/// Wrapper for Texture
#[derive(Debug)]
pub struct Texture(pub TextureRl);

/// Raw raylib structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TextureRl {
	pub id:		u32,
    pub width:	i32,
    pub height:	i32,
    pub mipmaps: i32,
    pub format:	i32,
}

/// Npatch
#[repr(C)]
pub struct NPatchInfo {
	pub source:	Rectangle,
    pub left:	i32,
    pub top:	i32,
    pub right:	i32,
    pub bottom:	i32,
    pub layout:	i32,
}


impl Texture {
	
	//=Loading
	/// Wrapper for LoadTexture
	///
	/// Load texture from file into GPU memory (VRAM)
	pub fn load(filename: &str) -> Self {
		unsafe { Self(LoadTexture(rl_str!(filename))) }
	}
	/// Wrapper for UnloadTexture
	///
	/// Unload texture from GPU memory (VRAM)
	pub fn unload(&self) {
		unsafe { UnloadTexture(self.0) }
	}
	/// Wrapper for IsTextureReady
	///
	/// Check if a texture is ready
	pub fn is_ready(&self) -> bool {
		unsafe { IsTextureReady(self.0) }
	}
	/// Wrapper for UpdateTexture
	///
	/// Update GPU texture with new data
	pub fn update(&self, pixels: &mut Vec<Color>) {
		unsafe {
			UpdateTexture(self.0, pixels.as_mut_slice().as_mut_ptr().cast())
		}
	}
	/// Wrapper for UpdateTextureRec
	///
	/// Update GPU texture rectangle with new data
	pub fn update_rec(&self, rec: Rectangle, pixels: &mut Vec<Color>) {
		unsafe { UpdateTextureRec(self.0, rec, pixels.as_mut_slice().as_mut_ptr().cast()) }
	}

	//= Manipulations
	/// Wrapper for GenTextureMipmaps
	///
	/// Generate GPU mipmaps for a texture
	pub fn gen_mipmaps(&mut self) {
		unsafe { GenTextureMipmaps(&mut self.0) }
	}
	/// Wrapper for SetTextureFilter
	///
	/// Set texture scaling filter mode
	pub fn set_filter(&self, filter: i32) {
		unsafe { SetTextureFilter(self.0, filter) }
	}
	/// Wrapper for SetTextureWrap
	///
	/// Set texture wrapping mode
	pub fn set_wrap(&self, wrap: i32) {
		unsafe { SetTextureWrap(self.0, wrap) }
	}
	/// Wrapper for DrawTexture
	///
	/// Draw a Texture2D
	pub fn draw(&self, pos_x: i32, pos_y: i32, tint: Color) {
		unsafe { DrawTexture(self.0, pos_x, pos_y, tint) }
	}
	/// Wrapper for DrawTextureV
	///
	/// Draw a Texture2D with position defined as Vector2
	pub fn draw_v(&self, position: Vector2, tint: Color) {
		unsafe { DrawTextureV(self.0, position, tint) }
	}
	/// Wrapper for DrawTextureEx
	///
	/// Draw a Texture2D with extended parameters
	pub fn draw_ex(&self, position: Vector2, rotation: f32, scale: f32, tint: Color) {
		unsafe { DrawTextureEx(self.0, position, rotation, scale, tint) }
	}
	/// Wrapper for DrawTextureRec
	///
	/// Draw a part of a texture defined by a rectangle
	pub fn draw_rec(&self, source: Rectangle, position: Vector2, tint: Color) {
		unsafe { DrawTextureRec(self.0, source, position, tint) }
	}
	/// Wrapper for DrawTexturePro
	///
	/// raw a part of a texture defined by a rectangle with 'pro' parameters
	pub fn draw_pro(&self, source: Rectangle, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
		unsafe { DrawTexturePro(self.0, source, dest, origin, rotation, tint) }
	}
	/// Wrapper for DrawTextureNPatch
	///
	/// Draws a texture (or part of it) that stretches or shrinks nicely
	pub fn draw_npatch(&self, npatch_info: NPatchInfo, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
		unsafe { DrawTextureNPatch(self.0, npatch_info, dest, origin, rotation, tint) }
	}

	//= Conversions

}


extern "C" { fn LoadTexture(fileName: *const i8) -> TextureRl; }
extern "C" { fn IsTextureReady(texture: TextureRl) -> bool; }
extern "C" { fn UnloadTexture(texture: TextureRl); }
extern "C" { fn UpdateTexture(texture: TextureRl, pixels: *const ::std::os::raw::c_void); }
extern "C" { fn UpdateTextureRec(texture: TextureRl, rec: Rectangle, pixels: *const ::std::os::raw::c_void); }
extern "C" { fn GenTextureMipmaps(texture: *mut TextureRl); }
extern "C" { fn SetTextureFilter(texture: TextureRl, filter: i32); }
extern "C" { fn SetTextureWrap(texture: TextureRl, wrap: i32); }
extern "C" { fn DrawTexture(texture: TextureRl, posX: i32, posY: i32, tint: Color); }
extern "C" { fn DrawTextureV(texture: TextureRl, position: Vector2, tint: Color); }
extern "C" { fn DrawTextureEx(texture: TextureRl, position: Vector2, rotation: f32, scale: f32, tint: Color); }
extern "C" { fn DrawTextureRec(texture: TextureRl, source: Rectangle, position: Vector2, tint: Color); }
extern "C" { fn DrawTexturePro(texture: TextureRl, source: Rectangle, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color); }
extern "C" { fn DrawTextureNPatch(texture: TextureRl, nPatchInfo: NPatchInfo, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color); }