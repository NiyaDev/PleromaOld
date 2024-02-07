

//= Allows


//= Imports
use std::{fmt::Display, ops::BitAnd};
use current_platform::CURRENT_PLATFORM;

pub mod data_structures;
use crate::{*, vectors::*, matrix::*, logging::*, platform::desktop::*, tracelog, flags::WindowFlags::*};


//= Constants
pub const MAX_KEYBOARD_KEYS: usize = 512;
pub const MAX_KEY_PRESSED_QUEUE: usize = 16;
pub const MAX_CHAR_PRESSED_QUEUE: usize = 16;
pub const MAX_MOUSE_BUTTONS: usize = 8;
pub const MAX_GAMEPADS: usize = 4;
pub const MAX_GAMEPAD_BUTTONS: usize = 32;
pub const MAX_GAMEPAD_AXIS: usize = 8;
pub const MAX_FILEPATH_LENGTH: usize = 4096;


//= Structures & Enumerations

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyboardKey {
	KeyNull				= 0, 		// Key: NULL, used for no key pressed
	// Alphanumeric keys
	KeyApostrophe		= 39,		// Key: '
	KeyComma			= 44,		// Key: ,
	KeyMinus			= 45,		// Key: -
	KeyPeriod			= 46,		// Key: .
	KeySlash			= 47,		// Key: /
	KeyZero				= 48,		// Key: 0
	KeyOne				= 49,		// Key: 1
	KeyTwo				= 50,		// Key: 2
	KeyThree			= 51,		// Key: 3
	KeyFour				= 52,		// Key: 4
	KeyFive				= 53,		// Key: 5
	KeySix				= 54,		// Key: 6
	KeySeven			= 55,		// Key: 7
	KeyEight			= 56,		// Key: 8
	KeyNine				= 57,		// Key: 9
	KeySemicolon		= 59,		// Key: ;
	KeyEqual			= 61,		// Key: =
	KeyA				= 65,		// Key: A | a
	KeyB				= 66,		// Key: B | b
	KeyC				= 67,		// Key: C | c
	KeyD				= 68,		// Key: D | d
	KeyE				= 69,		// Key: E | e
	KeyF				= 70,		// Key: F | f
	KeyG				= 71,		// Key: G | g
	KeyH				= 72,		// Key: H | h
	KeyI				= 73,		// Key: I | i
	KeyJ				= 74,		// Key: J | j
	KeyK				= 75,		// Key: K | k
	KeyL				= 76,		// Key: L | l
	KeyM				= 77,		// Key: M | m
	KeyN				= 78,		// Key: N | n
	KeyO				= 79,		// Key: O | o
	KeyP				= 80,		// Key: P | p
	KeyQ				= 81,		// Key: Q | q
	KeyR				= 82,		// Key: R | r
	KeyS				= 83,		// Key: S | s
	KeyT				= 84,		// Key: T | t
	KeyU				= 85,		// Key: U | u
	KeyV				= 86,		// Key: V | v
	KeyW				= 87,		// Key: W | w
	KeyX				= 88,		// Key: X | x
	KeyY				= 89,		// Key: Y | y
	KeyZ				= 90,		// Key: Z | z
	KeyLeftBracket		= 91,		// Key: [
	KeyBackslash		= 92,		// Key: '\'
	KeyRightBracket		= 93,		// Key: ]
	KeyGrave			= 96,		// Key: `
	// Function keys
	KeySpace			= 32, 		// Key: Space
	KeyEscape			= 256,		// Key: Esc
	KeyEnter			= 257,		// Key: Enter
	KeyTab				= 258,		// Key: Tab
	KeyBackspace		= 259,		// Key: Backspace
	KeyInsert			= 260,		// Key: Ins
	KeyDelete			= 261,		// Key: Del
	KeyRight			= 262,		// Key: Cursor right
	KeyLeft				= 263,		// Key: Cursor left
	KeyDown				= 264,		// Key: Cursor down
	KeyUp				= 265,		// Key: Cursor up
	KeyPageUp			= 266,		// Key: Page up
	KeyPageDown			= 267,		// Key: Page down
	KeyHome				= 268,		// Key: Home
	KeyEnd				= 269,		// Key: End
	KeyCapsLock			= 280,		// Key: Caps lock
	KeyScrollLock		= 281,		// Key: Scroll down
	KeyNumLock			= 282,		// Key: Num lock
	KeyPrintScreen		= 283,		// Key: Print screen
	KeyPause			= 284,		// Key: Pause
	KeyF1				= 290,		// Key: F1
	KeyF2				= 291,		// Key: F2
	KeyF3				= 292,		// Key: F3
	KeyF4				= 293,		// Key: F4
	KeyF5				= 294,		// Key: F5
	KeyF6				= 295,		// Key: F6
	KeyF7				= 296,		// Key: F7
	KeyF8				= 297,		// Key: F8
	KeyF9				= 298,		// Key: F9
	KeyF10				= 299,		// Key: F10
	KeyF11				= 300,		// Key: F11
	KeyF12				= 301,		// Key: F12
	KeyLeftShift		= 340,		// Key: Shift left
	KeyLeftControl		= 341,		// Key: Control left
	KeyLeftAlt			= 342,		// Key: Alt left
	KeyLeftSuper		= 343,		// Key: Super left
	KeyRightShift		= 344,		// Key: Shift right
	KeyRightControl		= 345,		// Key: Control right
	KeyRightAlt			= 346,		// Key: Alt right
	KeyRightSuper		= 347,		// Key: Super right
	KeyKbMenu			= 348,		// Key: KB menu
	// Keypad keys
	KeyKp0				= 320,		// Key: Keypad 0
	KeyKp1				= 321,		// Key: Keypad 1
	KeyKp2				= 322,		// Key: Keypad 2
	KeyKp3				= 323,		// Key: Keypad 3
	KeyKp4				= 324,		// Key: Keypad 4
	KeyKp5				= 325,		// Key: Keypad 5
	KeyKp6				= 326,		// Key: Keypad 6
	KeyKp7				= 327,		// Key: Keypad 7
	KeyKp8				= 328,		// Key: Keypad 8
	KeyKp9				= 329,		// Key: Keypad 9
	KeyKpDecimal		= 330,		// Key: Keypad .
	KeyKpDivide			= 331,		// Key: Keypad /
	KeyKpMultiply		= 332,		// Key: Keypad *
	KeyKpSubtract		= 333,		// Key: Keypad -
	KeyKpAdd			= 334,		// Key: Keypad +
	KeyKpEnter			= 335,		// Key: Keypad Enter
	KeyKpEqual			= 336,		// Key: Keypad =
	// Android key buttons
	KeyBack				= 4,		// Key: Android back button
	KeyMenu				= 5,		// Key: Android menu button
	KeyVolumeUp			= 24,		// Key: Android volume up button
	KeyVolumeDown		= 25		// Key: Android volume down button
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseButton {
	MouseButtonLeft			= 0,	// Mouse button left
	MouseButtonRight		= 1,	// Mouse button right
	MouseButtonMiddle		= 2,	// Mouse button middle (pressed wheel)
	MouseButtonSide			= 3,	// Mouse button side (advanced mouse device)
	MouseButtonExtra		= 4,	// Mouse button extra (advanced mouse device)
	MouseButtonForward		= 5,	// Mouse button forward (advanced mouse device)
	MouseButtonBack			= 6,	// Mouse button back (advanced mouse device)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseCursor {
	MouseCursorDefault		= 0,	// Default pointer shape
	MouseCursorArrow		= 1,	// Arrow shape
	MouseCursorIbeam		= 2,	// Text writing cursor shape
	MouseCursorCrosshair	= 3,	// Cross shape
	MouseCursorPointingHand	= 4,	// Pointing hand cursor
	MouseCursorResizeEw		= 5,	// Horizontal resize/move arrow shape
	MouseCursorResizeNs		= 6,	// Vertical resize/move arrow shape
	MouseCursorResizeNwse	= 7,	// Top-left to bottom-right diagonal resize/move arrow shape
	MouseCursorResizeNesw	= 8,	// The top-right to bottom-left diagonal resize/move arrow shape
	MouseCursorResizeAll	= 9,	// The omnidirectional resize/move cursor shape
	MouseCursorNotAllowed	= 10	// The operation-not-allowed shape
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Platform {
	Desktop,
	DesktopSdl,
	Web,
	DRM,
	Android,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ConfigFlags {
	FlagVsyncHint,
	FlagFullscreenMode,
	FlagWindowResizable,
	FlagWindowUndecorated,
	FlagWindowHidden,
	FlagWindowMinimized,
	FlagWindowMaximized,
	FlagWindowUnfocused,
	FlagWindowTopmost,
	FlagWindowAlwaysRun,
	FlagWindowTransparent,
	FlagWindowHighdpi,
	FlagMsaa4xHint,
	FlagInterlacedHint,
}
impl BitAnd<u32> for ConfigFlags {
	type Output = bool;

	fn bitand(self, rhs: u32) -> Self::Output {
	    return (self as u32 & rhs) != 0;
	}
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
	pub x: i32,
	pub y: i32,
}
impl Display for Point {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}x{}", self.x, self.y)
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size {
	pub width: u32,
	pub height: u32,
}
impl Display for Size {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}x{}", self.width, self.height)
	}
}


//= Procedures

/// Sets the target platform to initialize
pub fn set_platform(platform: Platform) {
	unsafe { CORE.platform = platform; }
}

/// Initialize window and OpenGL context
/// 
/// NOTE: data parameter could be used to pass any kind of required data to the initialization
pub fn init_window(width: u32, height: u32, title: &'static str) {
	unsafe {
		tracelog!(TraceLogLevel::LogInfo, "Initializing \x1b[95mNavia\x1b[0m using \x1b[94mRaylib {}\x1b[0m", RAYLIB_VERSION);
		match CORE.platform {
			Platform::Desktop =>	tracelog!(TraceLogLevel::LogInfo, "Platform backend: DESKTOP (GLFW)"),
			Platform::DesktopSdl =>	tracelog!(TraceLogLevel::LogInfo, "Platform backend: DESKTOP (SDL)"),
			Platform::Web =>		tracelog!(TraceLogLevel::LogInfo, "Platform backend: WEB (HTML5)"),
			Platform::DRM =>		tracelog!(TraceLogLevel::LogInfo, "Platform backend: Native DRM"),
			Platform::Android =>	tracelog!(TraceLogLevel::LogInfo, "Platform backend: Android"),
		}
		

		tracelog!(TraceLogLevel::LogInfo, "Supported Navia modules:");
		tracelog!(TraceLogLevel::LogInfo, "    > rcore:..... \x1b[92mLoaded\x1b[0m (mandatory)");
		tracelog!(TraceLogLevel::LogInfo, "    > rlgl:...... \x1b[92mLoaded\x1b[0m (mandatory)");
		if CORE.rshapes   { tracelog!(TraceLogLevel::LogInfo, "    > rshapes:... \x1b[92mLoaded\x1b[0m (optional)"); }
		else              { tracelog!(TraceLogLevel::LogInfo, "    > rshapes:... \x1b[91mNot loaded\x1b[0m (optional)"); }
		if CORE.rtextures { tracelog!(TraceLogLevel::LogInfo, "    > rtextures:. \x1b[92mLoaded\x1b[0m (optional)"); }
		else              { tracelog!(TraceLogLevel::LogInfo, "    > rtextures:. \x1b[91mNot loaded\x1b[0m (optional)"); }
		if CORE.rtext     { tracelog!(TraceLogLevel::LogInfo, "    > rtext:..... \x1b[92mLoaded\x1b[0m (optional)"); }
		else              { tracelog!(TraceLogLevel::LogInfo, "    > rtext:..... \x1b[91mNot loaded\x1b[0m (optional)"); }
		if CORE.rmodels   { tracelog!(TraceLogLevel::LogInfo, "    > rmodels:... \x1b[92mLoaded\x1b[0m (optional)"); }
		else              { tracelog!(TraceLogLevel::LogInfo, "    > rmodels:... \x1b[91mNot loaded\x1b[0m (optional)"); }
		if CORE.raudio    { tracelog!(TraceLogLevel::LogInfo, "    > raudio:.... \x1b[92mLoaded\x1b[0m (optional)"); }
		else              { tracelog!(TraceLogLevel::LogInfo, "    > raudio:.... \x1b[91mNot loaded\x1b[0m (optional)"); }

		//* Initialize Window data */
		CORE.window.screen.width = width;
		CORE.window.screen.height = height;
		CORE.window.event_waiting = false;
		CORE.window.screen_scale = Matrix::identity();
		CORE.window.title = title;

		//* Initialize global input state */
		CORE.input.keyboard.exit_key = KeyboardKey::KeyEscape;
		CORE.input.mouse.scale = Vector2{x: 1.0, y: 1.0};
		CORE.input.mouse.cursor = MouseCursor::MouseCursorArrow;
		// TODO: CORE.input.gamepad.lastButtonPressed = GAMEPAD_BUTTON_UNKNOWN;

		//* Initialize platform */
		let _ = init_platform();

		//* Initialize rlgl default data (buffers and shaders) */
		// NOTE: CORE.window.current_fbo.width and CORE.window.current_fbo.height not used, just stored as globals in rlgl
		// TODO: rlglInit(CORE.window.current_fbo.width, CORE.window.current_fbo.height);

		//* Setup default viewport */
		// TODO: SetupViewport(CORE.Window.currentFbo.width, CORE.Window.currentFbo.height);

		if CORE.rtext {
			//* Load default font */
			// WARNING: External function: Module required: rtext
			// TODO: LoadFontDefault();

			//* Set font white rectangle for shapes drawing, so shapes and text can be batched together */
			// WARNING: rshapes module is required, if not available, default internal white rectangle is used
			if CORE.rshapes {
				// TODO: Rectangle rec = GetFontDefault().recs[95];
				// TODO: if ConfigFlags::FlagMsaa4xHint & CORE.window.flags {
				if CORE.window.flags.contains(MSAA4xHint.into()) {
					// NOTE: We try to maxime rec padding to avoid pixel bleeding on MSAA filtering
					// TODO: SetShapesTexture(GetFontDefault().texture, (Rectangle){ rec.x + 2, rec.y + 2, 1, 1 });
				} else {
					// NOTE: We set up a 1px padding on char rectangle to avoid pixel bleeding
					// TODO: SetShapesTexture(GetFontDefault().texture, (Rectangle){ rec.x + 1, rec.y + 1, rec.width - 2, rec.height - 2 });
				}
			}
		} else {
			if CORE.rshapes {
				//* Set default texture and rectangle to be used for shapes drawing */
				// NOTE: rlgl default texture is a 1x1 pixel UNCOMPRESSED_R8G8B8A8
				// TODO: Texture2D texture = { rlGetTextureIdDefault(), 1, 1, 1, PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 };
				// TODO: SetShapesTexture(texture, (Rectangle){ 0.0f, 0.0f, 1.0f, 1.0f });    // WARNING: Module required: rshapes
			}
		}

		if CORE.rtext {
			// TODO: if ConfigFlags::FlagWindowHighdpi & CORE.window.flags {
			if CORE.window.flags.contains(HighDPI.into()) {
				//* Set default font texture filter for HighDPI (blurry) */
				//* RL_TEXTURE_FILTER_LINEAR - tex filter: BILINEAR, no mipmaps */
				// TODO: SrlTextureParameters(GetFontDefault().texture.id, RL_TEXTURE_MIN_FILTER, RL_TEXTURE_FILTER_LINEAR);
				// TODO: SrlTextureParameters(GetFontDefault().texture.id, RL_TEXTURE_MAG_FILTER, RL_TEXTURE_FILTER_LINEAR);
			}
		}

		CORE.time.frame_counter = 0;
		CORE.window.should_close = false;

		//* Initialize random seed */
		// TODO: SetRandomSeed((unsigned int)time(NULL));
	}
}

/// Compute framebuffer size relative to screen size and display size
// NOTE: Global variables CORE.Window.render.width/CORE.Window.render.height and CORE.Window.renderOffset.x/CORE.Window.renderOffset.y can be modified
pub fn setup_framebuffer() {
	unsafe {
		//* Calculate CORE.Window.render.width and CORE.Window.render.height, we have the display size (input params) and the desired screen size (global var) */
		if (CORE.window.screen.width > CORE.window.display.width) || (CORE.window.screen.height > CORE.window.display.height) {
			tracelog!(TraceLogLevel::LogWarning, "DISPLAY: Downscaling required: Screen size ({}) is bigger than the display size ({})", CORE.window.screen, CORE.window.display);

			//* Downscaling to fit display with border-bars */
			let width_ratio = CORE.window.display.width as f32 / CORE.window.screen.width as f32;
			let height_ratio = CORE.window.display.height as f32 / CORE.window.screen.height as f32;

			if width_ratio <= height_ratio {
				CORE.window.render.width = CORE.window.display.width;
				CORE.window.render.height = (CORE.window.screen.height as f32 * width_ratio).round() as u32;
				CORE.window.render_offset.x = 0;
				CORE.window.render_offset.y = (CORE.window.display.height - CORE.window.render.height) as i32;
			} else {
				CORE.window.render.width = (CORE.window.screen.width as f32 * height_ratio).round() as u32;
				CORE.window.render.height = CORE.window.display.height;
				CORE.window.render_offset.x = (CORE.window.display.width - CORE.window.render.width) as i32;
				CORE.window.render_offset.y = 0;
			}

			//* Screen scaling required */
			let scale_ratio = CORE.window.render.width as f32 / CORE.window.screen.width as f32;
			CORE.window.screen_scale = Matrix::scale(scale_ratio, scale_ratio, 1.0);

			// NOTE: We render to full display resolution!
			//* We just need to calculate above parameters for downscale matrix and offsets */
    	    CORE.window.render.width = CORE.window.display.width;
    	    CORE.window.render.height = CORE.window.display.height;

			tracelog!(TraceLogLevel::LogWarning, "DISPLAY: Downscale matrix generated, content will be rendered at ({})", CORE.window.render);
		} else if (CORE.window.screen.width < CORE.window.display.width) || (CORE.window.screen.height < CORE.window.display.height) {
			//* Required screen size is smaller than display size */
			tracelog!(TraceLogLevel::LogWarning, "DISPLAY: Upscaling required: Screen size ({}) is smaller than the display size ({})", CORE.window.screen, CORE.window.display);

			if (CORE.window.screen.width == 0) || (CORE.window.screen.height == 0) {
				CORE.window.screen.width = CORE.window.display.width;
				CORE.window.screen.height = CORE.window.display.height;
			}

			//* Upscaling to fit display with border-bars */
			let display_ratio = CORE.window.display.width as f32 / CORE.window.display.height as f32;
			let screen_ratio = CORE.window.screen.width as f32 / CORE.window.screen.height as f32;

			if display_ratio <= screen_ratio {
				CORE.window.render.width = CORE.window.screen.width;
				CORE.window.render.height = (CORE.window.screen.width as f32 / display_ratio).round() as u32;
				CORE.window.render_offset.x = 0;
				CORE.window.render_offset.y = (CORE.window.render.height - CORE.window.screen.height) as i32;
			}
			else
			{
				CORE.window.render.width = (CORE.window.screen.height as f32 * display_ratio).round() as u32;
				CORE.window.render.height = CORE.window.screen.height;
				CORE.window.render_offset.x = (CORE.window.render.width - CORE.window.screen.width) as i32;
				CORE.window.render_offset.y = 0;
			}
		} else {
			CORE.window.render.width = CORE.window.screen.width;
			CORE.window.render.height = CORE.window.screen.height;
			CORE.window.render_offset.x = 0;
			CORE.window.render_offset.y = 0;
		}
	}
}

/// Set mouse scaling
pub fn set_mouse_scale(scale_x: f32, scale_y: f32) {
	unsafe { CORE.input.mouse.offset = Vector2{x: scale_x, y: scale_y} }
}

/// Check if window is currently fullscreen
pub fn is_window_fullscreen() -> bool {
	unsafe { return CORE.window.fullscreen; }
}

/// Initialize hi-resolution timer
pub fn init_timer() {
	//* Setting a higher resolution can improve the accuracy of time-out intervals in wait functions. */
	//* However, it can also reduce overall system performance, because the thread scheduler switches tasks more often. */
	//* High resolutions can also prevent the CPU power management system from entering power-saving modes. */
	//* Setting a higher resolution does not improve the accuracy of the high-resolution performance counter. */
	let is_linux = if CURRENT_PLATFORM.contains(&platforms::OS::Linux.to_string()) { true } else { false };
	let is_freebsd = if CURRENT_PLATFORM.contains(&platforms::OS::FreeBSD.to_string()) { true } else { false };
	let is_openbsd = if CURRENT_PLATFORM.contains(&platforms::OS::OpenBSD.to_string()) { true } else { false };
	let is_emscripten = if CURRENT_PLATFORM.contains(&platforms::OS::Emscripten.to_string()) { true } else { false };

	if is_linux || is_freebsd || is_openbsd || is_emscripten {
		//let now = 0;
	}
}