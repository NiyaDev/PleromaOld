

//= Imports


//= Structures and Enumerations

/// Color wrapper
#[derive(Debug)]
pub struct Color{
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}
impl PartialEq<Self> for Color {
    fn eq(&self, other: &Self) -> bool {
		self.r == other.r &&
		self.g == other.g &&
		self.b == other.b &&
		self.a == other.a
	}
}
impl Into<raylib_ffi::Color> for Color {
	fn into(self) -> raylib_ffi::Color {
		raylib_ffi::Color {
			r: self.r,
			g: self.g,
			b: self.b,
			a: self.a,
		}
	}
}
impl From<raylib_ffi::Color> for Color {
	fn from(value: raylib_ffi::Color) -> Self {
		Self{
			r: value.r,
			g: value.g,
			b: value.b,
			a: value.a,
		}
	}
}


//= Constants

pub const LIGHTGRAY: Color = Color {r: 200,g: 200,b: 200,a: 255};
pub const GRAY: Color = Color {r: 130,g: 130,b: 130,a: 255};
pub const DARKGRAY: Color = Color {r: 80,g: 80,b: 80,a: 255};
pub const YELLOW: Color = Color {r: 253,g: 249,b: 0,a: 255};
pub const GOLD: Color = Color {r: 255,g: 203,b: 0,a: 255};
pub const ORANGE: Color = Color {r: 255,g: 161,b: 0,a: 255};
pub const PINK: Color = Color {r: 255,g: 109,b: 194,a: 255};
pub const RED: Color = Color {r: 230,g: 41,b: 55,a: 255};
pub const MAROON: Color = Color {r: 190,g: 33,b: 55,a: 255};
pub const GREEN: Color = Color {r: 0,g: 228,b: 48,a: 255};
pub const LIME: Color = Color {r: 0,g: 158,b: 47,a: 255};
pub const DARKGREEN: Color = Color {r: 0,g: 117,b: 44,a: 255};
pub const SKYBLUE: Color = Color {r: 102,g: 191,b: 255,a: 255};
pub const BLUE: Color = Color {r: 0,g: 121,b: 241,a: 255};
pub const DARKBLUE: Color = Color {r: 0,g: 82,b: 172,a: 255};
pub const PURPLE: Color = Color {r: 200,g: 122,b: 255,a: 255};
pub const VIOLET: Color = Color {r: 135,g: 60,b: 190,a: 255};
pub const DARKPURPLE: Color = Color {r: 112,g: 31,b: 126,a: 255};
pub const BEIGE: Color = Color {r: 211,g: 176,b: 131,a: 255};
pub const BROWN: Color = Color {r: 127,g: 106,b: 79,a: 255};
pub const DARKBROWN: Color = Color {r: 76,g: 63,b: 47,a: 255};
pub const WHITE: Color = Color {r: 255,g: 255,b: 255,a: 255};
pub const BLACK: Color = Color {r: 0,g: 0,b: 0,a: 255};
pub const BLANK: Color = Color {r: 0,g: 0,b: 0,a: 0};
pub const MAGENTA: Color = Color {r: 255,g: 0,b: 255,a: 255};
pub const RAYWHITE: Color = Color {r: 245,g: 245,b: 245,a: 255};
pub const PALETTE_30: Color = Color {r: 48,g: 56,b: 67,a: 255};