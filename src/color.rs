

use crate::vectors::*;


/// Color
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}
impl Into<Vector3> for Color {
	fn into(self) -> Vector3 {
		let r = self.r as f32 / 255.0;
		let g = self.g as f32 / 255.0;
		let b = self.b as f32 / 255.0;
		let cmax = r.max(g.max(b));
		let cmin = r.min(g.min(b));
		let delta = cmax - cmin;

		let hue = if delta == 0.0 { 0.0 }
			else if cmax == r { (60.0 * (((g - b) / delta) + 360.0)) % 360.0 }
			else if cmax == g { (60.0 * (((b - r) / delta) + 120.0)) % 360.0 }
			else if cmax == b { (60.0 * (((r - g) / delta) + 240.0)) % 360.0 }
			else { -1.0 };
		let sat = if cmax == 0.0 { 0.0 } else { delta / cmax };

		Vector3 {
			x: hue,
			y: sat,
			z: cmax,
		}
	}
}
impl From<Vector3> for Color {
	fn from(value: Vector3) -> Self {
		unsafe { Self::from(ColorFromHSV(value.x, value.y, value.z)) }
	}
}
impl Into<Vector4> for Color {
	fn into(self) -> Vector4 {
		Vector4 {
			x: self.r as f32 / 255.0,
			y: self.g as f32 / 255.0,
			z: self.b as f32 / 255.0,
			w: self.a as f32 / 255.0,
		}
	}
}
impl From<Vector4> for Color {
	fn from(value: Vector4) -> Self {
		unsafe { Self::from(ColorFromNormalized(value.into())) }
	}
}


pub const LIGHTGRAY:	Color = Color {r: 200,g: 200,b: 200,a: 255};
pub const GRAY:			Color = Color {r: 130,g: 130,b: 130,a: 255};
pub const DARKGRAY:		Color = Color {r: 80,g: 80,b: 80,a: 255};
pub const YELLOW:		Color = Color {r: 253,g: 249,b: 0,a: 255};
pub const GOLD:			Color = Color {r: 255,g: 203,b: 0,a: 255};
pub const ORANGE:		Color = Color {r: 255,g: 161,b: 0,a: 255};
pub const PINK:			Color = Color {r: 255,g: 109,b: 194,a: 255};
pub const RED:			Color = Color {r: 230,g: 41,b: 55,a: 255};
pub const MAROON:		Color = Color {r: 190,g: 33,b: 55,a: 255};
pub const GREEN:		Color = Color {r: 0,g: 228,b: 48,a: 255};
pub const LIME:			Color = Color {r: 0,g: 158,b: 47,a: 255};
pub const DARKGREEN:	Color = Color {r: 0,g: 117,b: 44,a: 255};
pub const SKYBLUE:		Color = Color {r: 102,g: 191,b: 255,a: 255};
pub const BLUE:			Color = Color {r: 0,g: 121,b: 241,a: 255};
pub const DARKBLUE:		Color = Color {r: 0,g: 82,b: 172,a: 255};
pub const PURPLE:		Color = Color {r: 200,g: 122,b: 255,a: 255};
pub const VIOLET:		Color = Color {r: 135,g: 60,b: 190,a: 255};
pub const DARKPURPLE:	Color = Color {r: 112,g: 31,b: 126,a: 255};
pub const BEIGE:		Color = Color {r: 211,g: 176,b: 131,a: 255};
pub const BROWN:		Color = Color {r: 127,g: 106,b: 79,a: 255};
pub const DARKBROWN:	Color = Color {r: 76,g: 63,b: 47,a: 255};
pub const WHITE:		Color = Color {r: 255,g: 255,b: 255,a: 255};
pub const BLACK:		Color = Color {r: 0,g: 0,b: 0,a: 255};
pub const BLANK:		Color = Color {r: 0,g: 0,b: 0,a: 0};
pub const MAGENTA:		Color = Color {r: 255,g: 0,b: 255,a: 255};
pub const RAYWHITE:		Color = Color {r: 245,g: 245,b: 245,a: 255};
pub const PALETTE_30:	Color = Color {r: 48,g: 56,b: 67,a: 255};


impl Color {
	
	//= Manipulations
	/// Wrapper for Fade
	///
	/// Get color with alpha applied, alpha goes from 0.0f to 1.0f
	pub fn fade(self, alpha: f32) -> Self {
		unsafe { Fade(self, alpha) }
	}
	/// Wrapper for ColorToInt
	///
	/// Get hexadecimal value for a Color
	pub fn to_int(self) -> i32 {
		unsafe { ColorToInt(self) }
	}
	/// Wrapper for ColorTint
	///
	/// Get color multiplied with another color
	pub fn tint(self, tint: Color) -> Self {
		unsafe { ColorTint(self, tint) }
	}
	/// Wrapper for ColorBrightness
	///
	/// Get color with brightness correction, brightness factor goes from -1.0f to 1.0f
	pub fn brightness(self, factor: f32) -> Self {
		unsafe { ColorBrightness(self, factor) }
	}
	/// Wrapper for ColorContrast
	///
	/// Get color with contrast correction, contrast values between -1.0f and 1.0fGet color with contrast correction, contrast values between -1.0f and 1.0f
	pub fn contrast(self, contrast: f32) -> Self {
		unsafe { ColorContrast(self, contrast) }
	}
	/// Wrapper for ColorAlpha
	///
	/// Get color with alpha applied, alpha goes from 0.0f to 1.0f
	pub fn alpha(self, alpha: f32) -> Self {
		unsafe { ColorAlpha(self, alpha) }
	}
	/// Wrapper for ColorAlphaBlend
	///
	/// Get src alpha-blended into dst color with tint
	pub fn alpha_blend(self, src: Color, tint: Color) -> Self {
		unsafe { ColorAlphaBlend(self, src, tint) }
	}
	/// Wrapper for GetColor
	///
	/// Get Color structure from hexadecimal value
	pub fn hex(hex_value: u32) -> Self {
		unsafe { GetColor(hex_value) }
	}

}


extern "C" { fn ColorFromNormalized(normalized: Vector4) -> Color; }
extern "C" { fn ColorFromHSV(hue: f32, saturation: f32, value: f32) -> Color; }
extern "C" { fn Fade(color: Color, alpha: f32) -> Color; }
extern "C" { fn ColorToInt(color: Color) -> i32; }
extern "C" { fn ColorTint(color: Color, tint: Color) -> Color; }
extern "C" { fn ColorBrightness(color: Color, factor: f32) -> Color; }
extern "C" { fn ColorContrast(color: Color, contrast: f32) -> Color; }
extern "C" { fn ColorAlpha(color: Color, alpha: f32) -> Color; }
extern "C" { fn ColorAlphaBlend(dst: Color, src: Color, tint: Color) -> Color; }
extern "C" { fn GetColor(hexValue: u32) -> Color; }