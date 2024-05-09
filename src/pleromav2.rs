

use crate::{rl_str, structures::{color::*, font::*, render_texture::*, resolution::*}};


pub const DEFAULT_FONT_FILENAME: &str = "data/EarlyGameBoy.ttf";
pub const DEFAULT_SCREEN_RESOLUTION: Resolution = Resolution { width: 1280, height: 720 };
pub const DEFAULT_FRAMERATE: i32 = 60;
pub const DEFAULT_BACKGROUND_COLOR: Color = DARKGRAY;



pub enum WindowFlags {
	FlagVsyncHint			= 0x00000040,
	FlagFullscreenMode		= 0x00000002,
	FlagWindowResizable		= 0x00000004,
	FlagWindowUndecorated	= 0x00000008,
	FlagWindowHidden		= 0x00000080,
	FlagWindowMinimized		= 0x00000200,
	FlagWindowMaximized		= 0x00000400,
	FlagWindowUnfocused		= 0x00000800,
	FlagWindowTopmost		= 0x00001000,
	FlagWindowAlwaysRun		= 0x00000100,
	FlagWindowTransparent	= 0x00000010,
	FlagWindowHighdpi		= 0x00002000,
	FlagWindowMousePassthrough = 0x00004000,
	FlagBorderlessWindowedMode = 0x00008000,
	FlagMsaa4xHint			= 0x00000020,
	FlagInterlacedHint		= 0x00010000,
}
impl Into<i32> for WindowFlags {
	fn into(self) -> i32 {
		match self {
			WindowFlags::FlagVsyncHint			=> 0x00000040,
			WindowFlags::FlagFullscreenMode		=> 0x00000002,
			WindowFlags::FlagWindowResizable	=> 0x00000004,
			WindowFlags::FlagWindowUndecorated	=> 0x00000008,
			WindowFlags::FlagWindowHidden		=> 0x00000080,
			WindowFlags::FlagWindowMinimized	=> 0x00000200,
			WindowFlags::FlagWindowMaximized	=> 0x00000400,
			WindowFlags::FlagWindowUnfocused	=> 0x00000800,
			WindowFlags::FlagWindowTopmost		=> 0x00001000,
			WindowFlags::FlagWindowAlwaysRun	=> 0x00000100,
			WindowFlags::FlagWindowTransparent	=> 0x00000010,
			WindowFlags::FlagWindowHighdpi		=> 0x00002000,
			WindowFlags::FlagWindowMousePassthrough => 0x00004000,
			WindowFlags::FlagBorderlessWindowedMode => 0x00008000,
			WindowFlags::FlagMsaa4xHint			=> 0x00000020,
			WindowFlags::FlagInterlacedHint		=> 0x00010000,
		}
	}
}

pub struct Pleroma {
	initialized: bool,

	//* Screen */
	screen_size: Resolution,
	render_size: Resolution,
	framerate: i32,
	window_state: i32,

	render_texture: Option<RenderTexture>,
	is_rendering: bool,

	background_color: Color,

	debug_ui_font: Font,
}

impl Default for Pleroma {
	fn default() -> Self {
		//* If chosen font exists, use it. Otherwise use Raylib default. */
		let mut font = Font::load_ex(DEFAULT_FONT_FILENAME, 20, Vec::new());
		if !font.ready() { font = Font::default() }

		//* Initialize raylib */
		unsafe {
			SetTraceLogLevel(7);
			InitWindow(
				DEFAULT_SCREEN_RESOLUTION.width,
				DEFAULT_SCREEN_RESOLUTION.height,
				rl_str!("Default"),
			);
			SetTargetFPS(DEFAULT_FRAMERATE);
		}

		//* Create structure */
		Self {
			initialized: true,

			screen_size: DEFAULT_SCREEN_RESOLUTION,
			render_size: DEFAULT_SCREEN_RESOLUTION / 2,
			framerate:   DEFAULT_FRAMERATE,
			window_state: 0,

			render_texture: None,
			is_rendering: false,

			background_color: DEFAULT_BACKGROUND_COLOR,

			debug_ui_font: font,
		}
	}
}

impl Pleroma {

	/// Close the window and unload anything necesary.
	pub fn close(&mut self) {
		//* Unload RenderTexture */
		if self.render_texture.is_some() { self.render_texture.unwrap().unload() }

		//* Close Window */
		unsafe { CloseWindow() }
	}
	/// Wrapper for IsWindowReady
	pub fn ready(&self) -> bool {
		unsafe { IsWindowReady() }
	}
	/// Wrapper for WindowShouldClose
	pub fn should_close(&mut self) -> bool {
		unsafe { WindowShouldClose() }
	}
	//= Screen
	/// Sets screen resolution
	pub fn set_resolution(&mut self, width: i32, height: i32) -> &mut Self {
		self.screen_size = Resolution { width, height };
		unsafe { SetWindowSize(width, height) }

		self
	}
	/// Sets render resolution
	pub fn set_render(&mut self, width: i32, height: i32) -> &mut Self {
		if !self.is_rendering {
			self.render_size = Resolution { width, height };

			self.update_render()
		} else {
			// DEBUG: Attempted to change screen resolution while drawing to RenderTexture
			self
		}
	}
	// TODO Go back over this and make it better
	/// Sets the windows flag
	fn set_window_flag(&mut self, flag: WindowFlags) -> &mut Self {
		unsafe { SetWindowState(flag.into()) }

		self
	}
	/// Set window to fullscreen
	pub fn set_fullscreen(&mut self) -> &mut Self {
		self.set_window_flag(WindowFlags::FlagFullscreenMode)
	}
	/// Set window to borderless window
	pub fn set_borderless(&mut self) -> &mut Self {
		self.set_window_flag(WindowFlags::FlagBorderlessWindowedMode)
	}
	/// Sets the title of the window
	pub fn set_title(&mut self, title: &str) -> &mut Self {
		unsafe { SetWindowTitle(rl_str!(title)) }

		self
	}
	//= Rendering
	//
	pub fn update_render(&mut self) -> &mut Self {
		if self.render_texture.is_some() { self.render_texture.as_mut().unwrap().unload() }
		if self.initialized { self.render_texture = Some(RenderTexture::load(self.render_size.width, self.render_size.height)) }

		self
	}

}

extern "C" { fn InitWindow(width: i32, height: i32, title: *const i8); }
extern "C" { fn CloseWindow(); }
extern "C" { fn WindowShouldClose() -> bool; }
extern "C" { fn IsWindowReady() -> bool; }
extern "C" { fn IsWindowHidden() -> bool; }
extern "C" { fn IsWindowMinimized() -> bool; }
extern "C" { fn IsWindowMaximized() -> bool; }
extern "C" { fn IsWindowFocused() -> bool; }
extern "C" { fn SetWindowState(flags: i32); }
extern "C" { fn ToggleFullscreen(); }
extern "C" { fn ToggleBorderlessWindowed(); }
extern "C" { fn GetScreenWidth() -> i32; }
extern "C" { fn GetScreenHeight() -> i32; }
extern "C" { fn SetWindowSize(width: i32, height: i32); }
extern "C" { fn BeginDrawing(); }
extern "C" { fn EndDrawing(); }
extern "C" { fn SetTargetFPS(fps: i32); }
extern "C" { fn SetTextLineSpacing(spacing: i32); }
extern "C" { fn SetWindowTitle(title: *const i8); }

extern "C" { fn SetTraceLogLevel(logLevel: i32); }