

use crate::{
	rl_str,
	color::*,
	rectangle::*,
	vectors::*,
};


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
#[derive(Debug, Clone, Copy)]
pub struct NPatchInfo {
	pub source:	Rectangle,
    pub left:	i32,
    pub top:	i32,
    pub right:	i32,
    pub bottom:	i32,
    pub layout:	i32,
}


impl TextureRl {
	
	//=Loading
	/// #### load
	/// Wrapper for Raylib::LoadTexture().
	pub fn load(filename: &str) -> Self {
		unsafe { LoadTexture(rl_str!(filename)) }
	}
	/// #### unload
	/// Wrapper for Raylib::UnloadTexture().
	pub fn unload(&self) {
		unsafe { UnloadTexture(*self) }
	}
	/// #### is_ready
	/// Wrapper for Raylib::IsTextureReady().
	pub fn is_ready(&self) -> bool {
		unsafe { IsTextureReady(*self) }
	}
	/// #### update
	/// Wrapper for Raylib::UpdateTexture().
	pub fn update(&self, pixels: &mut Vec<Color>) {
		unsafe {
			UpdateTexture(*self, pixels.as_mut_slice().as_mut_ptr().cast())
		}
	}
	/// #### update_rec
	/// Wrapper for Raylib::UpdateTextureRec().
	pub fn update_rec(&self, rec: Rectangle, pixels: &mut Vec<Color>) {
		unsafe { UpdateTextureRec(*self, rec, pixels.as_mut_slice().as_mut_ptr().cast()) }
	}

	//= Manipulations
	/// #### gen_mipmaps
	/// Wrapper for Raylib::GenTextureMipmaps().
	pub fn gen_mipmaps(&mut self) {
		unsafe { GenTextureMipmaps(self) }
	}
	/// #### set_filter
	/// Wrapper for Raylib::SetTextureFilter().
	pub fn set_filter(&self, filter: i32) {
		unsafe { SetTextureFilter(*self, filter) }
	}
	/// #### set_wrap
	/// Wrapper for Raylib::SetTextureWrap().
	pub fn set_wrap(&self, wrap: i32) {
		unsafe { SetTextureWrap(*self, wrap) }
	}
	/// #### draw
	/// Wrapper for Raylib::DrawTexture().
	pub fn draw(&self, pos_x: i32, pos_y: i32, tint: Color) {
		unsafe { DrawTexture(*self, pos_x, pos_y, tint) }
	}
	/// #### draw_v
	/// Wrapper for Raylib::DrawTextureV().
	pub fn draw_v(&self, position: Vector2, tint: Color) {
		unsafe { DrawTextureV(*self, position, tint) }
	}
	/// #### draw_ex
	/// Wrapper for Raylib::DrawTextureEx().
	pub fn draw_ex(&self, position: Vector2, rotation: f32, scale: f32, tint: Color) {
		unsafe { DrawTextureEx(*self, position, rotation, scale, tint) }
	}
	/// #### draw_rec
	/// Wrapper for Raylib::DrawTextureRec().
	pub fn draw_rec(&self, source: Rectangle, position: Vector2, tint: Color) {
		unsafe { DrawTextureRec(*self, source, position, tint) }
	}
	/// #### draw_pro
	/// Wrapper for Raylib::DrawTexturePro().
	pub fn draw_pro(&self, source: Rectangle, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
		unsafe { DrawTexturePro(*self, source, dest, origin, rotation, tint) }
	}
	/// #### draw_npatch
	/// Wrapper for Raylib::DrawTextureNPatch().
	pub fn draw_npatch(&self, npatch_info: NPatchInfo, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
		unsafe { DrawTextureNPatch(*self, npatch_info, dest, origin, rotation, tint) }
	}

	//= Conversions

}


//= Texture loading functions
extern "C" { fn LoadTexture(fileName: *const i8) -> TextureRl; }
extern "C" { fn IsTextureReady(texture: TextureRl) -> bool; }
extern "C" { fn UnloadTexture(texture: TextureRl); }
extern "C" { fn UpdateTexture(texture: TextureRl, pixels: *const ::std::os::raw::c_void); }
extern "C" { fn UpdateTextureRec(texture: TextureRl, rec: Rectangle, pixels: *const ::std::os::raw::c_void); }

//= Texture configuration functions
extern "C" { fn GenTextureMipmaps(texture: *mut TextureRl); }
extern "C" { fn SetTextureFilter(texture: TextureRl, filter: i32); }
extern "C" { fn SetTextureWrap(texture: TextureRl, wrap: i32); }

//= Texture drawing functions
extern "C" { fn DrawTexture(texture: TextureRl, posX: i32, posY: i32, tint: Color); }
extern "C" { fn DrawTextureV(texture: TextureRl, position: Vector2, tint: Color); }
extern "C" { fn DrawTextureEx(texture: TextureRl, position: Vector2, rotation: f32, scale: f32, tint: Color); }
extern "C" { fn DrawTextureRec(texture: TextureRl, source: Rectangle, position: Vector2, tint: Color); }
extern "C" { fn DrawTexturePro(texture: TextureRl, source: Rectangle, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color); }
extern "C" { fn DrawTextureNPatch(texture: TextureRl, nPatchInfo: NPatchInfo, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color); }