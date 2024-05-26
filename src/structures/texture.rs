

use crate::{
	rl_str,
	structures::{
		color::*,
		rectangle::*,
		vectors::*,
	}
};


/// Wrapper for Texture
#[derive(Debug, Clone, Copy)]
pub struct Texture(pub TextureRl, pub Color);

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


impl Texture {
	
	//=Loading
	/// #### load
	/// Wrapper for Raylib::LoadTexture().
	pub fn load(filename: &str) -> Self {
		unsafe { Self(LoadTexture(rl_str!(filename)), WHITE) }
	}
	/// #### unload
	/// Wrapper for Raylib::UnloadTexture().
	pub fn unload(&self) {
		unsafe { UnloadTexture(self.0) }
	}
	/// #### is_ready
	/// Wrapper for Raylib::IsTextureReady().
	pub fn is_ready(&self) -> bool {
		unsafe { IsTextureReady(self.0) }
	}
	/// #### update
	/// Wrapper for Raylib::UpdateTexture().
	pub fn update(&self, pixels: &mut Vec<Color>) {
		unsafe {
			UpdateTexture(self.0, pixels.as_mut_slice().as_mut_ptr().cast())
		}
	}
	/// #### update_rec
	/// Wrapper for Raylib::UpdateTextureRec().
	pub fn update_rec(&self, rec: Rectangle, pixels: &mut Vec<Color>) {
		unsafe { UpdateTextureRec(self.0, rec, pixels.as_mut_slice().as_mut_ptr().cast()) }
	}
	/// #### set_tint
	/// Set texture tint
	pub fn set_tint(&mut self, tint: Color) {
		self.1 = tint;
	}

	//= Manipulations
	/// #### gen_mipmaps
	/// Wrapper for Raylib::GenTextureMipmaps().
	pub fn gen_mipmaps(&mut self) {
		unsafe { GenTextureMipmaps(&mut self.0) }
	}
	/// #### set_filter
	/// Wrapper for Raylib::SetTextureFilter().
	pub fn set_filter(&self, filter: i32) {
		unsafe { SetTextureFilter(self.0, filter) }
	}
	/// #### set_wrap
	/// Wrapper for Raylib::SetTextureWrap().
	pub fn set_wrap(&self, wrap: i32) {
		unsafe { SetTextureWrap(self.0, wrap) }
	}
	/// #### draw
	/// Wrapper for Raylib::DrawTexture().
	pub fn draw(&self, pos_x: i32, pos_y: i32) {
		unsafe { DrawTexture(self.0, pos_x, pos_y, self.1) }
	}
	/// #### draw_v
	/// Wrapper for Raylib::DrawTextureV().
	pub fn draw_v(&self, position: Vector2) {
		unsafe { DrawTextureV(self.0, position, self.1) }
	}
	/// #### draw_ex
	/// Wrapper for Raylib::DrawTextureEx().
	pub fn draw_ex(&self, position: Vector2, rotation: f32, scale: f32) {
		unsafe { DrawTextureEx(self.0, position, rotation, scale, self.1) }
	}
	/// #### draw_rec
	/// Wrapper for Raylib::DrawTextureRec().
	pub fn draw_rec(&self, source: Rectangle, position: Vector2) {
		unsafe { DrawTextureRec(self.0, source, position, self.1) }
	}
	/// #### draw_pro
	/// Wrapper for Raylib::DrawTexturePro().
	pub fn draw_pro(&self, source: Rectangle, dest: Rectangle, origin: Vector2, rotation: f32) {
		unsafe { DrawTexturePro(self.0, source, dest, origin, rotation, self.1) }
	}
	/// #### draw_npatch
	/// Wrapper for Raylib::DrawTextureNPatch().
	pub fn draw_npatch(&self, npatch_info: NPatchInfo, dest: Rectangle, origin: Vector2, rotation: f32) {
		unsafe { DrawTextureNPatch(self.0, npatch_info, dest, origin, rotation, self.1) }
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