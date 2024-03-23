

use crate::{image::*, rectangle::*, texture::*};


/// Font
pub struct Font(pub FontRl);

/// Raw raylib structure
#[repr(C)]
pub struct FontRl {
	pub base_size:		i32,
    pub glyph_count:	i32,
    pub glyph_padding:	i32,
    pub texture:		TextureRl,
    pub recs:	   *mut Rectangle,
    pub glyphs:	   *mut GlyphInfo,
}

/// Info on each symbol
#[repr(C)]
pub struct GlyphInfo {
	pub value:		i32,
    pub offset_x:	i32,
    pub offset_y:	i32,
    pub advance_x:	i32,
    pub image:		ImageRl,
}