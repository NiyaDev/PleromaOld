

//= Imports


//= Structure and Enumerations

/// Wrapper for Texture
pub struct RenderTexture(pub raylib_ffi::RenderTexture);
impl Into<raylib_ffi::RenderTexture> for RenderTexture {
	fn into(self) -> raylib_ffi::RenderTexture {
		self.0
	}
}
impl From<raylib_ffi::RenderTexture> for RenderTexture {
	fn from(value: raylib_ffi::RenderTexture) -> Self {
		Self { 0: value }
	}
}


//= Constants


//= Implementations

impl RenderTexture {
	
	//=Loading

	/// Wrapper for LoadRenderTexture
	///
	/// Load texture from file into GPU memory (VRAM)
	pub fn load(width: i32, height: i32) -> Self {
		unsafe { RenderTexture::from(raylib_ffi::LoadRenderTexture(width, height)) }
	}
	/// Wrapper for UnloadRenderTexture
	///
	/// Unload render texture from GPU memory (VRAM)
	pub fn unload(&self) {
		unsafe { raylib_ffi::UnloadRenderTexture(self.0) }
	}
	/// Wrapper for IsRenderTextureReady
	///
	/// Check if a render texture is ready
	pub fn is_ready(&self) -> bool {
		unsafe { raylib_ffi::IsRenderTextureReady(self.0) }
	}

	//= Manipulations
	/// Wrapper for BeginTextureMode
	///
	/// Begin drawing to render texture
	pub fn begin_texture_mode(&self) {
		unsafe { raylib_ffi::BeginTextureMode(self.0) }
	}
	/// Wrapper for EndTextureMode
	///
	/// Ends drawing to render texture
	pub fn end_texture_mode(&self) {
		unsafe { raylib_ffi::EndTextureMode() }
	}

	//= Conversions

}


//= Procedures
