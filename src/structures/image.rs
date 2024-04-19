

use crate::{
	rl_str,
	structures::{
		color::*,
		font::*,
		pixel_format::*,
		rectangle::*,
		texture::*,
		vectors::*,
	}
};


/// Texture wrapper
#[derive(Debug, Clone, Copy)]
pub struct Image(pub ImageRl);
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

/// Raw raylib structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ImageRl {
	pub data: *mut std::os::raw::c_void,
    pub width: i32,
    pub height: i32,
    pub mipmaps: i32,
    pub format: i32,
}


impl Image {
	
	//= Loading
	/// Wrapper for LoadImage
	///
	/// Load image from file into CPU memory (RAM)
	pub fn load(filename: &str) -> Self {
		unsafe { Self(LoadImage(rl_str!(filename))) }
	}
	/// Wrapper for LoadImageRaw
	///
	/// Load image from RAW file data
	pub fn load_raw(filename: &str, width: i32, height: i32, format: PixelFormat, header_size: i32) -> Self {
		unsafe { Self(LoadImageRaw(rl_str!(filename), width, height, format as i32, header_size)) }
	}
	/// Wrapper for LoadImageSVG
	///
	/// Load image from SVG file data or string with specified size
	pub fn load_svg(filename: &str, width: i32, height: i32) -> Self {
		unsafe { Self(LoadImageSvg(rl_str!(filename), width, height)) }
	}
	/// Wrapper for LoadImageAnim
	///
	/// Load image sequence from file (frames appended to image.data)
	pub fn load_anim(filename: &str) -> (Self, i32) {
		unsafe {
			let mut frames = [0;1];
			let image = Self(LoadImageAnim(rl_str!(filename), frames.as_mut_ptr()));

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
			Self(LoadImageFromMemory(rl_str!(file_type), array, file_data.len() as i32))
		}
	}
	/// Wrapper for LoadImageFromTexture
	///
	/// Load image from GPU texture data
	pub fn load_from_texture(texture: Texture) -> Self {
		unsafe { Self(LoadImageFromTexture(texture.0)) }
	}
	/// Wrapper for LoadImageFromScreen
	///
	/// Load image from screen buffer and (screenshot)
	pub fn load_from_screen() -> Self {
		unsafe { Self(LoadImageFromScreen()) }
	}
	/// Wrapper for IsImageReady
	///
	/// Check if an image is ready
	pub fn is_image_ready(&self) -> bool {
		unsafe { IsImageReady(self.0) }
	}
	/// Wrapper for UnloadImage
	///
	/// Unload image from CPU memory (RAM)
	pub fn unload(&self) {
		unsafe { UnloadImage(self.0) }
	}
	/// Wrapper for ExportImage
	///
	/// Export image data to file, returns true on success
	pub fn export_image(&self, filename: &str) -> bool {
		unsafe { ExportImage(self.0, rl_str!(filename)) }
	}
	/// Wrapper for ExportImageToMemory
	///
	/// Export image to memory buffer
	pub fn export_image_to_memory(&self, file_type: &str) -> Vec<u8> {
		unsafe {
			let mut array = Vec::new();
			let mut length = [0;1];
			let data = ExportImageToMemory(self.0, rl_str!(file_type), length.as_mut_ptr());

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
		unsafe { ExportImageAsCode(self.0, rl_str!(file_name)) }
	}

	//= Generation
	/// Wrapper for GenImageColor
	///
	/// Generate image: plain color
	pub fn gen_color(width: i32, height: i32, color: Color) -> Self {
		unsafe { Self(GenImageColor(width, height, color)) }
	}
	/// Wrapper for GenImageGradientLinear
	///
	/// Generate image: linear gradient, direction in degrees [0..360], 0=Vertical gradient
	pub fn gen_linear_gradient(width: i32, height: i32, direction: i32, start: Color, end: Color) -> Self {
		unsafe { Self(GenImageGradientLinear(width, height, direction, start.into(), end.into())) }
	}
	/// Wrapper for GenImageGradientRadial
	///
	/// Generate image: radial gradient
	pub fn gen_radial_gradient(width: i32, height: i32, density: f32, inner: Color, outer: Color) -> Self {
		unsafe { Self(GenImageGradientRadial(width, height, density, inner.into(), outer.into())) }
	}
	/// Wrapper for GenImageGradientSquare
	///
	/// Generate image: square gradient
	pub fn gen_square_gradient(width: i32, height: i32, density: f32, inner: Color, outer: Color) -> Self {
		unsafe { Self(GenImageGradientSquare(width, height, density, inner.into(), outer.into())) }
	}
	/// Wrapper for GenImageChecked
	///
	/// Generate image: checked
	pub fn gen_checked(width: i32, height: i32, checks_x: i32, checks_y: i32, col1: Color, col2: Color) -> Self {
		unsafe { Self(GenImageChecked(width, height, checks_x, checks_y, col1.into(), col2.into())) }
	}
	/// Wrapper for GenImageWhiteNoise
	///
	/// Generate image: white noise
	pub fn gen_white_noise(width: i32, height: i32, factor: f32) -> Self {
		unsafe { Self(GenImageWhiteNoise(width, height, factor)) }
	}
	/// Wrapper for GenImagePerlinNoise
	///
	/// Generate image: perlin noise
	pub fn gen_perlin_noise(width: i32, height: i32, offset_x: i32, offset_y: i32, scale: f32) -> Self {
		unsafe { Self(GenImagePerlinNoise(width, height, offset_x, offset_y, scale)) }
	}
	/// Wrapper for GenImageCellular
	///
	/// Generate image: cellular algorithm, bigger tileSize means bigger cells
	pub fn gen_cellular(width: i32, height: i32, tile_size: i32) -> Self {
		unsafe { Self(GenImageCellular(width, height, tile_size)) }
	}
	/// Wrapper for GenImageText
	///
	/// Generate image: grayscale image from text data
	pub fn gen_text(width: i32, height: i32, text: &str) -> Self {
		unsafe { Self(GenImageText(width, height, rl_str!(text))) }
	}

	//= Manipulation
	/// Wrapper for ImageCopy
	///
	/// Create an image duplicate (useful for transformations)
	pub fn copy(&self) -> Self {
		unsafe { Self(ImageCopy(self.0)) }
	}
	/// Wrapper for ImageFromImage
	///
	/// Create an image from another image piece
	pub fn from_image(&self, rect: Rectangle) -> Self {
		unsafe { Self(ImageFromImage(self.0, rect)) }
	}
	/// Wrapper for ImageText
	///
	/// Create an image from text (default font)
	pub fn text(text: &str, font_size: i32, color: Color) -> Self {
		unsafe { Self(ImageText(rl_str!(text), font_size, color)) }
	}
	/// Wrapper for ImageTextEx
	///
	/// Create an image from text (custom sprite font)
	pub fn text_ex(font: Font, text: &str, font_size: f32, spacing: f32, tint: Color) -> Self {
		unsafe { Self(ImageTextEx(font.0, rl_str!(text), font_size, spacing, tint)) }
	}
	/// Wrapper for ImageFormat
	///
	/// Convert image data to desired format
	pub fn format(&mut self, new_format: PixelFormat) {
		unsafe { ImageFormat(&mut self.0, new_format as i32) }
	}
	/// Wrapper for ImageToPOT
	///
	/// Convert image to POT (power-of-two)
	pub fn to_pot(&mut self, fill: Color) {
		unsafe { ImageToPOT(&mut self.0, fill.into()) }
	}
	/// Wrapper for ImageCrop
	///
	/// Crop an image to a defined rectangle
	pub fn crop(&mut self, crop: Rectangle) {
		unsafe { ImageCrop(&mut self.0, crop.into()) }
	}
	/// Wrapper for ImageAlphaCrop
	///
	/// Crop image depending on alpha value
	pub fn crop_alpha(&mut self, threshold: f32) {
		unsafe { ImageAlphaCrop(&mut self.0, threshold) }
	}
	/// Wrapper for ImageAlphaClear
	///
	/// Clear alpha channel to desired color
	pub fn crop_clear(&mut self, color: Color, threshold: f32) {
		unsafe { ImageAlphaClear(&mut self.0, color, threshold) }
	}
	/// Wrapper for ImageAlphaMask
	///
	/// Apply alpha mask to image
	pub fn alpha_mask(&mut self, alpha_mask: Image) {
		unsafe { ImageAlphaMask(&mut self.0, alpha_mask.0) }
	}
	/// Wrapper for ImageAlphaPremultiply
	///
	/// Premultiply alpha channel
	pub fn alpha_premultiply(&mut self) {
		unsafe { ImageAlphaPremultiply(&mut self.0) }
	}
	/// Wrapper for ImageBlurGaussian
	///
	/// Apply Gaussian blur using a box blur approximation
	pub fn blur_gaussian(&mut self, blur_size: i32) {
		unsafe { ImageBlurGaussian(&mut self.0, blur_size) }
	}
	/// Wrapper for ImageResize
	///
	/// Resize image (Bicubic scaling algorithm)
	pub fn resize(&mut self, new_width: i32, new_height: i32) {
		unsafe { ImageResize(&mut self.0, new_width, new_height) }
	}
	/// Wrapper for ImageResizeNN
	///
	/// Resize image (Nearest-Neighbor scaling algorithm)
	pub fn resize_nn(&mut self, new_width: i32, new_height: i32) {
		unsafe { ImageResizeNN(&mut self.0, new_width, new_height) }
	}
	/// Wrapper for ImageResizeCanvas
	///
	/// Resize canvas and fill with color
	pub fn resize_canvas(&mut self, new_width: i32, new_height: i32, offset_x: i32, offset_y: i32, fill: Color) {
		unsafe { ImageResizeCanvas(&mut self.0, new_width, new_height, offset_x, offset_y, fill) }
	}
	/// Wrapper for ImageMipmaps
	///
	/// Compute all mipmap levels for a provided image
	pub fn mipmaps(&mut self) {
		unsafe { ImageMipmaps(&mut self.0) }
	}
	/// Wrapper for ImageDither
	///
	/// Dither image data to 16bpp or lower (Floyd-Steinberg dithering)
	pub fn dither(&mut self, rbpp: i32, gbpp: i32, bbpp: i32, abpp: i32) {
		unsafe { ImageDither(&mut self.0, rbpp, gbpp, bbpp, abpp) }
	}
	/// Wrapper for ImageFlipVertical
	///
	/// Flip image vertically
	pub fn flip_vertical(&mut self) {
		unsafe { ImageFlipVertical(&mut self.0) }
	}
	/// Wrapper for ImageFlipVertical
	///
	/// Flip image horizontally
	pub fn flip_horizontal(&mut self) {
		unsafe { ImageFlipHorizontal(&mut self.0) }
	}
	/// Wrapper for ImageRotate
	///
	/// Rotate image by input angle in degrees (-359 to 359)
	pub fn rotate(&mut self, degrees: i32) {
		unsafe { ImageRotate(&mut self.0, degrees) }
	}
	/// Wrapper for ImageRotateCW
	///
	/// Rotate image clockwise 90deg
	pub fn rotate_cw(&mut self) {
		unsafe { ImageRotateCW(&mut self.0) }
	}
	/// Wrapper for ImageRotateCCW
	///
	/// Rotate image counter-clockwise 90deg
	pub fn rotate_ccw(&mut self) {
		unsafe { ImageRotateCCW(&mut self.0) }
	}
	/// Wrapper for ImageColorTint
	///
	/// Modify image color: tint
	pub fn color_tint(&mut self, color: Color) {
		unsafe { ImageColorTint(&mut self.0, color) }
	}
	/// Wrapper for ImageColorInvert
	///
	/// Modify image color: invert
	pub fn color_invert(&mut self) {
		unsafe { ImageColorInvert(&mut self.0) }
	}
	/// Wrapper for ImageColorGrayscale
	///
	/// Modify image color: grayscale
	pub fn color_grayscale(&mut self) {
		unsafe { ImageColorGrayscale(&mut self.0) }
	}
	/// Wrapper for ImageColorContrast
	///
	/// Modify image color: contrast (-100 to 100)
	pub fn color_contrast(&mut self, contrast: f32) {
		unsafe { ImageColorContrast(&mut self.0, contrast) }
	}
	/// Wrapper for ImageColorBrightness
	///
	/// Modify image color: brightness (-255 to 255)
	pub fn color_brightness(&mut self, brightness: i32) {
		unsafe { ImageColorBrightness(&mut self.0, brightness) }
	}
	/// Wrapper for ImageColorReplace
	///
	/// Modify image color: replace color
	pub fn color_replace(&mut self, color: Color, replace: Color) {
		unsafe { ImageColorReplace(&mut self.0, color, replace) }
	}
	/// Wrapper for LoadImagePalette
	///
	/// Load colors palette from image as a Color array (RGBA - 32bit)
	pub fn load_palette(&self, max_palette_size: i32) -> Vec<Color> {
		unsafe {
			let mut palette = Vec::new();
			let mut count = [0;1];
			let colors = LoadImagePalette(self.0, max_palette_size, count.as_mut_ptr());

			for i in 0..count[0] {
				palette.push(Color::from(*colors.wrapping_add(i as usize).clone()))
			}
			UnloadImagePalette(colors);

			palette
		}
	}
	/// Wrapper for GetImageAlphaBorder
	///
	/// Get image alpha border rectangle
	pub fn get_alpha_border(&self, threshold: f32) -> Rectangle {
		unsafe { GetImageAlphaBorder(self.0, threshold) }
	}
	/// Wrapper for GetImageColor
	///
	/// Get image pixel color at (x, y) position
	pub fn get_color(&self, x: i32, y: i32) -> Color {
		unsafe { GetImageColor(self.0, x, y) }
	}

	//= Drawing
	/// Wrapper for ImageClearBackground
	///
	/// Clear image background with given color
	pub fn clear_background(&mut self, color: Color) {
		unsafe { ImageClearBackground(&mut self.0, color) }
	}
	/// Wrapper for ImageDrawPixel
	///
	/// Draw pixel within an image
	pub fn draw_pixel(&mut self, pos_x: i32, pos_y: i32, color: Color) {
		unsafe { ImageDrawPixel(&mut self.0, pos_x, pos_y, color) }
	}
	/// Wrapper for ImageDrawPixelV
	///
	/// Draw pixel within an image (Vector version)
	pub fn draw_pixel_v(&mut self, position: Vector2, color: Color) {
		unsafe { ImageDrawPixelV(&mut self.0, position, color) }
	}
	/// Wrapper for ImageDrawLine
	///
	/// Draw line within an image
	pub fn draw_line(&mut self, start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: Color) {
		unsafe { ImageDrawLine(&mut self.0, start_pos_x, start_pos_y, end_pos_x, end_pos_y, color) }
	}
	/// Wrapper for ImageDrawLineV
	///
	/// Draw line within an image (Vector version)
	pub fn draw_line_v(&mut self, start: Vector2, end: Vector2, color: Color) {
		unsafe { ImageDrawLineV(&mut self.0, start, end, color) }
	}
	/// Wrapper for ImageDrawCircle
	///
	/// Draw a filled circle within an image
	pub fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: i32, color: Color) {
		unsafe { ImageDrawCircle(&mut self.0, center_x, center_y, radius, color) }
	}
	/// Wrapper for ImageDrawCircleV
	///
	/// Draw a filled circle within an image (Vector version)
	pub fn draw_circle_v(&mut self, center: Vector2, radius: i32, color: Color) {
		unsafe { ImageDrawCircleV(&mut self.0, center, radius, color) }
	}
	/// Wrapper for ImageDrawCircleLines
	///
	/// Draw circle outline within an image
	pub fn draw_circle_lines(&mut self, center_x: i32, center_y: i32, radius: i32, color: Color) {
		unsafe { ImageDrawCircleLines(&mut self.0, center_x, center_y, radius, color) }
	}
	/// Wrapper for ImageDrawCircleLinesV
	///
	/// Draw circle outline within an image (Vector version)
	pub fn draw_circle_lines_v(&mut self, center: Vector2, radius: i32, color: Color) {
		unsafe { ImageDrawCircleLinesV(&mut self.0, center, radius, color) }
	}
	/// Wrapper for ImageDrawRectangle
	///
	/// Draw rectangle within an image
	pub fn draw_rectangle(&mut self, pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
		unsafe { ImageDrawRectangle(&mut self.0, pos_x, pos_y, width, height, color.into()) }
	}
	/// Wrapper for ImageDrawRectangleV
	///
	/// Draw rectangle within an image (Vector version)
	pub fn draw_rectangle_v(&mut self, position: Vector2, size: Vector2, color: Color) {
		unsafe { ImageDrawRectangleV(&mut self.0, position, size, color) }
	}
	/// Wrapper for ImageDrawRectangleRec
	///
	/// Draw rectangle within an image
	pub fn draw_rectangle_rec(&mut self, rec: Rectangle, color: Color) {
		unsafe { ImageDrawRectangleRec(&mut self.0, rec, color) }
	}
	/// Wrapper for ImageDrawRectangleLines
	///
	/// Draw rectangle lines within an image
	pub fn draw_rectangle_lines(&mut self, rec: Rectangle, thick: i32, color: Color) {
		unsafe { ImageDrawRectangleLines(&mut self.0, rec, thick, color) }
	}
	/// Wrapper for ImageDraw
	///
	/// Draw a source image within a destination image (tint applied to source)
	pub fn draw(&mut self, src: Image, src_rec: Rectangle, dst_rec: Rectangle, color: Color) {
		unsafe { ImageDraw(&mut self.0, src.0, src_rec, dst_rec, color) }
	}
	/// Wrapper for ImageDrawText
	///
	/// Draw text (using default font) within an image (destination)
	pub fn draw_text(&mut self, text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
		unsafe { ImageDrawText(&mut self.0, rl_str!(text), pos_x, pos_y, font_size, color) }
	}
	/// Wrapper for ImageDrawTextEx
	///
	/// Draw text (custom sprite font) within an image (destination)
	pub fn draw_text_ex(&mut self, font: Font, text: &str, position: Vector2, font_size: f32, spacing: f32, tint: Color) {
		unsafe { ImageDrawTextEx(&mut self.0, font.0, rl_str!(text), position, font_size, spacing, tint) }
	}

	//= Conversion
	/// Wrapper for LoadTextureFromImage
	///
	/// Load texture from image data
	pub fn texture(&self) -> Texture {
		unsafe { Texture(LoadTextureFromImage(self.0), WHITE) }
	}
	/// Wrapper for LoadTextureCubemap
	///
	/// Load cubemap from image, multiple image cubemap layouts supported
	// TODO: fix layout enum
	pub fn cubemap(&self, layout: i32) -> Texture {
		unsafe { Texture(LoadTextureCubemap(self.0, layout), WHITE) }
	}

}


extern "C" { fn LoadImage(fileName: *const i8) -> ImageRl; }
extern "C" { fn LoadImageRaw( fileName: *const i8, width: i32, height: i32, format: i32, header_size: i32) -> ImageRl; }
extern "C" { fn LoadImageSvg(fileNameOrString: *const i8, width: i32, height: i32) -> ImageRl; }
extern "C" { fn LoadImageAnim(fileName: *const i8, frames: *mut i32) -> ImageRl; }
extern "C" { fn LoadImageFromMemory(fileType: *const i8, fileData: *const u8, dataSize: i32) -> ImageRl; }
extern "C" { fn LoadImageFromTexture(texture: TextureRl) -> ImageRl; }
extern "C" { fn LoadImageFromScreen() -> ImageRl; }
extern "C" { fn IsImageReady(image: ImageRl) -> bool; }
extern "C" { fn UnloadImage(image: ImageRl); }
extern "C" { fn ExportImage(image: ImageRl, fileName: *const i8) -> bool; }
extern "C" { fn ExportImageToMemory(image: ImageRl, fileType: *const i8, fileSize: *mut i32) -> *mut u8; }
extern "C" { fn ExportImageAsCode(image: ImageRl, fileName: *const i8) -> bool; }
extern "C" { fn GenImageColor(width: i32, height: i32, color: Color) -> ImageRl; }
extern "C" { fn GenImageGradientLinear(width: i32, height: i32, direction: i32, start: Color, end: Color,) -> ImageRl; }
extern "C" { fn GenImageGradientRadial(width: i32, height: i32, density: f32, inner: Color, outer: Color) -> ImageRl; }
extern "C" { fn GenImageGradientSquare(width: i32, height: i32, density: f32, inner: Color, outer: Color) -> ImageRl; }
extern "C" { fn GenImageChecked(width: i32, height: i32, checksX: i32, checksY: i32, col1: Color, col2: Color) -> ImageRl; }
extern "C" { fn GenImageWhiteNoise(width: i32, height: i32, factor: f32) -> ImageRl; }
extern "C" { fn GenImagePerlinNoise(width: i32, height: i32, offsetX: i32, offsetY: i32, scale: f32,) -> ImageRl; }
extern "C" { fn GenImageCellular(width: i32, height: i32, tileSize: i32) -> ImageRl; }
extern "C" { fn GenImageText(width: i32, height: i32, text: *const i8) -> ImageRl; }
extern "C" { fn ImageCopy(image: ImageRl) -> ImageRl; }
extern "C" { fn ImageFromImage(image: ImageRl, rec: Rectangle) -> ImageRl; }
extern "C" { fn ImageText(text: *const i8, fontSize: i32, color: Color) -> ImageRl; }
extern "C" { fn ImageTextEx(font: FontRl, text: *const i8, fontSize: f32, spacing: f32, tint: Color) -> ImageRl; }
extern "C" { fn ImageFormat(image: *mut ImageRl, newFormat: i32); }
extern "C" { fn ImageToPOT(image: *mut ImageRl, fill: Color); }
extern "C" { fn ImageCrop(image: *mut ImageRl, crop: Rectangle); }
extern "C" { fn ImageAlphaCrop(image: *mut ImageRl, threshold: f32); }
extern "C" { fn ImageAlphaClear(image: *mut ImageRl, color: Color, threshold: f32); }
extern "C" { fn ImageAlphaMask(image: *mut ImageRl, alphaMask: ImageRl); }
extern "C" { fn ImageAlphaPremultiply(image: *mut ImageRl); }
extern "C" { fn ImageBlurGaussian(image: *mut ImageRl, blurSize: i32); }
extern "C" { fn ImageResize(image: *mut ImageRl, newWidth: i32, newHeight: i32); }
extern "C" { fn ImageResizeNN(image: *mut ImageRl, newWidth: i32, newHeight: i32); }
extern "C" { fn ImageResizeCanvas(image: *mut ImageRl, newWidth: i32, newHeight: i32, offsetX: i32, offsetY: i32, fill: Color); }
extern "C" { fn ImageMipmaps(image: *mut ImageRl); }
extern "C" { fn ImageDither(image: *mut ImageRl, rBpp: i32, gBpp: i32, bBpp: i32, aBpp: i32); }
extern "C" { fn ImageFlipVertical(image: *mut ImageRl); }
extern "C" { fn ImageFlipHorizontal(image: *mut ImageRl); }
extern "C" { fn ImageRotate(image: *mut ImageRl, degrees: i32); }
extern "C" { fn ImageRotateCW(image: *mut ImageRl); }
extern "C" { fn ImageRotateCCW(image: *mut ImageRl); }
extern "C" { fn ImageColorTint(image: *mut ImageRl, color: Color); }
extern "C" { fn ImageColorInvert(image: *mut ImageRl); }
extern "C" { fn ImageColorGrayscale(image: *mut ImageRl); }
extern "C" { fn ImageColorContrast(image: *mut ImageRl, contrast: f32); }
extern "C" { fn ImageColorBrightness(image: *mut ImageRl, brightness: i32); }
extern "C" { fn ImageColorReplace(image: *mut ImageRl, color: Color, replace: Color); }
extern "C" { fn LoadImagePalette(image: ImageRl, maxPaletteSize: i32, colorCount: *mut i32) -> *mut Color; }
extern "C" { fn UnloadImagePalette(colors: *mut Color); }
extern "C" { fn GetImageAlphaBorder(image: ImageRl, threshold: f32) -> Rectangle; }
extern "C" { fn GetImageColor(image: ImageRl, x: i32, y: i32) -> Color; }
extern "C" { fn ImageClearBackground(dst: *mut ImageRl, color: Color); }
extern "C" { fn ImageDrawPixel(dst: *mut ImageRl, posX: i32, posY: i32, color: Color); }
extern "C" { fn ImageDrawPixelV(dst: *mut ImageRl, position: Vector2, color: Color); }
extern "C" { fn ImageDrawLine(dst: *mut ImageRl, startPosX: i32, startPosY: i32, endPosX: i32, endPosY: i32, color: Color); }
extern "C" { fn ImageDrawLineV(dst: *mut ImageRl, start: Vector2, end: Vector2, color: Color); }
extern "C" { fn ImageDrawCircle(dst: *mut ImageRl, centerX: i32, centerY: i32, radius: i32, color: Color); }
extern "C" { fn ImageDrawCircleV(dst: *mut ImageRl, center: Vector2, radius: i32, color: Color); }
extern "C" { fn ImageDrawCircleLines(dst: *mut ImageRl, centerX: i32, centerY: i32, radius: i32, color: Color); }
extern "C" { fn ImageDrawCircleLinesV(dst: *mut ImageRl, center: Vector2, radius: i32, color: Color); }
extern "C" { fn ImageDrawRectangle(dst: *mut ImageRl, posX: i32, posY: i32, width: i32, height: i32, color: Color); }
extern "C" { fn ImageDrawRectangleV(dst: *mut ImageRl, position: Vector2, size: Vector2, color: Color); }
extern "C" { fn ImageDrawRectangleRec(dst: *mut ImageRl, rec: Rectangle, color: Color); }
extern "C" { fn ImageDrawRectangleLines(dst: *mut ImageRl, rec: Rectangle, thick: i32, color: Color); }
extern "C" { fn ImageDraw(dst: *mut ImageRl, src: ImageRl, srcRec: Rectangle, dstRec: Rectangle, tint: Color); }
extern "C" { fn ImageDrawText(dst: *mut ImageRl, text: *const i8, posX: i32, posY: i32, fontSize: i32, color: Color); }
extern "C" { fn ImageDrawTextEx(dst: *mut ImageRl, font: FontRl, text: *const i8, position: Vector2, fontSize: f32, spacing: f32, tint: Color); }
extern "C" { fn LoadTextureFromImage(image: ImageRl) -> TextureRl; }
extern "C" { fn LoadTextureCubemap(image: ImageRl, layout: i32) -> TextureRl; }