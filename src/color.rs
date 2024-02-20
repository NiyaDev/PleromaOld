

//= Imports


//= Structures and Enumerations

/// Color wrapper
#[derive(Debug)]
pub struct Color(raylib_ffi::Color);
impl PartialEq<Self> for Color {
    fn eq(&self, other: &Self) -> bool {
		self.0.r == other.0.r &&
		self.0.g == other.0.g &&
		self.0.b == other.0.b &&
		self.0.a == other.0.a
    }
}
impl Into<raylib_ffi::Color> for Color {
	fn into(self) -> raylib_ffi::Color {
		self.0
	}
}
impl From<raylib_ffi::Color> for Color {
	fn from(value: raylib_ffi::Color) -> Self {
		Self{0: value}
	}
}


//= Constants

pub const LIGHTGRAY: Color = Color {0: raylib_ffi::Color {r: 200,g: 200,b: 200,a: 255} };
pub const GRAY: Color = Color {0: raylib_ffi::Color {r: 130,g: 130,b: 130,a: 255} };
pub const DARKGRAY: Color = Color {0: raylib_ffi::Color {r: 80,g: 80,b: 80,a: 255} };
pub const YELLOW: Color = Color {0: raylib_ffi::Color {r: 253,g: 249,b: 0,a: 255} };
pub const GOLD: Color = Color {0: raylib_ffi::Color {r: 255,g: 203,b: 0,a: 255} };
pub const ORANGE: Color = Color {0: raylib_ffi::Color {r: 255,g: 161,b: 0,a: 255} };
pub const PINK: Color = Color {0: raylib_ffi::Color {r: 255,g: 109,b: 194,a: 255} };
pub const RED: Color = Color {0: raylib_ffi::Color {r: 230,g: 41,b: 55,a: 255} };
pub const MAROON: Color = Color {0: raylib_ffi::Color {r: 190,g: 33,b: 55,a: 255} };
pub const GREEN: Color = Color {0: raylib_ffi::Color {r: 0,g: 228,b: 48,a: 255} };
pub const LIME: Color = Color {0: raylib_ffi::Color {r: 0,g: 158,b: 47,a: 255} };
pub const DARKGREEN: Color = Color {0: raylib_ffi::Color {r: 0,g: 117,b: 44,a: 255} };
pub const SKYBLUE: Color = Color {0: raylib_ffi::Color {r: 102,g: 191,b: 255,a: 255} };
pub const BLUE: Color = Color {0: raylib_ffi::Color {r: 0,g: 121,b: 241,a: 255} };
pub const DARKBLUE: Color = Color {0: raylib_ffi::Color {r: 0,g: 82,b: 172,a: 255} };
pub const PURPLE: Color = Color {0: raylib_ffi::Color {r: 200,g: 122,b: 255,a: 255} };
pub const VIOLET: Color = Color {0: raylib_ffi::Color {r: 135,g: 60,b: 190,a: 255} };
pub const DARKPURPLE: Color = Color {0: raylib_ffi::Color {r: 112,g: 31,b: 126,a: 255} };
pub const BEIGE: Color = Color {0: raylib_ffi::Color {r: 211,g: 176,b: 131,a: 255} };
pub const BROWN: Color = Color {0: raylib_ffi::Color {r: 127,g: 106,b: 79,a: 255} };
pub const DARKBROWN: Color = Color {0: raylib_ffi::Color {r: 76,g: 63,b: 47,a: 255} };
pub const WHITE: Color = Color {0: raylib_ffi::Color {r: 255,g: 255,b: 255,a: 255} };
pub const BLACK: Color = Color {0: raylib_ffi::Color {r: 0,g: 0,b: 0,a: 255} };
pub const BLANK: Color = Color {0: raylib_ffi::Color {r: 0,g: 0,b: 0,a: 0} };
pub const MAGENTA: Color = Color {0: raylib_ffi::Color {r: 255,g: 0,b: 255,a: 255} };
pub const RAYWHITE: Color = Color {0: raylib_ffi::Color {r: 245,g: 245,b: 245,a: 255} };
pub const PALETTE_30: Color = Color {0: raylib_ffi::Color {r: 48,g: 56,b: 67,a: 255} };
