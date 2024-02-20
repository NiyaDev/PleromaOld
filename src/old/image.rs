


//= Imports
use super::{pixel_format::*, texture::*};
use std::ffi::c_void;


//= Structure and Enumerations

//
pub struct Image {
	pub data:		*mut c_void,
	//pub data:		Vec<Color>,

	pub width:		i32,
	pub height:		i32,

	pub mipmaps:	i32,
	pub format:		PixelFormat,
}


//= Procedures

impl Image {

	//= Generation
	/// Loading image from file
	pub fn load(filename: &str) -> Self {
		unsafe { return Image::from_ffi(raylib_ffi::LoadImage(raylib_ffi::rl_str!(filename))) }
	}
	
	//= Editing

	//= Conversion
	/// Converting to FFI version
	pub fn into_ffi(&self) -> raylib_ffi::Image {
		raylib_ffi::Image{
			data:		self.data,
			width:		self.width,
			height:		self.height,
			mipmaps:	self.mipmaps,
			format:		self.format as i32,
		}
	}
	/// Converting from FFI version
	pub fn from_ffi(value: raylib_ffi::Image) -> Self {
		Self{
			data:		value.data,
			width:		value.width,
			height:		value.height,
			mipmaps:	value.mipmaps,
			format:		PixelFormat::from(value.format),
		}
	}
	/// Uploading to GPU as a texture
	pub fn to_texture(&self) -> Texture {
		unsafe { return Texture::from_ffi(raylib_ffi::LoadTextureFromImage(self.into_ffi())); }
	}

}