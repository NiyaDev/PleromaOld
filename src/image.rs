

//= Imports
use raylib_ffi::rl_str;
use crate::{color::Color, pixel_format::PixelFormat, texture::Texture};


//= Structure and Enumerations

/// Texture stored on CPU
pub struct Image(pub raylib_ffi::Image);
impl Into<raylib_ffi::Image> for Image {
	fn into(self) -> raylib_ffi::Image {
		self.0
	}
}
impl From<raylib_ffi::Image> for Image {
	fn from(value: raylib_ffi::Image) -> Self {
		Self { 0: value }
	}
}


//= Constants


//= Implementations

impl Image {
	
	//= Loading

	/// Wrapper for LoadImage
	pub fn load(filename: &str) -> Self {
		unsafe { Self(raylib_ffi::LoadImage(rl_str!(filename))) }
	}
	/// Wrapper for LoadImageRaw
	pub fn load_raw(filename: &str, width: i32, height: i32, format: PixelFormat, header_size: i32) -> Self {
		unsafe { Self(raylib_ffi::LoadImageRaw(rl_str!(filename), width, height, format as i32, header_size)) }
	}
	/// Wrapper for LoadImageSVG
	pub fn load_svg(filename: &str, width: i32, height: i32) -> Self {
		unsafe { Self(raylib_ffi::LoadImageSvg(rl_str!(filename), width, height)) }
	}
	/// Wrapper for LoadImageAnim
	pub fn load_anim(filename: &str) -> (Self, i32) {
		unsafe {
			let mut frames = [0;1];
			let image = Self(raylib_ffi::LoadImageAnim(rl_str!(filename), frames.as_mut_ptr()));

			(image, frames[0])
		}
	}
	/// Wrapper for LoadImageFromMemory
	// TODO: Create enum for filetype?
	pub fn load_from_memory(file_type: &str, file_data: &mut Vec<u8>) -> Self {
		unsafe {
			let array = file_data.as_mut_ptr();
			Self(raylib_ffi::LoadImageFromMemory(raylib_ffi::rl_str!(file_type), array, file_data.len() as i32))
		}
	}
	/// Wrapper for LoadImageFromTexture
	pub fn load_from_texture(texture: Texture) -> Self {
		unsafe { Self(raylib_ffi::LoadImageFromTexture(texture.0)) }
	}
	/// Wrapper for LoadImageFromScreen
	pub fn load_from_screen() -> Self {
		unsafe { Self(raylib_ffi::LoadImageFromScreen()) }
	}
	/// Wrapper for IsImageReady
	pub fn is_image_ready(&self) -> bool {
		unsafe { raylib_ffi::IsImageReady(self.0) }
	}
	/// Wrapper for UnloadImage
	pub fn unload(&self) {
		unsafe { raylib_ffi::UnloadImage(self.0) }
	}
	/// Wrapper for ExportImage
	pub fn export_image(&self, filename: &str) -> bool {
		unsafe { raylib_ffi::ExportImage(self.0, rl_str!(filename)) }
	}
	/// Wrapper for ExportImageToMemory
	pub fn export_image_to_memory(&self, file_type: &str) -> Vec<u8> {
		unsafe {
			let mut array = Vec::new();
			let mut length = [0;1];
			let data = raylib_ffi::ExportImageToMemory(self.0, rl_str!(file_type), length.as_mut_ptr());

			for i in 0..length[0] as usize {
				array.push(data.wrapping_add(i).read())
			}

			array
		}
	}
	/// Wrapper for ExportImageAsCode
	pub fn export_image_as_code(&self, file_name: &str) -> bool {
		unsafe { raylib_ffi::ExportImageAsCode(self.0, rl_str!(file_name)) }
	}

	//= Generation
	/// Wrapper for GenImageColor
	pub fn gen_color(width: i32, height: i32, color: Color) -> Self {
		unsafe { Self(raylib_ffi::GenImageColor(width, height, color.into())) }
	}
	/// Wrapper for GenImageGradientLinear
	pub fn gen_linear_gradient(width: i32, height: i32, direction: i32, start: Color, end: Color) -> Self {
		unsafe { Self(raylib_ffi::GenImageGradientLinear(width, height, direction, start.into(), end.into())) }
	}
	/// Wrapper for GenImageGradientRadial
	pub fn gen_radial_gradient(width: i32, height: i32, density: f32, inner: Color, outer: Color) -> Self {
		unsafe { Self(raylib_ffi::GenImageGradientRadial(width, height, density, inner.into(), outer.into())) }
	}
	/// Wrapper for GenImageGradientSquare
	pub fn gen_square_gradient(width: i32, height: i32, density: f32, inner: Color, outer: Color) -> Self {
		unsafe { Self(raylib_ffi::GenImageGradientSquare(width, height, density, inner.into(), outer.into())) }
	}
	/// Wrapper for GenImageChecked
	pub fn gen_checked(width: i32, height: i32, checks_x: i32, checks_y: i32, col1: Color, col2: Color) -> Self {
		unsafe { Self(raylib_ffi::GenImageChecked(width, height, checks_x, checks_y, col1.into(), col2.into())) }
	}
	/// Wrapper for GenImageWhiteNoise
	pub fn gen_white_noise(width: i32, height: i32, factor: f32) -> Self {
		unsafe { Self(raylib_ffi::GenImageWhiteNoise(width, height, factor)) }
	}
	/// Wrapper for GenImagePerlinNoise
	pub fn gen_perlin_noise(width: i32, height: i32, offset_x: i32, offset_y: i32, scale: f32) -> Self {
		unsafe { Self(raylib_ffi::GenImagePerlinNoise(width, height, offset_x, offset_y, scale)) }
	}
	/// Wrapper for GenImageCellular
	pub fn gen_cellular(width: i32, height: i32, tile_size: i32) -> Self {
		unsafe { Self(raylib_ffi::GenImageCellular(width, height, tile_size)) }
	}
	/// Wrapper for GenImageText
	pub fn gen_text(width: i32, height: i32, text: &str) -> Self {
		unsafe { Self(raylib_ffi::GenImageText(width, height, rl_str!(text))) }
	}

	//= Manipulation

	//= Drawing

	//= Conversion

}


//= Procedures
