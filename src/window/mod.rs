

//= Allows


//= Imports
use current_platform::CURRENT_PLATFORM;

pub mod core;
pub mod structures;
use crate::{*, vectors::*, matrix::*, logging::*, platform::desktop::*, tracelog, flags::WindowFlags::*};

use self::structures::*;


//= Constants


//= Structures & Enumerations


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
		CORE.input.keyboard.exit_key = KeyboardKey::Escape;
		CORE.input.mouse.scale = Vector2{x: 1.0, y: 1.0};
		CORE.input.mouse.cursor = MouseCursor::Arrow;
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