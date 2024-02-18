

//= Imports
use super::{color::Color, pixel_format::PixelFormat, rectangle::Rectangle, vector::Vector2};


//= Structure and Enumerations

/// Texture wrapper
#[derive(Debug, Clone, Copy)]
pub struct Texture {
	pub id:			u32,

    pub width:		i32,
    pub height:		i32,

    pub mipmaps:	i32,

    pub format:		PixelFormat,

	pub tint:		Color,
}

/// Render texture wrapper
#[derive(Debug, Clone, Copy)]
pub struct RenderTexture {
	pub id:			u32,

    pub texture:	Texture,
	pub depth:		Texture,
}


//= Procedures

impl Texture {

	//= Loading
	/// Wrapper for LoadTexture
	pub fn load(filename: &str) -> Self {
		unsafe { return Texture::from_ffi(raylib_ffi::LoadTexture(raylib_ffi::rl_str!(filename))) }
	}

	//= Generation

	//= Drawing
	/// Wrapper for DrawTexture
	pub fn draw(&self, pos_x: i32, pos_y: i32) {
		unsafe {
			raylib_ffi::DrawTexture(self.into_ffi(), pos_x, pos_y, self.tint.into_ffi())
		}
	}
	/// Wrapper for DrawTexturePro
	pub fn draw_pro(&self, source: Rectangle, dest: Rectangle, origin: Vector2, rotation: f32) {
		unsafe { raylib_ffi::DrawTexturePro(self.into_ffi(), source.into_ffi(), dest.into_ffi(), origin.into_ffi(), rotation, self.tint.into_ffi()) }
	}

	//= Conversion
	/// Converting to FFI version
	pub fn into_ffi(&self) -> raylib_ffi::Texture {
		raylib_ffi::Texture{
			id: self.id,
			width: self.width,
			height: self.height,
			mipmaps: self.mipmaps,
			format: self.format as i32,
		}
	}
	/// Converting from FFI version
	pub fn from_ffi(value: raylib_ffi::Texture) -> Self {
		Self {
			id: value.id,
			width: value.width,
			height: value.height,
			mipmaps: value.mipmaps,
			format: PixelFormat::from(value.format),
			tint: Color::white(),
		}
	}

}

impl RenderTexture {

	//= Loading
	/// Wrapper for LoadRenderTexture
	pub fn load_render(width: i32, height: i32) -> Self {
		unsafe { return RenderTexture::from_ffi(raylib_ffi::LoadRenderTexture(width, height)) }
	}

	//= Drawing
	/// Wrapper for BeginTextureMode
	pub fn begin_texture_mode(&self) {
		unsafe { raylib_ffi::BeginTextureMode(self.into_ffi()) }
	}
	/// Wrapper for EndTextureMode
	pub fn end_texture_mode(&self) {
		unsafe { raylib_ffi::EndTextureMode() }
	}

	//= Conversion
	/// Converting to FFI version
	pub fn into_ffi(&self) -> raylib_ffi::RenderTexture {
		raylib_ffi::RenderTexture {
			id:			self.id,
			texture:	self.texture.into_ffi(),
			depth:		self.depth.into_ffi(),
		}
	}
	/// Converting from FFI version
	pub fn from_ffi(value: raylib_ffi::RenderTexture) -> Self {
		Self {
			id:			value.id,
			texture:	Texture::from_ffi(value.texture),
			depth:		Texture::from_ffi(value.depth),
		}
	}

}