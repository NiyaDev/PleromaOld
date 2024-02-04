

//= Allows


//= Imports
use glfw::{self, ffi::GLFWmonitor, Context};
use platforms;
use current_platform::CURRENT_PLATFORM;

use crate::{*, logging::TraceLogLevel};



//= Constants
pub static mut CONTEXT: Option<glfw::Glfw> = None;
pub static mut WINDOW:  Option<glfw::PWindow> = None;


//= Structures & Enumerations


//= Procedures

pub fn init_platform() -> i32 {
	unsafe {
		//* Check whether this is on MacOS */
		let is_mac = if CURRENT_PLATFORM.contains(&platforms::OS::MacOS.to_string()) { true } else { false };
		if is_mac { glfw::init_hint(glfw::InitHint::CocoaChdirResources(false)) }

		//* Init glfw */
		let result = glfw::init_no_callbacks();
		if result.is_err() { tracelog!(TraceLogLevel::LogWarning, "GLFW: Failed to initialize GLFW"); return -1; }

		let mut context = result.unwrap();
		context.set_error_callback(error_callback);

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

			if is_mac { context.window_hint(glfw::WindowHint::CocoaRetinaFramebuffer(true)) }
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

			if is_mac { context.window_hint(glfw::WindowHint::OpenGlForwardCompat(true)) }
			else { context.window_hint(glfw::WindowHint::OpenGlForwardCompat(false)) }

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
		context.unset_joystick_callback();

		//* Find monitor resolution */
		let vid_mode = context.with_primary_monitor(|_, m| {
			m.unwrap().get_video_mode()
		});
		if vid_mode.is_none() { tracelog!(TraceLogLevel::LogWarning, "GLFW: Failed to get primary monitor"); return -1; }

		CORE.window.display.width = vid_mode.unwrap().width;
		CORE.window.display.height = vid_mode.unwrap().height;

		//* Set screen width/height to the display width/height if they are 0 */
		if CORE.window.screen.width == 0 { CORE.window.screen.width = CORE.window.display.width }
		if CORE.window.screen.height == 0 { CORE.window.screen.height = CORE.window.display.height }

		let mut window: glfw::PWindow;

		if CORE.window.fullscreen {
			//* remember center for switchinging from fullscreen to window */
			if (CORE.window.screen.height == CORE.window.display.height) && (CORE.window.screen.width == CORE.window.display.width) {
				//* If screen width/height equal to the display, we can't calculate the window pos for toggling full-screened/windowed. */
				//* Toggling full-screened/windowed with pos(0, 0) can cause problems in some platforms, such as X11. */
				CORE.window.position.x = (CORE.window.display.width/4) as i32;
				CORE.window.position.y = (CORE.window.display.height/4) as i32;
			} else {
				CORE.window.position.x = (CORE.window.display.width/2 - CORE.window.screen.width/2) as i32;
				CORE.window.position.y = (CORE.window.display.height/2 - CORE.window.screen.height/2) as i32;
			}

			if CORE.window.position.x < 0 { CORE.window.position.x = 0 }
			if CORE.window.position.y < 0 { CORE.window.position.y = 0 }

			//* Obtain recommended CORE.Window.display.width/CORE.Window.display.height from a valid videomode for the monitor */
			let video_modes = context.with_primary_monitor(|_, m| {
				m.unwrap().get_video_modes()
			});

			//* Get closest video mode to desired CORE.Window.screen.width/CORE.Window.screen.height */
			for mode in video_modes {
				if mode.width >= CORE.window.screen.width {
					if mode.height >= CORE.window.screen.height {
						CORE.window.display.width = mode.width;
						CORE.window.display.height = mode.height;
						break;
					}
				}
			}

			tracelog!(TraceLogLevel::LogWarning, "SYSTEM: Closest fullscreen videomode: {}", CORE.window.display);

			// NOTE: ISSUE: Closest videomode could not match monitor aspect-ratio, for example,
			//* for a desired screen size of 800x450 (16:9), closest supported videomode is 800x600 (4:3), */
			//* framebuffer is rendered correctly but once displayed on a 16:9 monitor, it gets stretched */
			//* by the sides to fit all monitor space... */

			//* Try to setup the most appropriate fullscreen framebuffer for the requested screenWidth/screenHeight */
			//* It considers device display resolution mode and setups a framebuffer with black bars if required (render size/offset) */
			//* Modified global variables: CORE.Window.screen.width/CORE.Window.screen.height - CORE.Window.render.width/CORE.Window.render.height - CORE.Window.renderOffset.x/CORE.Window.renderOffset.y - CORE.Window.screenScale */
			// TODO: It is a quite cumbersome solution to display size vs requested size, it should be reviewed or removed...
			//* HighDPI monitors are properly considered in a following similar function: SetupViewport() */
			setup_framebuffer();

			let result = context.create_window(CORE.window.display.width, CORE.window.display.height, CORE.window.title, glfw::WindowMode::FullScreen(&glfw::Monitor::from_primary()));
			if result.is_none() {
				tracelog!(TraceLogLevel::LogWarning, "GLFW: Failed to initialize Window");
				return -1;
			}
			window = result.unwrap().0;
		} else {
			//* If we are windowed fullscreen, ensures that window does not minimize when focus is lost */
			if (CORE.window.screen.width == CORE.window.display.width) && (CORE.window.screen.height == CORE.window.display.height) {
				context.window_hint(glfw::WindowHint::AutoIconify(false))
			}

			//* No-fullscreen window creation */
			let result = context.create_window(CORE.window.screen.width, CORE.window.screen.height, CORE.window.title, glfw::WindowMode::Windowed);
			if result.is_none() {
				tracelog!(TraceLogLevel::LogWarning, "GLFW: Failed to initialize Window");
				return -1;
			}
			window = result.unwrap().0;

			CORE.window.render.width = CORE.window.screen.width;
			CORE.window.render.height = CORE.window.screen.height;
		}

		window.make_current();

		let error = glfw::get_error();

		//* Check context activation */
		if (error != glfw::Error::NoWindowContext) && (error != glfw::Error::PlatformError) {
			CORE.window.ready = true;

			context.set_swap_interval(glfw::SwapInterval::None);

			//* Try to enable GPU V-Sync, so frames are limited to screen refresh rate (60Hz -> 60 FPS) */
			// NOTE: V-Sync can be enabled by graphic driver configuration, it doesn't need
			//* to be activated on web platforms since VSync is enforced there. */
			if (CORE.window.flags & FLAG_VSYNC_HINT) != 0 {
				// WARNING: It seems to hit a critical render path in Intel HD Graphics
				context.set_swap_interval(glfw::SwapInterval::Sync(1));
				tracelog!(TraceLogLevel::LogInfo, "DISPLAY: Trying to enable VSYNC");
			}

			let mut fb_width = CORE.window.screen.width;
			let mut fb_height = CORE.window.screen.height;

			if (CORE.window.flags & FLAG_WINDOW_HIGHDPI) > 0 {
				// NOTE: On APPLE platforms system should manage window/input scaling and also framebuffer scaling.
				//* Framebuffer scaling should be activated with: glfwWindowHint(GLFW_COCOA_RETINA_FRAMEBUFFER, GLFW_TRUE); */
				if !is_mac {
					let (inter_width, inter_height) = window.get_framebuffer_size();
					fb_width = inter_width as u32;
					fb_height = inter_height as u32;

					//* Screen scaling matrix is required in case desired screen area is different from display area */
					CORE.window.screen_scale = Matrix::scale(
						(fb_width  / CORE.window.screen.width) as f32,
						(fb_height / CORE.window.screen.height) as f32,
						1.0,
					);

					//* Mouse input scaling for the new screen size */
					set_mouse_scale((CORE.window.screen.width / fb_width) as f32, (CORE.window.screen.height / fb_height) as f32);
				}
			}

			CORE.window.render.width = fb_width;
			CORE.window.render.height = fb_height;
			CORE.window.current_fbo.width = fb_width;
			CORE.window.current_fbo.height = fb_height;

			tracelog!(TraceLogLevel::LogInfo, "DISPLAY: Device initialized successfully");
			tracelog!(TraceLogLevel::LogInfo, "    > Display size: {}", CORE.window.display);
			tracelog!(TraceLogLevel::LogInfo, "    > Screen size:  {}", CORE.window.screen);
			tracelog!(TraceLogLevel::LogInfo, "    > Render size:  {}", CORE.window.render);
			tracelog!(TraceLogLevel::LogInfo, "    > Viewport offsets: {}", CORE.window.render_offset);
		} else {
			tracelog!(TraceLogLevel::LogFatal, "PLATFORM: Failed to initialize graphics device");
			return -1;
		}

		if (CORE.window.flags & FLAG_WINDOW_MINIMIZED) > 0 { minimize_window() }

		//* If graphic device is not properly initialized, we end program */
		if !CORE.window.ready { tracelog!(TraceLogLevel::LogFatal, "PLATFORM: Failed to initialize graphics device") }
		else { set_window_position(get_monitor_width(get_current_monitor()) / 2 - CORE.window.screen.width as i32 / 2, get_monitor_height(get_current_monitor()) / 2 - CORE.window.screen.height as i32 / 2) }
		
		//* Load OpenGL extensions */
		// NOTE: GL procedures address loader is required to load extensions
		// TODO: rl_load_extensions(glfwGetProcAddress);
		
		//=----------------------------------------------------------------------------
		//= Initialize input events callbacks
		//=----------------------------------------------------------------------------
		//* Set window callback events */
		// TODO: window.set_size_callback(window_size_callback);			// NOTE: Resizing not allowed by default!
		// TODO: window.set_maximize_callback(window_maximize_callback);
		// TODO: window.set_iconify_callback(window_iconify_callback);
		// TODO: window.set_focus_callback(window_focus_callback);
		// TODO: window.set_drag_and_drop_callback(window_drop_callback);

		// TODO: if (CORE.window.flags & FLAG_WINDOW_HIGHDPI) > 0 { window.set_content_scale_callback(window_content_scale_callback) }

		//* Set input callback events */
		// TODO: window.set_key_callback(key_callback);
		// TODO: window.set_char_callback(char_callback);
		// TODO: window.set_mouse_button_callback(mouse_button_callback);
		// TODO: window.set_cursor_pos_callback(mouse_cursor_position_callback);
		// TODO: window.set_scroll_callback(mouse_scroll_callback);
		// TODO: window.set_cursor_enter_callback(mouse_cursor_enter_callback);
		// TODO: context.set_joystick_callback(joystick_callback);

		// TODO: glfwSetInputMode(platform.handle, GLFW_LOCK_KEY_MODS, GLFW_TRUE);    // Enable lock keys modifiers (CAPS, NUM)

		//* Retrieve gamepad names */
		// TODO: 
		for i in 0..MAX_GAMEPADS {
			let joystick = context.get_joystick(glfw::JoystickId::from_i32(i as i32).unwrap());
			if joystick.is_present() {
				let str = joystick.get_gamepad_name().unwrap();
				let bytes = str.as_bytes();
				for l in 0..str.len() {
					if l >= 64 {break}
					CORE.input.gamepad.name[i][l] = bytes[l];
				}
			}
		}

		//=----------------------------------------------------------------------------
		//= Initialize timing system
		//=----------------------------------------------------------------------------
		// TODO: init_timer();


		CONTEXT = Some(context);
		WINDOW = Some(window);

		tracelog!(TraceLogLevel::LogInfo, "PLATFORM: DESKTOP (GLFW): Initialized successfully");

		return 0;
	}
}

/// Set window state: minimized
pub fn minimize_window() {
	// NOTE: Following function launches callback that sets appropriate flag!
	unsafe { WINDOW.as_mut().unwrap().iconify() }
}

/// Set window position on screen (windowed mode)
pub fn set_window_position(x: i32, y: i32) {
	unsafe { WINDOW.as_mut().unwrap().set_pos(x, y) }
}

/// Get number of monitors
pub fn get_monitor_count() -> i32 {
	let mut count = [0;1];
	unsafe { let _ = glfw::ffi::glfwGetMonitors(count.as_mut_ptr()); }

	return count[0];
}

/// Get number of monitors
pub fn get_current_monitor() -> i32 {
	unsafe {
		let mut index = 0;
		let mut monitor_count = [0;1];
		let monitors = glfw::ffi::glfwGetMonitors(monitor_count.as_mut_ptr());
		let mut monitor: *mut GLFWmonitor;

		if monitor_count[0] >= 1 {
			if is_window_fullscreen() {
				//* Get the handle of the monitor that the specified window is in full screen on */
				monitor = glfw::ffi::glfwGetWindowMonitor(WINDOW.as_mut().unwrap().window_ptr());

				for i in 0..monitor_count[0] {
					if *monitors.wrapping_add(i as usize).as_ref().unwrap() == monitor {
						index = i;
						break;
					}
				}
			} else {
				//* In case the window is between two monitors, we use below logic */
				//* to try to detect the "current monitor" for that window, note that */
				//* this is probably an overengineered solution for a very side case */
				//* trying to match SDL behaviour */

				let mut closest_dist = 0x7FFFFFFF;

				//* Window center position */
				let (wcx, wcy) = WINDOW.as_mut().unwrap().get_pos();

				for i in 0..monitor_count[0] {
					//* Monitor top-left position */
					let (mut mx, mut my) = ([0;1],[0;1]);

					monitor = *monitors.wrapping_add(i as usize).as_ref().unwrap();
					glfw::ffi::glfwGetMonitorPos(monitor, mx.as_mut_ptr(), my.as_mut_ptr());
					let video_mode = glfw::ffi::glfwGetVideoMode(monitor);

					if !video_mode.is_null() {
						let right = mx[0] + video_mode.as_ref().unwrap().width - 1;
						let bottom = my[0] + video_mode.as_ref().unwrap().height - 1;

						if (wcx >= mx[0]) && (wcx <= right) && (wcy >= my[0]) && (wcy >= bottom) {
							index = i;
							break;
						}

						let mut closest_x = wcx;
						if wcx < mx[0] { closest_x = mx[0] }
						else if wcx > right { closest_x = right }

						let mut closest_y = wcy;
						if wcy < my[0] { closest_y = my[0] }
						else if wcy > bottom { closest_y = bottom }

						let dx = wcx - closest_x;
						let dy = wcy - closest_y;
						let dist = (dx*dx) + (dy*dy);
						if dist < closest_dist {
							index = i;
							closest_dist = dist;
						}
					} else {
						tracelog!(TraceLogLevel::LogWarning, "GLFW: Failed to find the video mode for selected monitor");
					}
				}
			}
		}

		return index;
	}
}

/// Get selected monitor position
pub fn get_monitor_position(monitor: i32) -> Vector2 {
	unsafe {
		let mut monitor_count = [0;1];
		let monitors = glfw::ffi::glfwGetMonitors(monitor_count.as_mut_ptr());

		if monitor >= 0 && monitor< monitor_count[0] {
			let (mut x, mut y) = ([0;1], [0;1]);
			glfw::ffi::glfwGetMonitorPos(*monitors.wrapping_add(monitor as usize).as_ref().unwrap(), x.as_mut_ptr(), y.as_mut_ptr());
			
			return Vector2{x: x[0] as f32, y: y[0] as f32};
		}
		tracelog!(TraceLogLevel::LogWarning, "GLFW: Failed to find selected monitor");

		return Vector2{x: 0.0, y: 0.0};
	}
}

/// Get selected monitor width (currently used by monitor)
pub fn get_monitor_width(monitor: i32) -> i32 {
	unsafe {
		let mut width = 0;
		let mut monitor_count = [0;1];
		let monitors = glfw::ffi::glfwGetMonitors(monitor_count.as_mut_ptr());

		if monitor >= 0 && monitor < monitor_count[0] {
			let video_mode = glfw::ffi::glfwGetVideoMode(*monitors.wrapping_add(monitor as usize).as_ref().unwrap());

			if !video_mode.is_null() { width = video_mode.as_ref().unwrap().width }
			else { tracelog!(TraceLogLevel::LogWarning, "GLFW: Failed to find video mode for selected monitor") }
		} else { tracelog!(TraceLogLevel::LogWarning, "GLFW: Failed to find selected monitor") }

		return width;
	}
}

/// Get selected monitor height (currently used by monitor)
pub fn get_monitor_height(monitor: i32) -> i32 {
	unsafe {
		let mut height = 0;
		let mut monitor_count = [0;1];
		let monitors = glfw::ffi::glfwGetMonitors(monitor_count.as_mut_ptr());

		if monitor >= 0 && monitor < monitor_count[0] {
			let video_mode = glfw::ffi::glfwGetVideoMode(*monitors.wrapping_add(monitor as usize).as_ref().unwrap());

			if !video_mode.is_null() { height = video_mode.as_ref().unwrap().height }
			else { tracelog!(TraceLogLevel::LogWarning, "GLFW: Failed to find video mode for selected monitor") }
		} else { tracelog!(TraceLogLevel::LogWarning, "GLFW: Failed to find selected monitor") }

		return height;
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