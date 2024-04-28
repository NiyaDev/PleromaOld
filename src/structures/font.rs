

use crate::{
	rl_str,
	structures::{
		color::*,
		image::*,
		rectangle::*,
		texture::*,
		vectors::*,
	}
};


/// Font
pub struct Font(pub FontRl);

/// Raw raylib structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
pub struct GlyphInfo {
	pub value:		i32,
    pub offset_x:	i32,
    pub offset_y:	i32,
    pub advance_x:	i32,
    pub image:		ImageRl,
}


impl Font {

	//= Creation
	/// Wrapper for GetFontDefault
	pub fn default() -> Self {
		unsafe { Self(GetFontDefault()) }
	}
	/// Wrapper for LoadFont
	pub fn load(filename: &str) -> Self {
		unsafe { Self(LoadFont(rl_str!(filename))) }
	}
	/// Wrapper for LoadFontEx
	pub fn load_ex(filename: &str, font_size: i32, codepoints: Vec<i32>) -> Self {
		unsafe { Self(LoadFontEx(rl_str!(filename), font_size, codepoints.as_ptr(), codepoints.len() as i32)) }
	}
	/// Wrapper for LoadFontFromImage
	pub fn load_from_image(image: Image, key: Color, first_char: i32) -> Self {
		unsafe { Self(LoadFontFromImage(image.0, key, first_char)) }
	}
	/// Wrapper for LoadFontFromMemory
	pub fn load_from_memory(file_type: &str, file_data: Vec<u8>, font_size: i32, codepoints: Vec<i32>) -> Self {
		unsafe { Self(LoadFontFromMemory(
			rl_str!(file_type),
			file_data.as_ptr(), file_data.len() as i32,
			font_size,
			codepoints.as_ptr(), codepoints.len() as i32,
		)) }
	}
	/// Wrapper for 
	pub fn unload(&mut self) {
		unsafe { UnloadFont(self.0) }
	}

	//= Manipulation
	pub fn draw(&self, text: &str, position: Vector2, font_size: f32, spacing: f32, tint: Color) {
		unsafe {
			DrawTextEx(
				self.0,
				rl_str!(text),
				position,
				font_size,
				spacing,
				tint,
			)
		}
	}

}


extern "C" { fn GetFontDefault() -> FontRl; }
extern "C" { fn LoadFont(fileName: *const i8) -> FontRl; }
extern "C" { fn LoadFontEx(fileName: *const i8, fontSize: i32, codepoints: *const i32, codepointCount: i32) -> FontRl; }
extern "C" { fn LoadFontFromImage(image: ImageRl, key: Color, firstChar: i32) -> FontRl; }
extern "C" { fn LoadFontFromMemory(fileType: *const i8, fileData: *const u8, dataSize: i32, fontSize: i32, codepoints: *const i32, codepointCount: i32) -> FontRl; }
extern "C" { fn UnloadFont(font: FontRl); }

extern "C" { fn DrawTextEx(font: FontRl, text: *const i8, position: Vector2, font_size: f32, spacing: f32, tint: Color); }

// Font loading/unloading functions
//bool IsFontReady(Font font);                                                          // Check if a font is ready
//GlyphInfo *LoadFontData(const unsigned char *fileData, int dataSize, int fontSize, int *codepoints, int codepointCount, int type); // Load font data for further use
//Image GenImageFontAtlas(const GlyphInfo *glyphs, Rectangle **glyphRecs, int glyphCount, int fontSize, int padding, int packMethod); // Generate image font atlas using chars info
//void UnloadFontData(GlyphInfo *glyphs, int glyphCount);                               // Unload font chars info data (RAM)
//void UnloadFont(Font font);                                                           // Unload font from GPU memory (VRAM)
//bool ExportFontAsCode(Font font, const char *fileName);

// Text drawing functions
//void DrawFPS(int posX, int posY);                                                     // Draw current FPS
//void DrawTextEx(Font font, const char *text, Vector2 position, float fontSize, float spacing, Color tint); // Draw text using font and additional parameters
//void DrawTextPro(Font font, const char *text, Vector2 position, Vector2 origin, float rotation, float fontSize, float spacing, Color tint); // Draw text using Font and pro parameters (rotation)
//void DrawTextCodepoint(Font font, int codepoint, Vector2 position, float fontSize, Color tint); // Draw one character (codepoint)
//void DrawTextCodepoints(Font font, const int *codepoints, int codepointCount, Vector2 position, float fontSize, float spacing, Color tint); // Draw multiple character (codepoint)

// Text font info functions
//void SetTextLineSpacing(int spacing);                                                 // Set vertical line spacing when drawing with line-breaks
//int MeasureText(const char *text, int fontSize);                                      // Measure string width for default font
//Vector2 MeasureTextEx(Font font, const char *text, float fontSize, float spacing);    // Measure string size for Font
//int GetGlyphIndex(Font font, int codepoint);                                          // Get glyph index position in font for a codepoint (unicode character), fallback to '?' if not found
//GlyphInfo GetGlyphInfo(Font font, int codepoint);                                     // Get glyph font info data for a codepoint (unicode character), fallback to '?' if not found
//Rectangle GetGlyphAtlasRec(Font font, int codepoint);                                 // Get glyph rectangle in font atlas for a codepoint (unicode character), fallback to '?' if not found

// Text codepoints management functions (unicode characters)
//char *LoadUTF8(const int *codepoints, int length);                // Load UTF-8 text encoded from codepoints array
//void UnloadUTF8(char *text);                                      // Unload UTF-8 text encoded from codepoints array
//int *LoadCodepoints(const char *text, int *count);                // Load all codepoints from a UTF-8 text string, codepoints count returned by parameter
//void UnloadCodepoints(int *codepoints);                           // Unload codepoints data from memory
//int GetCodepointCount(const char *text);                          // Get total number of codepoints in a UTF-8 encoded string
//int GetCodepoint(const char *text, int *codepointSize);           // Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
//int GetCodepointNext(const char *text, int *codepointSize);       // Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
//int GetCodepointPrevious(const char *text, int *codepointSize);   // Get previous codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
//const char *CodepointToUTF8(int codepoint, int *utf8Size);        // Encode one codepoint into UTF-8 byte array (array length returned as parameter)

// Text strings management functions (no UTF-8 strings, only byte chars)
// NOTE: Some strings allocate memory internally for returned strings, just be careful!
//int TextCopy(char *dst, const char *src);                                             // Copy one string to another, returns bytes copied
//bool TextIsEqual(const char *text1, const char *text2);                               // Check if two text string are equal
//unsigned int TextLength(const char *text);                                            // Get text length, checks for '\0' ending
//const char *TextFormat(const char *text, ...);                                        // Text formatting with variables (sprintf() style)
//const char *TextSubtext(const char *text, int position, int length);                  // Get a piece of a text string
//char *TextReplace(char *text, const char *replace, const char *by);                   // Replace text string (WARNING: memory must be freed!)
//char *TextInsert(const char *text, const char *insert, int position);                 // Insert text in a position (WARNING: memory must be freed!)
//const char *TextJoin(const char **textList, int count, const char *delimiter);        // Join text strings with delimiter
//const char **TextSplit(const char *text, char delimiter, int *count);                 // Split text into multiple strings
//void TextAppend(char *text, const char *append, int *position);                       // Append text at specific position and move cursor!
//int TextFindIndex(const char *text, const char *find);                                // Find first text occurrence within a string
//const char *TextToUpper(const char *text);                      // Get upper case version of provided string
//const char *TextToLower(const char *text);                      // Get lower case version of provided string
//const char *TextToPascal(const char *text);                     // Get Pascal case notation version of provided string
//int TextToInteger(const char *text);                            // Get integer value from text (negative values not supported)