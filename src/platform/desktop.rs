

//= Allows


//= Imports
use glfw::{self, ffi::{glfwGetPrimaryMonitor, glfwWindowHint}};
//use current_platform::CURRENT_PLATFORM;

use crate::{*, logging::TraceLogLevel};



//= Constants


//= Structures & Enumerations


//= Procedures

pub fn init_platform() -> i32 {
	unsafe {
		let result = glfw::init_no_callbacks();
		if result.is_err() { tracelog!(TraceLogLevel::LogWarning, "GLFW: Failed to initialize GLFW"); return -1; }

		let mut context = result.unwrap();
		context.set_error_callback(error_callback);

		// TODO: if CURRENT_PLATFORM == APPLE { context.window_hint(glfw::WindowHint::CocoaChdirResources(false)) }

		context.default_window_hints();

    	//= Check window creation flags
		//* Fullscreen */
		if (CORE.window.flags & FLAG_FULLSCREEN_MODE) > 0 { CORE.window.fullscreen = true }

		//* Window visible */
		if (CORE.window.flags & FLAG_WINDOW_HIDDEN) > 0 { context.window_hint(glfw::WindowHint::Visible(false)) }
		else { context.window_hint(glfw::WindowHint::Visible(true)) }

		//* Window decoration */
		if (CORE.window.flags & FLAG_WINDOW_UNDECORATED) > 0 { context.window_hint(glfw::WindowHint::Decorated(false)) }
		else { context.window_hint(glfw::WindowHint::Decorated(true)) }

		//* Window resizeable */
		if (CORE.window.flags & FLAG_WINDOW_RESIZABLE) > 0 { context.window_hint(glfw::WindowHint::Resizable(false)) }
		else { context.window_hint(glfw::WindowHint::Resizable(true)) }

		//* Disable FLAG_WINDOW_MINIMIZED, not supported on initialization */
		if (CORE.window.flags & FLAG_WINDOW_MINIMIZED) > 0 { CORE.window.flags &= !FLAG_WINDOW_MINIMIZED }

		//* Disable FLAG_WINDOW_MAXIMIZED, not supported on initialization */
		if (CORE.window.flags & FLAG_WINDOW_MAXIMIZED) > 0 { CORE.window.flags &= !FLAG_WINDOW_MAXIMIZED }

		//* Window unfocused */
		if (CORE.window.flags & FLAG_WINDOW_UNFOCUSED) > 0 { context.window_hint(glfw::WindowHint::Focused(false)) }
		else { context.window_hint(glfw::WindowHint::Focused(true)) }

		//* Window unfocused */
		if (CORE.window.flags & FLAG_WINDOW_TOPMOST) > 0 { context.window_hint(glfw::WindowHint::Floating(true)) }
		else { context.window_hint(glfw::WindowHint::Floating(false)) }

		//* Window unfocused */
		// NOTE: Some GLFW flags are not supported on HTML5
		if (CORE.window.flags & FLAG_WINDOW_TRANSPARENT) > 0 { context.window_hint(glfw::WindowHint::TransparentFramebuffer(true)) }
		else { context.window_hint(glfw::WindowHint::TransparentFramebuffer(false)) }

		if (CORE.window.flags & FLAG_WINDOW_HIGHDPI) > 0 {
			//* Resize window content area based on the monitor content scale. */
			//* Scale content area based on the monitor content scale where window is placed on. */
			//* On platforms like macOS the resolution of the framebuffer is changed independently of the window size. */
			// NOTE: This hint only has an effect on platforms where screen coordinates and pixels always map 1:1 such as Windows and X11.
			context.window_hint(glfw::WindowHint::ScaleToMonitor(true));

			// TODO: if CURRENT_PLATFORM == APPLE { context.window_hint(glfw::WindowHint::CocoaRetinaFramebuffer(true)) }
		} else { context.window_hint(glfw::WindowHint::ScaleToMonitor(false)) }

		//* Mouse passthrough */
		// TODO: Rusts version of glfw doesn't have "GLFW_MOUSE_PASSTHROUGH"
		// TODO: if (CORE.window.flags & FLAG_WINDOW_MOUSE_PASSTHROUGH) > 0 { context.window_hint(glfw::WindowHint::(true)) }
		// TODO: else { context.window_hint(glfw::WindowHint::(false)) }

		if (CORE.window.flags & FLAG_MSAA_4X_HINT) > 0 {
			// NOTE: MSAA is only enabled for main framebuffer, not user-created FBOs
			tracelog!(TraceLogLevel::LogInfo, "DISPLAY: Trying to enable MSAA x4");
			context.window_hint(glfw::WindowHint::Samples(Some(4)));
		}

		// NOTE: When asking for an OpenGL context version, most drivers provide the highest supported version
		//* with backward compatibility to older OpenGL versions. */
		//* For example, if using OpenGL 1.1, driver can provide a 4.3 backwards compatible context. */
		
		//* Check selection OpenGL version */
		if rl_get_version() == GlVersion::RlOpengl21 { context.window_hint(glfw::WindowHint::ContextVersion(2, 1)) }
		else if rl_get_version() == GlVersion::RlOpengl33 {
			context.window_hint(glfw::WindowHint::ContextVersion(3, 3));
			context.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

			// TODO: if CURRENT_PLATFORM == APPLE { context.window_hint(glfw::WindowHint::OpenGlForwardCompat(true)) }
			// TODO: else { context.window_hint(glfw::WindowHint::OpenGlForwardCompat(false)) }
			context.window_hint(glfw::WindowHint::OpenGlForwardCompat(false));
		} else if rl_get_version() == GlVersion::RlOpengl43 {
			context.window_hint(glfw::WindowHint::ContextVersion(4, 3));
			context.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
			context.window_hint(glfw::WindowHint::OpenGlForwardCompat(false));
		} else if rl_get_version() == GlVersion::RlOpenglEs20 {
			context.window_hint(glfw::WindowHint::ContextVersion(2, 0));
			context.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGlEs));
			context.window_hint(glfw::WindowHint::ContextCreationApi(glfw::ContextCreationApi::Egl));
		} else if rl_get_version() == GlVersion::RlOpenglEs30 {
			context.window_hint(glfw::WindowHint::ContextVersion(3, 0));
			context.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGlEs));
			context.window_hint(glfw::WindowHint::ContextCreationApi(glfw::ContextCreationApi::Egl));	
		}

		// NOTE: GLFW 3.4+ defers initialization of the Joystick subsystem on the first call to any Joystick related functions.
		//* Forcing this initialization here avoids doing it on PollInputEvents() called by EndDrawing() after first frame has been just drawn. */
		//* The initialization will still happen and possible delays still occur, but before the window is shown, which is a nicer experience. */
		// TODO: glfwSetJoystickCallback(NULL); ??

		//* Find monitor resolution */
		let vidMode = context.with_primary_monitor(|_, m| {
			m.unwrap().get_video_mode()
		});
		if vidMode.is_none() { tracelog!(TraceLogLevel::LogWarning, "GLFW: Failed to get primary monitor"); return -1; }

		// TODO line 1370
		

		return 0;
	}
}

/// GLFW3 Error Callback, runs on GLFW3 error
pub fn error_callback(error: glfw::Error, description: String) {
	unsafe {tracelog!(TraceLogLevel::LogWarning, "GLFW: Error: {} Description: {}", error, description);}
}

// TODO: Temporary location
// TODO: Go back through this
#[derive(PartialEq)]
pub enum GlVersion {
	RlOpengl11 = 1,	// OpenGL 1.1
	RlOpengl21,		// OpenGL 2.1 (GLSL 120)
	RlOpengl33,		// OpenGL 3.3 (GLSL 330)
	RlOpengl43,		// OpenGL 4.3 (using GLSL 330)
	RlOpenglEs20,	// OpenGL ES 2.0 (GLSL 100)
	RlOpenglEs30,	// OpenGL ES 3.0 (GLSL 300 es)
}
pub fn rl_get_version() -> GlVersion {
	GlVersion::RlOpengl43
}