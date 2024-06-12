

use crate::vectors::*;


/// #### Color
/// Red, Blue, Green, Alpha
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
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
	/// #### fade
	/// Apply alpha to color.
	pub fn fade(self, alpha: f32) -> Self {
		let n_alpha = if alpha < 0.0 { 0.0 } else if alpha > 1.0 { 1.0 } else { alpha };

		Self {
			r: self.r,
			g: self.g,
			b: self.b,
			a: (255.0 * n_alpha) as u8,
		}
	}
	/// #### to_int
	/// Converts color to hexadecimal representation.
	pub fn to_int(self) -> i32 {
		return ((self.r as i32) << 24) | ((self.g as i32) << 16) | ((self.b as i32) << 8) | self.a as i32;
	}
	/// #### normalize
	/// Converts color into Vector4 representation.
	pub fn normalize(self) -> Vector4 {
		Vector4 {
			x: self.r as f32 / 255.0,
			y: self.g as f32 / 255.0,
			z: self.b as f32 / 255.0,
			w: self.a as f32 / 255.0,
		}
	}
	/// #### from_normalized
	/// Converts normaized Vector4 into a color.
	pub fn from_normalized(normalized: Vector4) -> Self {
		Self {
			r: (normalized.x * 255.0) as u8,
			g: (normalized.y * 255.0) as u8,
			b: (normalized.z * 255.0) as u8,
			a: (normalized.w * 255.0) as u8,
		}
	}
	/// #### hsv
	/// Converts color into Hue, Saturation, and Value represented as a Vector3.
	pub fn hsv(self) -> Vector3 {
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
	/// #### from_hsv
	/// Converts input HSV into  it's equivalent color.
	pub fn from_hsv(value: Vector3) -> Self {
		let mut result = Self {
			r: 0,
			g: 0,
			b: 0,
			a: 255,
		};

		//* Red channel */
		let mut k = (5.0 + value.x / 60.0).rem_euclid(6.0);
		let t = 4.0 - k;
		k = if t < k   { t } else { k   };
		k = if k < 1.0 { k } else { 1.0 };
		k = if k > 0.0 { k } else { 0.0 };
		result.r = ((value.z - value.z * value.y * k) * 255.0) as u8;

		//* Green channel */
		let mut k = (3.0 + value.x / 60.0).rem_euclid(6.0);
		let t = 4.0 - k;
		k = if t < k   { t } else { k   };
		k = if k < 1.0 { k } else { 1.0 };
		k = if k > 0.0 { k } else { 0.0 };
		result.g = ((value.z - value.z * value.y * k) * 255.0) as u8;

		//* Blue channel */
		let mut k = (1.0 + value.x / 60.0).rem_euclid(6.0);
		let t = 4.0 - k;
		k = if t < k   { t } else { k   };
		k = if k < 1.0 { k } else { 1.0 };
		k = if k > 0.0 { k } else { 0.0 };
		result.b = ((value.z - value.z * value.y * k) * 255.0) as u8;

		result
	}
	/// #### tint
	/// Multiplies color by input.
	pub fn tint(self, tint: Color) -> Self {
		let mut result = self;

		let cr = tint.r as f32 / 255.0;
		let cg = tint.g as f32 / 255.0;
		let cb = tint.b as f32 / 255.0;
		let ca = tint.a as f32 / 255.0;

		result.r = ((self.r as f32 / 255.0 * cr) * 255.0) as u8;
		result.g = ((self.g as f32 / 255.0 * cg) * 255.0) as u8;
		result.b = ((self.b as f32 / 255.0 * cb) * 255.0) as u8;
		result.a = ((self.a as f32 / 255.0 * ca) * 255.0) as u8;

		result
	}
	/// #### brightness
	/// Corrects color for brightness.
	pub fn brightness(self, factor: f32) -> Self {
		let mut result = self;

		let mut n_factor = if factor > 1.0 { 1.0 } else if factor < -1.0 { -1.0 } else { factor };

		let mut red = self.r as f32;
		let mut green = self.g as f32;
		let mut blue = self.b as f32;

		if n_factor < 0.0 {
			n_factor = 1.0 + n_factor;
			red *= n_factor;
			green *= n_factor;
			blue *= n_factor;
		} else {
			red = (255.0 - red) * n_factor + red;
			green = (255.0 - green) * n_factor + green;
			blue = (255.0 - blue) * n_factor + blue;
		}

		result.r = red as u8;
		result.g = green as u8;
		result.b = blue as u8;

		result
	}
	/// #### contrast
	/// Corrects color for contrast.
	pub fn contrast(self, contrast: f32) -> Self {
		let mut result = self;

		let mut n_contrast = if contrast < -1.0 { -1.0 } else if contrast > 1.0 { 1.0 } else { contrast };
		n_contrast = 1.0 + n_contrast;
		n_contrast *= n_contrast;

		let mut pr = self.r as f32 / 255.0;
		pr -= 0.5;
		pr *= n_contrast;
		pr += 0.5;
		pr *= 255.0;
		pr = if pr < 0.0 { 0.0 } else if pr > 255.0 { 255.0 } else { pr };

		let mut pg = self.g as f32 / 255.0;
		pg -= 0.5;
		pg *= n_contrast;
		pg += 0.5;
		pg *= 255.0;
		pg = if pg < 0.0 { 0.0 } else if pg > 255.0 { 255.0 } else { pg };

		let mut pb = self.b as f32 / 255.0;
		pb -= 0.5;
		pb *= n_contrast;
		pb += 0.5;
		pb *= 255.0;
		pb = if pb < 0.0 { 0.0 } else if pb > 255.0 { 255.0 } else { pb };

		result.r = pr as u8;
		result.g = pg as u8;
		result.b = pb as u8;

		result
	}
	/// #### alpha
	/// Get color with alpha applied.
	pub fn alpha(self, alpha: f32) -> Self {
		let mut result = self;

		let n_alpha = if alpha < 0.0 { 0.0 } else if alpha > 1.0 { 1.0 } else { alpha };

		result.a = (255.0 * n_alpha) as u8;

		result
	}
	/// #### alpha_blend
	/// Get src alpha-blended into dst color with tint.
	pub fn alpha_blend(self, src: Color, tint: Color) -> Self {
		unsafe { ColorAlphaBlend(self, src, tint) }
	}
	/// #### hex
	/// Convert hexcode into color.
	pub fn hex(hex_value: u32) -> Self {
		Self {
			r: ((hex_value >> 24) & 0xFF) as u8,
			g: ((hex_value >> 16) & 0xFF) as u8,
			b: ((hex_value >> 8 ) & 0xFF) as u8,
			a: ( hex_value        & 0xFF) as u8,
		}
	}

}


extern "C" { fn ColorAlphaBlend(dst: Color, src: Color, tint: Color) -> Color; }