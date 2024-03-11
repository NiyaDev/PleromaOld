

//= Imports
use raylib_ffi::rl_str;
use crate::{color::Color, font::Font, pixel_format::PixelFormat, rectangle::Rectangle, texture::Texture, vectors::Vector2};


//= Structure and Enumerations

/// Texture stored on CPU
#[derive(Debug)]
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
impl PartialEq for Image {
	fn eq(&self, other: &Self) -> bool {
		let mut result;

		result = self.0.width == other.0.width &&
			self.0.height == other.0.height &&
			self.0.mipmaps == other.0.mipmaps &&
			self.0.format == other.0.format;

		if result {
			for i in 0..(self.0.width*self.0.height) {
				let self_color = self.get_color(i/self.0.width, i%self.0.width);
				let other_color = other.get_color(i/self.0.width, i%self.0.width);
				if self_color != other_color {
					result = false;
					break;
				}
			}
		}

		result
	}
}


//= Constants


//= Implementations

impl Image {
	
	//= Loading

	/// Wrapper for LoadImage
	///
	/// Load image from file into CPU memory (RAM)
	pub fn load(filename: &str) -> Self {
		unsafe { Self(raylib_ffi::LoadImage(rl_str!(filename))) }
	}
	/// Wrapper for LoadImageRaw
	///
	/// Load image from RAW file data
	pub fn load_raw(filename: &str, width: i32, height: i32, format: PixelFormat, header_size: i32) -> Self {
		unsafe { Self(raylib_ffi::LoadImageRaw(rl_str!(filename), width, height, format as i32, header_size)) }
	}
	/// Wrapper for LoadImageSVG
	///
	/// Load image from SVG file data or string with specified size
	pub fn load_svg(filename: &str, width: i32, height: i32) -> Self {
		unsafe { Self(raylib_ffi::LoadImageSvg(rl_str!(filename), width, height)) }
	}
	/// Wrapper for LoadImageAnim
	///
	/// Load image sequence from file (frames appended to image.data)
	pub fn load_anim(filename: &str) -> (Self, i32) {
		unsafe {
			let mut frames = [0;1];
			let image = Self(raylib_ffi::LoadImageAnim(rl_str!(filename), frames.as_mut_ptr()));

			(image, frames[0])
		}
	}
	/// Wrapper for LoadImageFromMemory
	///
	/// Load image from memory buffer, fileType refers to extension: i.e. '.png'
	// TODO: Create enum for filetype?
	pub fn load_from_memory(file_type: &str, file_data: &mut Vec<u8>) -> Self {
		unsafe {
			let array = file_data.as_mut_ptr();
			Self(raylib_ffi::LoadImageFromMemory(raylib_ffi::rl_str!(file_type), array, file_data.len() as i32))
		}
	}
	/// Wrapper for LoadImageFromTexture
	///
	/// Load image from GPU texture data
	pub fn load_from_texture(texture: Texture) -> Self {
		unsafe { Self(raylib_ffi::LoadImageFromTexture(texture.0)) }
	}
	/// Wrapper for LoadImageFromScreen
	///
	/// Load image from screen buffer and (screenshot)
	pub fn load_from_screen() -> Self {
		unsafe { Self(raylib_ffi::LoadImageFromScreen()) }
	}
	/// Wrapper for IsImageReady
	///
	/// Check if an image is ready
	pub fn is_image_ready(&self) -> bool {
		unsafe { raylib_ffi::IsImageReady(self.0) }
	}
	/// Wrapper for UnloadImage
	///
	/// Unload image from CPU memory (RAM)
	pub fn unload(&self) {
		unsafe { raylib_ffi::UnloadImage(self.0) }
	}
	/// Wrapper for ExportImage
	///
	/// Export image data to file, returns true on success
	pub fn export_image(&self, filename: &str) -> bool {
		unsafe { raylib_ffi::ExportImage(self.0, rl_str!(filename)) }
	}
	/// Wrapper for ExportImageToMemory
	///
	/// Export image to memory buffer
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
	///
	/// Export image as code file defining an array of bytes, returns true on success
	pub fn export_image_as_code(&self, file_name: &str) -> bool {
		unsafe { raylib_ffi::ExportImageAsCode(self.0, rl_str!(file_name)) }
	}

	//= Generation
	/// Wrapper for GenImageColor
	///
	/// Generate image: plain color
	pub fn gen_color(width: i32, height: i32, color: Color) -> Self {
		unsafe { Self(raylib_ffi::GenImageColor(width, height, color.into())) }
	}
	/// Wrapper for GenImageGradientLinear
	///
	/// Generate image: linear gradient, direction in degrees [0..360], 0=Vertical gradient
	pub fn gen_linear_gradient(width: i32, height: i32, direction: i32, start: Color, end: Color) -> Self {
		unsafe { Self(raylib_ffi::GenImageGradientLinear(width, height, direction, start.into(), end.into())) }
	}
	/// Wrapper for GenImageGradientRadial
	///
	/// Generate image: radial gradient
	pub fn gen_radial_gradient(width: i32, height: i32, density: f32, inner: Color, outer: Color) -> Self {
		unsafe { Self(raylib_ffi::GenImageGradientRadial(width, height, density, inner.into(), outer.into())) }
	}
	/// Wrapper for GenImageGradientSquare
	///
	/// Generate image: square gradient
	pub fn gen_square_gradient(width: i32, height: i32, density: f32, inner: Color, outer: Color) -> Self {
		unsafe { Self(raylib_ffi::GenImageGradientSquare(width, height, density, inner.into(), outer.into())) }
	}
	/// Wrapper for GenImageChecked
	///
	/// Generate image: checked
	pub fn gen_checked(width: i32, height: i32, checks_x: i32, checks_y: i32, col1: Color, col2: Color) -> Self {
		unsafe { Self(raylib_ffi::GenImageChecked(width, height, checks_x, checks_y, col1.into(), col2.into())) }
	}
	/// Wrapper for GenImageWhiteNoise
	///
	/// Generate image: white noise
	pub fn gen_white_noise(width: i32, height: i32, factor: f32) -> Self {
		unsafe { Self(raylib_ffi::GenImageWhiteNoise(width, height, factor)) }
	}
	/// Wrapper for GenImagePerlinNoise
	///
	/// Generate image: perlin noise
	pub fn gen_perlin_noise(width: i32, height: i32, offset_x: i32, offset_y: i32, scale: f32) -> Self {
		unsafe { Self(raylib_ffi::GenImagePerlinNoise(width, height, offset_x, offset_y, scale)) }
	}
	/// Wrapper for GenImageCellular
	///
	/// Generate image: cellular algorithm, bigger tileSize means bigger cells
	pub fn gen_cellular(width: i32, height: i32, tile_size: i32) -> Self {
		unsafe { Self(raylib_ffi::GenImageCellular(width, height, tile_size)) }
	}
	/// Wrapper for GenImageText
	///
	/// Generate image: grayscale image from text data
	pub fn gen_text(width: i32, height: i32, text: &str) -> Self {
		unsafe { Self(raylib_ffi::GenImageText(width, height, rl_str!(text))) }
	}

	//= Manipulation
	/// Wrapper for ImageCopy
	///
	/// Create an image duplicate (useful for transformations)
	pub fn copy(&self) -> Self {
		unsafe { Self(raylib_ffi::ImageCopy(self.0)) }
	}
	/// Wrapper for ImageFromImage
	///
	/// Create an image from another image piece
	pub fn from_image(&self, rect: Rectangle) -> Self {
		unsafe { Self(raylib_ffi::ImageFromImage(self.0, rect.into())) }
	}
	/// Wrapper for ImageText
	///
	/// Create an image from text (default font)
	pub fn text(text: &str, font_size: i32, color: Color) -> Self {
		unsafe { Self(raylib_ffi::ImageText(rl_str!(text), font_size, color.into())) }
	}
	/// Wrapper for ImageTextEx
	///
	/// Create an image from text (custom sprite font)
	pub fn text_ex(font: Font, text: &str, font_size: f32, spacing: f32, tint: Color) -> Self {
		unsafe { Self(raylib_ffi::ImageTextEx(font.into(), rl_str!(text), font_size, spacing, tint.into())) }
	}
	/// Wrapper for ImageFormat
	///
	/// Convert image data to desired format
	pub fn format(&mut self, new_format: PixelFormat) {
		unsafe { raylib_ffi::ImageFormat(&mut self.0, new_format as i32) }
	}
	/// Wrapper for ImageToPOT
	///
	/// Convert image to POT (power-of-two)
	pub fn to_pot(&mut self, fill: Color) {
		unsafe { raylib_ffi::ImageToPOT(&mut self.0, fill.into()) }
	}
	/// Wrapper for ImageCrop
	///
	/// Crop an image to a defined rectangle
	pub fn crop(&mut self, crop: Rectangle) {
		unsafe { raylib_ffi::ImageCrop(&mut self.0, crop.into()) }
	}
	/// Wrapper for ImageAlphaCrop
	///
	/// Crop image depending on alpha value
	pub fn crop_alpha(&mut self, threshold: f32) {
		unsafe { raylib_ffi::ImageAlphaCrop(&mut self.0, threshold) }
	}
	/// Wrapper for ImageAlphaClear
	///
	/// Clear alpha channel to desired color
	pub fn crop_clear(&mut self, color: Color, threshold: f32) {
		unsafe { raylib_ffi::ImageAlphaClear(&mut self.0, color.into(), threshold) }
	}
	/// Wrapper for ImageAlphaMask
	///
	/// Apply alpha mask to image
	pub fn alpha_mask(&mut self, alpha_mask: Image) {
		unsafe { raylib_ffi::ImageAlphaMask(&mut self.0, alpha_mask.into()) }
	}
	/// Wrapper for ImageAlphaPremultiply
	///
	/// Premultiply alpha channel
	pub fn alpha_premultiply(&mut self) {
		unsafe { raylib_ffi::ImageAlphaPremultiply(&mut self.0) }
	}
	/// Wrapper for ImageBlurGaussian
	///
	/// Apply Gaussian blur using a box blur approximation
	pub fn blur_gaussian(&mut self, blur_size: i32) {
		unsafe { raylib_ffi::ImageBlurGaussian(&mut self.0, blur_size) }
	}
	/// Wrapper for ImageResize
	///
	/// Resize image (Bicubic scaling algorithm)
	pub fn resize(&mut self, new_width: i32, new_height: i32) {
		unsafe { raylib_ffi::ImageResize(&mut self.0, new_width, new_height) }
	}
	/// Wrapper for ImageResizeNN
	///
	/// Resize image (Nearest-Neighbor scaling algorithm)
	pub fn resize_nn(&mut self, new_width: i32, new_height: i32) {
		unsafe { raylib_ffi::ImageResizeNN(&mut self.0, new_width, new_height) }
	}
	/// Wrapper for ImageResizeCanvas
	///
	/// Resize canvas and fill with color
	pub fn resize_canvas(&mut self, new_width: i32, new_height: i32, offset_x: i32, offset_y: i32, fill: Color) {
		unsafe { raylib_ffi::ImageResizeCanvas(&mut self.0, new_width, new_height, offset_x, offset_y, fill.into()) }
	}
	/// Wrapper for ImageMipmaps
	///
	/// Compute all mipmap levels for a provided image
	pub fn mipmaps(&mut self) {
		unsafe { raylib_ffi::ImageMipmaps(&mut self.0) }
	}
	/// Wrapper for ImageDither
	///
	/// Dither image data to 16bpp or lower (Floyd-Steinberg dithering)
	pub fn dither(&mut self, rbpp: i32, gbpp: i32, bbpp: i32, abpp: i32) {
		unsafe { raylib_ffi::ImageDither(&mut self.0, rbpp, gbpp, bbpp, abpp) }
	}
	/// Wrapper for ImageFlipVertical
	///
	/// Flip image vertically
	pub fn flip_vertical(&mut self) {
		unsafe { raylib_ffi::ImageFlipVertical(&mut self.0) }
	}
	/// Wrapper for ImageFlipVertical
	///
	/// Flip image horizontally
	pub fn flip_horizontal(&mut self) {
		unsafe { raylib_ffi::ImageFlipVertical(&mut self.0) }
	}
	/// Wrapper for ImageRotate
	///
	/// Rotate image by input angle in degrees (-359 to 359)
	pub fn rotate(&mut self, degrees: i32) {
		unsafe { raylib_ffi::ImageRotate(&mut self.0, degrees) }
	}
	/// Wrapper for ImageRotateCW
	///
	/// Rotate image clockwise 90deg
	pub fn rotate_cw(&mut self) {
		unsafe { raylib_ffi::ImageRotateCW(&mut self.0) }
	}
	/// Wrapper for ImageRotateCCW
	///
	/// Rotate image counter-clockwise 90deg
	pub fn rotate_ccw(&mut self) {
		unsafe { raylib_ffi::ImageRotateCCW(&mut self.0) }
	}
	/// Wrapper for ImageColorTint
	///
	/// Modify image color: tint
	pub fn color_tint(&mut self, color: Color) {
		unsafe { raylib_ffi::ImageColorTint(&mut self.0, color.into()) }
	}
	/// Wrapper for ImageColorInvert
	///
	/// Modify image color: invert
	pub fn color_invert(&mut self) {
		unsafe { raylib_ffi::ImageColorInvert(&mut self.0) }
	}
	/// Wrapper for ImageColorGrayscale
	///
	/// Modify image color: grayscale
	pub fn color_grayscale(&mut self) {
		unsafe { raylib_ffi::ImageColorGrayscale(&mut self.0) }
	}
	/// Wrapper for ImageColorContrast
	///
	/// Modify image color: contrast (-100 to 100)
	pub fn color_contrast(&mut self, contrast: f32) {
		unsafe { raylib_ffi::ImageColorContrast(&mut self.0, contrast) }
	}
	/// Wrapper for ImageColorBrightness
	///
	/// Modify image color: brightness (-255 to 255)
	pub fn color_brightness(&mut self, brightness: i32) {
		unsafe { raylib_ffi::ImageColorBrightness(&mut self.0, brightness) }
	}
	/// Wrapper for ImageColorReplace
	///
	/// Modify image color: replace color
	pub fn color_replace(&mut self, color: Color, replace: Color) {
		unsafe { raylib_ffi::ImageColorReplace(&mut self.0, color.into(), replace.into()) }
	}
	/// Wrapper for LoadImagePalette
	///
	/// Load colors palette from image as a Color array (RGBA - 32bit)
	pub fn load_palette(&self, max_palette_size: i32) -> Vec<Color> {
		unsafe {
			let mut palette = Vec::new();
			let mut count = [0;1];
			let colors = raylib_ffi::LoadImagePalette(self.0, max_palette_size, count.as_mut_ptr());

			for i in 0..count[0] {
				palette.push(Color::from(*colors.wrapping_add(i as usize).clone()))
			}
			raylib_ffi::UnloadImagePalette(colors);

			palette
		}
	}
	/// Wrapper for GetImageAlphaBorder
	///
	/// Get image alpha border rectangle
	pub fn get_alpha_border(&self, threshold: f32) -> Rectangle {
		unsafe { Rectangle::from(raylib_ffi::GetImageAlphaBorder(self.0, threshold)) }
	}
	/// Wrapper for GetImageColor
	///
	/// Get image pixel color at (x, y) position
	pub fn get_color(&self, x: i32, y: i32) -> Color {
		unsafe { Color::from(raylib_ffi::GetImageColor(self.0, x, y)) }
	}

	//= Drawing
	/// Wrapper for ImageClearBackground
	///
	/// Clear image background with given color
	pub fn clear_background(&mut self, color: Color) {
		unsafe { raylib_ffi::ImageClearBackground(&mut self.0, color.into()) }
	}
	/// Wrapper for ImageDrawPixel
	///
	/// Draw pixel within an image
	pub fn draw_pixel(&mut self, pos_x: i32, pos_y: i32, color: Color) {
		unsafe { raylib_ffi::ImageDrawPixel(&mut self.0, pos_x, pos_y, color.into()) }
	}
	/// Wrapper for ImageDrawPixelV
	///
	/// Draw pixel within an image (Vector version)
	pub fn draw_pixel_v(&mut self, position: Vector2, color: Color) {
		unsafe { raylib_ffi::ImageDrawPixelV(&mut self.0, position.into(), color.into()) }
	}
	/// Wrapper for ImageDrawLine
	///
	/// Draw line within an image
	pub fn draw_line(&mut self, start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: Color) {
		unsafe { raylib_ffi::ImageDrawLine(&mut self.0, start_pos_x, start_pos_y, end_pos_x, end_pos_y, color.into()) }
	}
	/// Wrapper for ImageDrawLineV
	///
	/// Draw line within an image (Vector version)
	pub fn draw_line_v(&mut self, start: Vector2, end: Vector2, color: Color) {
		unsafe { raylib_ffi::ImageDrawLineV(&mut self.0, start.into(), end.into(), color.into()) }
	}
	/// Wrapper for ImageDrawCircle
	///
	/// Draw a filled circle within an image
	pub fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: i32, color: Color) {
		unsafe { raylib_ffi::ImageDrawCircle(&mut self.0, center_x, center_y, radius, color.into()) }
	}
	/// Wrapper for ImageDrawCircleV
	///
	/// Draw a filled circle within an image (Vector version)
	pub fn draw_circle_v(&mut self, center: Vector2, radius: i32, color: Color) {
		unsafe { raylib_ffi::ImageDrawCircleV(&mut self.0, center.into(), radius, color.into()) }
	}
	/// Wrapper for ImageDrawCircleLines
	///
	/// Draw circle outline within an image
	pub fn draw_circle_lines(&mut self, center_x: i32, center_y: i32, radius: i32, color: Color) {
		unsafe { raylib_ffi::ImageDrawCircleLines(&mut self.0, center_x, center_y, radius, color.into()) }
	}
	/// Wrapper for ImageDrawCircleLinesV
	///
	/// Draw circle outline within an image (Vector version)
	pub fn draw_circle_lines_v(&mut self, center: Vector2, radius: i32, color: Color) {
		unsafe { raylib_ffi::ImageDrawCircleLinesV(&mut self.0, center.into(), radius, color.into()) }
	}
	/// Wrapper for ImageDrawRectangle
	///
	/// Draw rectangle within an image
	pub fn draw_rectangle(&mut self, pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
		unsafe { raylib_ffi::ImageDrawRectangle(&mut self.0, pos_x, pos_y, width, height, color.into()) }
	}
	/// Wrapper for ImageDrawRectangleV
	///
	/// Draw rectangle within an image (Vector version)
	pub fn draw_rectangle_v(&mut self, position: Vector2, size: Vector2, color: Color) {
		unsafe { raylib_ffi::ImageDrawRectangleV(&mut self.0, position.into(), size.into(), color.into()) }
	}
	/// Wrapper for ImageDrawRectangleRec
	///
	/// Draw rectangle within an image
	pub fn draw_rectangle_rec(&mut self, rec: Rectangle, color: Color) {
		unsafe { raylib_ffi::ImageDrawRectangleRec(&mut self.0, rec.into(), color.into()) }
	}
	/// Wrapper for ImageDrawRectangleLines
	///
	/// Draw rectangle lines within an image
	pub fn draw_rectangle_lines(&mut self, rec: Rectangle, thick: i32, color: Color) {
		unsafe { raylib_ffi::ImageDrawRectangleLines(&mut self.0, rec.into(), thick, color.into()) }
	}
	/// Wrapper for ImageDraw
	///
	/// Draw a source image within a destination image (tint applied to source)
	pub fn draw(&mut self, src: Image, src_rec: Rectangle, dst_rec: Rectangle, color: Color) {
		unsafe { raylib_ffi::ImageDraw(&mut self.0, src.0, src_rec.into(), dst_rec.into(), color.into()) }
	}
	/// Wrapper for ImageDrawText
	///
	/// Draw text (using default font) within an image (destination)
	pub fn draw_text(&mut self, text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
		unsafe { raylib_ffi::ImageDrawText(&mut self.0, rl_str!(text), pos_x, pos_y, font_size, color.into()) }
	}
	/// Wrapper for ImageDrawTextEx
	///
	/// Draw text (custom sprite font) within an image (destination)
	pub fn draw_text_ex(&mut self, font: Font, text: &str, position: Vector2, font_size: f32, spacing: f32, tint: Color) {
		unsafe { raylib_ffi::ImageDrawTextEx(&mut self.0, font.into(), rl_str!(text), position.into(), font_size, spacing, tint.into()) }
	}

	//= Conversion
	/// Wrapper for LoadTextureFromImage
	///
	/// Load texture from image data
	pub fn texture(&self) -> Texture {
		unsafe { Texture::from(raylib_ffi::LoadTextureFromImage(self.0)) }
	}
	/// Wrapper for LoadTextureCubemap
	///
	/// Load cubemap from image, multiple image cubemap layouts supported
	// TODO: fix layout enum
	pub fn cubemap(&self, layout: i32) -> Texture {
		unsafe { Texture::from(raylib_ffi::LoadTextureCubemap(self.0, layout)) }
	}

}


//= Procedures
