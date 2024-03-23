

/// Pixel format wrapper
#[derive(Debug, Clone, Copy)]
pub enum PixelFormat {
	Grayscale,
    GrayAlpha,
    R5g6b5,
    R8g8b8,
    R5g5b5a1,
    R4g4b4a4,
    R8g8b8a8,
    R32,
    R32g32b32,
    R32g32b32a32,
    R16,
    R16g16b16,
    R16g16b16a16,
    Dxt1Rgb,
    Dxt1Rgba,
    Dxt3Rgba,
    Dxt5Rgba,
    Etc1Rgb,
    Etc2Rgb,
    Etc2EacRgba,
    PvrtRgb,
    PvrtRgba,
    Astc4x4Rgba,
    Astc8x8Rgba,
	Unknown,
}
impl From<i32> for PixelFormat {
	fn from(value: i32) -> Self {
		match value {
			0 => Self::Grayscale,
			1 => Self::GrayAlpha,
			2 => Self::R5g6b5,
			3 => Self::R8g8b8,
			4 => Self::R5g5b5a1,
			5 => Self::R4g4b4a4,
			6 => Self::R8g8b8a8,
			7 => Self::R32,
			8 => Self::R32g32b32,
			9 => Self::R32g32b32a32,
			10 => Self::R16,
			11 => Self::R16g16b16,
			12 => Self::R16g16b16a16,
			13 => Self::Dxt1Rgb,
			14 => Self::Dxt1Rgba,
			15 => Self::Dxt3Rgba,
			16 => Self::Dxt5Rgba,
			17 => Self::Etc1Rgb,
			18 => Self::Etc2Rgb,
			19 => Self::Etc2EacRgba,
			20 => Self::PvrtRgb,
			21 => Self::PvrtRgba,
			22 => Self::Astc4x4Rgba,
			23 => Self::Astc8x8Rgba,
			_ => Self::Unknown,
		}
	}
}


impl PixelFormat {
	
	//= Manipulations
	/// Wrapper for GetPixelDataSize
	///
	/// Get pixel data size in bytes for certain format
	pub fn fade(self, width: i32, height: i32) -> i32 {
		unsafe { GetPixelDataSize(width, height, self as i32) }
	}

}


extern "C" { fn GetPixelDataSize(width: i32, height: i32, format: i32) -> i32; }