use std::{collections::HashMap, ops::{BitOrAssign, BitXorAssign}};

use bitflags::{bitflags, Flags};

use crate::{
	audio::AudioHandler, debug::{
		self,
		errors::*,
		DebugFlags,
		LogLevel,
	},
	keybinds::Keybind,
	rl_str,
	structures::{
		color::*,
		font::*,
		image::*,
		rectangle::*,
		render_texture::*,
		resolution::*,
		texture::*,
		vectors::*
	}
};

pub struct Pleroma {
	initialized: bool,

	//* Settings */
	screen_size: Resolution,
	render_size: Resolution,
	framerate: i32,
	windows_flags: WindowFlags,

	//* Screen */
	render_texture: Option<RenderTexture>,
	is_rendering: bool,
	background_color: Color,
	line_spacing: f32,

	//* Input */
	pub keybindings: HashMap<String, Keybind>,
	
	//* Audio */
	pub audio: AudioHandler,

	//* Debug */
	db_level: debug::LogLevel,
	db_settings: debug::DebugFlags,
	db_list: Vec<(LogLevel, String, u32)>,
	db_font: Font,
}
impl Default for Pleroma {
	fn default() -> Self {
		//* Init Raylib */
		unsafe {
			SetTraceLogLevel(7);
			InitWindow(1280, 720, rl_str!("Default"));
			SetTargetFPS(60);
		}

		//* Create structure */
		Self {
			initialized: true,

			screen_size: Resolution {
				width: 1280,
				height: 720,
			},
			render_size: Resolution {
				width: 640,
				height: 360,
			},
			framerate: 60,
			windows_flags: WindowFlags::empty(),

			render_texture: Some(RenderTexture::load(640, 360)),
			is_rendering: false,
			background_color: DARKGRAY,
			line_spacing: 1.0,

			keybindings: HashMap::new(),
			
			audio: AudioHandler::default(),

			db_level: debug::LogLevel::Info,
			db_settings: debug::DebugFlags::all(),
			db_list: Vec::new(),
			db_font: Font::default(),
		}
	}
}

bitflags! {
	/// ### WindowFlags
	/// Implementation of Raylib window flags.
	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
	pub struct WindowFlags: u32 {
		const FULLSCREEN	= 0x00000002;
		const RESIZABLE		= 0x00000004;
		const UNDECORATED	= 0x00000008;
		const TRANSPARENT	= 0x00000010;
		const HINT_4X		= 0x00000020;
		const VSYNC			= 0x00000040;
		const HIDDEN		= 0x00000080;
		const ALWAYS_RUN	= 0x00000100;
		const MINIMIZED		= 0x00000200;
		const MAXIMIZED		= 0x00000400;
		const UNFOCUSED		= 0x00000800;
		const TOPMOST		= 0x00001000;
		const HIGH_DPI		= 0x00002000;
		const MOUSE_PASS	= 0x00004000;
		const BORDERLESS	= 0x00008000;
		const INTERLACED	= 0x00010000;

		const _ = !0;
	}
}

impl Pleroma {

	//= System
	/// ### close
	/// Unloads the RenderTexture and tells Raylib to close the window.
	pub fn close(&mut self) {
		//* Unload RenderTexture */
		if self.render_texture.is_some() {
			self.render_texture.unwrap().unload()
		}

		//* Close Window */
		unsafe { CloseWindow() }
	}
	/// ### should_close
	/// Wrapper for Raylib::WindowShouldClose().
	pub fn should_close(&mut self) -> bool {
		unsafe { WindowShouldClose() }
	}
	/// ### ready
	/// Wrapper for Raylib::IsWindowReady().
	pub fn ready(&mut self) -> bool {
		unsafe{ IsWindowReady() }
	}
	/// ### set_icon
	/// Wrapper for Raylib::SetWindowIcon(image: Image).
	pub fn set_icon(&mut self, image: Image) -> &mut Self {
		unsafe{ SetWindowIcon(image.0) }
		
		self
	}
	
	//= Monitor
	/// ### get_monitor_count
	/// Wrapper for Raylib::SetWindowIcon() -> i32.
	pub fn get_monitor_count(&mut self) -> i32 {
		unsafe{ GetMonitorCount() }
	}
	/// ### set_monitor
	/// Wrapper for Raylib::SetWindowMonitor(monitor: i32).
	pub fn set_monitor(&mut self, monitor: i32) -> &mut Self {
		unsafe{ SetWindowMonitor(monitor) }
		
		self
	}
	/// ### get_current_monitor
	/// Wrapper for Raylib::GetCurrentMonitor() -> i32.
	pub fn get_current_monitor(&mut self) -> i32 {
		unsafe{ GetCurrentMonitor() }
	}
	/// ### get_monitor_size
	/// Wrapper for Raylib::GetMonitorWidth(monitor: i32) -> i32 and Raylib::GetMonitorHeight(monitor: i32) -> i32.
	pub fn get_monitor_size(&mut self, monitor: i32) -> [i32;2] {
		unsafe{
			[
				GetMonitorWidth(monitor),
				GetMonitorHeight(monitor),
			]
		}
	}
	/// ### get_physical_monitor_size
	/// Wrapper for Raylib::GetMonitorPhysicalWidth(monitor: i32) -> i32 and Raylib::GetMonitorPhysicalHeight(monitor: i32) -> i32.
	pub fn get_physical_monitor_size(&mut self, monitor: i32) -> [i32;2] {
		unsafe{
			[
				GetMonitorPhysicalWidth(monitor),
				GetMonitorPhysicalHeight(monitor),
			]
		}
	}
	/// ### get_monitor_refresh_rate
	/// Wrapper for Raylib::GetMonitorRefreshRate(monitor: i32) -> i32.
	pub fn get_monitor_refresh_rate(&mut self, monitor: i32) -> i32 {
		unsafe{ GetMonitorRefreshRate(monitor) }
	}
	/// ### get_monitor_name
	/// Wrapper for Raylib::GetMonitorRefreshRate(monitor: i32) -> i32.
	//pub fn get_monitor_name(&mut self, monitor: i32) -> &str {
	//	unsafe{ GetMonitorName(monitor) }
	//}
	
	//= Screen
	/// ### set_resolution
	/// Sets the resolution of the wondow to the input.
	pub fn set_resolution(&mut self, width: i32, height: i32) -> &mut Self {
		self.screen_size = Resolution { width, height };
		unsafe { SetWindowSize(width, height) }

		self
	}
	/// ### set_render
	/// Sets the resolution the game is to be rendered at.
	pub fn set_render(&mut self, width: i32, height: i32) -> &mut Self {
		if !self.is_rendering {
			self.render_size = Resolution { width, height };

			self.update_render();
		} else {
			self.log(PlError::EditingRenderSize)
		}

		self
	}
	/// ### set_window_flags
	/// Sets the window flags for Raylib.
	fn set_window_flags(&mut self, flags: WindowFlags) -> &mut Self {
		self.windows_flags = flags;
		unsafe { SetWindowState(flags.0 .0 as i32) }

		self
	}
	/// ### get_window_flag
	/// Gets whether the input flag is currently set.
	pub fn get_window_flag(&mut self, flag: WindowFlags) -> bool {
		self.windows_flags.contains(flag)
	}
	/// ### fullscreen
	/// Sets the window to be fullscreen mode.
	pub fn fullscreen(&mut self) -> &mut Self {
		self.windows_flags.bitxor_assign(WindowFlags::FULLSCREEN);

		self.set_window_flags(self.windows_flags)
	}
	/// ### borderless
	/// Sets the window to be borderless window mode.
	pub fn borderless(&mut self) -> &mut Self {
		self.windows_flags.bitxor_assign(WindowFlags::BORDERLESS);

		self.set_window_flags(self.windows_flags)
	}
	/// ### set_framerate
	/// Sets the window to run at the refresh rate of the monitor
	pub fn set_framerate(&mut self, framerate: i32) -> &mut Self {
		if self.windows_flags.contains(WindowFlags::VSYNC) {
			self.framerate = framerate;
			unsafe{ SetTargetFPS(framerate) }
		}
		
		self
	}
	/// ### vsync
	/// Sets the window to run at the refresh rate of the monitor
	pub fn vsync(&mut self) -> &mut Self {
		let cur_mon = self.get_current_monitor();
		let rate = self.get_monitor_refresh_rate(cur_mon);
		self.set_framerate(rate);
		self.windows_flags.bitxor_assign(WindowFlags::VSYNC);
		
		self
	}
	/// ### set_title
	/// Wrapper for Raylib::SetWindowTitle(const char* title).
	pub fn set_title(&mut self, title: &str) -> &mut Self {
		unsafe { SetWindowTitle(rl_str!(title)) }

		self
	}
	/// ### set_font
	/// Sets the font to use in the debug text
	pub fn set_font(&mut self, font: Font) -> &mut Self {
		self.db_font.unload();
		self.db_font = font;

		self
	}
	/// ### set_position
	/// Wrapper for Raylib::SetWindowPosition(x: i32, y: i32).
	pub fn set_position(&mut self, x: i32, y: i32) -> &mut Self {
		unsafe{ SetWindowPosition(x, y) }
		
		self
	}
	/// ### get_position
	/// Wrapper for Raylib::GetWindowPosition() -> Vector2.
	pub fn get_position(&mut self) -> [i32;2] {
		unsafe{
			let vec2 = GetWindowPosition();
			[
				vec2.x as i32,
				vec2.y as i32,
			]
		}
	}
	/// ### set_size_minimum
	/// Wrapper for Raylib::SetWindowMinSize(width: i32, height: i32).
	pub fn set_size_minimum(&mut self, width: i32, height: i32) -> &mut Self {
		unsafe{ SetWindowMinSize(width, height) }
		
		self
	}
	/// ### set_size_maximum
	/// Wrapper for Raylib::SetWindowMaxSize(width: i32, height: i32).
	pub fn set_size_maximum(&mut self, width: i32, height: i32) -> &mut Self {
		unsafe{ SetWindowMaxSize(width, height) }
		
		self
	}
	/// ### set_opacity
	/// Wrapper for Raylib::SetWindowMaxSize(width: i32, height: i32).
	pub fn set_opacity(&mut self, opacity: f32) -> &mut Self {
		unsafe{ SetWindowOpacity(opacity) }
		
		self
	}
	/// ### get_dpi
	/// Wrapper for Raylib::GetWindowScaleDPI() -> Vector2.
	pub fn get_dpi(&mut self) -> [i32;2] {
		unsafe{
			let vec2 = GetWindowScaleDPI();
			[
				vec2.x as i32,
				vec2.y as i32,
			]
		}
	}
	
	//= Fonts
	/// #### set_line_spacing
	/// Wrapper for Raylib::SetTextLineSpacing(spacing: i32).
	pub fn set_line_spacing(&mut self, spacing: i32) -> &mut Self {
		self.line_spacing = spacing as f32;
		unsafe{ SetTextLineSpacing(spacing) }
		
		self
	}
	
	//= Rendering
	/// ### update_render
	/// Recreates the RenderTexture with the current render size.
	pub fn update_render(&mut self) -> &mut Self {
		if self.render_texture.is_some() {
			self.render_texture.as_mut().unwrap().unload()
		}
		if self.initialized {
			self.render_texture = Some(RenderTexture::load(
				self.render_size.width,
				self.render_size.height,
			))
		}

		self
	}
	/// #### draw
	/// Draws to the screen. Calling any code implemented in add_contents.
	pub fn draw(&mut self, add_contents: impl FnOnce(&mut Pleroma)) ->&mut Self {
		//* Check if render texture exists */
		if self.render_texture.is_none() {
			self.log(PlError::RenderTextureDoesntExist);
			return self;
		}
		
		//* Start drawing to render texture */
		self.render_texture.as_mut().unwrap().begin_texture_mode();
		unsafe{ ClearBackground(self.background_color.into()) }
		self.is_rendering = true;
		
		//* Run content */
		let _ = add_contents(self);

		//* Debug info */
		if self.get_debug_setting(DebugFlags::INFO_ENABLE) { self.draw_debug_info(&self.db_font) }

		//* Draw log */
		if self.get_debug_setting(DebugFlags::SCRN_ENABLE) {
			//* Run through each message, draw them, then decrement their timer */
			let mut count = 0;
			let mut list: Vec<i32> = Vec::new();
			for i in self.db_list.as_mut_slice().into_iter() {
				i.2 -= 1;
				if i.2 <= 0 {
					list.push(count)
				} else {
					let col = match i.0 {
						LogLevel::Error => GRAY,
						LogLevel::Critical => RED,
						_ => BLACK,
					};
					let height = self.render_size.height as f32 - 8.0 - (10.0 * count as f32);
					self.db_font
						.draw_force(&i.1, Vector2 { x: 0.0, y: height }, 8.0, 1.0, col);
					count += 1;
				}
			}
			//* Reverse list and delete finished messages from bottom to top */
			list.reverse();
			for i in list { self.db_list.remove(i as usize); }
		}

		//* Draw render texture to screen */
		self.render_texture.as_mut().unwrap().end_texture_mode();
		unsafe {
			BeginDrawing();

			Texture(self.render_texture.as_mut().unwrap().0.texture, WHITE).draw_pro(
				Rectangle {
					x: 0.0,
					y: 0.0,
					width: self.render_size.width as f32,
					height: -self.render_size.height as f32,
				},
				Rectangle {
					x: 0.0,
					y: 0.0,
					width: self.screen_size.width as f32,
					height: self.screen_size.height as f32,
				},
				Vector2 { x: 0.0, y: 0.0 },
				0.0,
			);

			EndDrawing();
		}
		self.is_rendering = false;
		
		self
	}

	//= Debug
	/// ### set_log_level
	/// Sets the level of severity that the messages that the system shows to the developer.
	pub fn set_log_level(&mut self, level: debug::LogLevel) -> &mut Self {
		self.db_level = level;

		self
	}
	/// ### get_log_level
	/// Gets the current level of severity that the system will show the developer.
	pub fn get_log_level(&self) -> debug::LogLevel {
		self.db_level.clone()
	}
	/// ### set_debug_settings
	/// Sets the debug settings to the inputted bitflags.
	pub fn set_debug_settings(&mut self, flags: debug::DebugFlags) -> &mut Self {
		self.db_settings = flags;

		self
	}
	/// ### get_debug_setting
	/// Returns whether the input flag is current set.
	pub fn get_debug_setting(&self, flag: debug::DebugFlags) -> bool {
		self.db_settings.contains(flag)
	}
	/// ### push_message
	/// Appends a message to the end of the debug message list.
	///
	/// The list is displayed on screen for 1 second, as the duration is set as twice the current framerate and each frame it is decremented.
	pub fn push_message(&mut self, level: LogLevel, message: String) {
		self.db_list.push((level, message, (self.framerate * 2) as u32));
	}
	
}

extern "C" { fn InitWindow(width: i32, height: i32, title: *const i8); }
extern "C" { fn CloseWindow(); }
extern "C" { fn WindowShouldClose() -> bool; }
extern "C" { fn IsWindowReady() -> bool; }
extern "C" { fn SetWindowIcon(image: ImageRl); }
extern "C" { fn SetWindowPosition(x: i32, y: i32); }
extern "C" { fn SetWindowMonitor(monitor: i32); }
extern "C" { fn SetWindowMinSize(width: i32, height: i32); }
extern "C" { fn SetWindowMaxSize(width: i32, height: i32); }
extern "C" { fn SetWindowOpacity(opacity: f32); }
extern "C" { fn GetMonitorCount() -> i32; }
extern "C" { fn GetCurrentMonitor() -> i32; }
extern "C" { fn GetMonitorWidth(monitor: i32) -> i32; }
extern "C" { fn GetMonitorHeight(monitor: i32) -> i32; }
extern "C" { fn GetMonitorPhysicalWidth(monitor: i32) -> i32; }
extern "C" { fn GetMonitorPhysicalHeight(monitor: i32) -> i32; }
extern "C" { fn GetMonitorRefreshRate(monitor: i32) -> i32; }
extern "C" { fn GetWindowPosition() -> Vector2; }
extern "C" { fn GetWindowScaleDPI() -> Vector2; }
extern "C" { fn GetMonitorName(monitor: i32) -> *const i8; }

extern "C" { fn SetWindowTitle(title: *const i8); }
extern "C" { fn SetWindowSize(width: i32, height: i32); }
extern "C" { fn SetWindowState(flags: i32); }

extern "C" { fn SetTraceLogLevel(logLevel: i32); }
extern "C" { fn SetTargetFPS(fps: i32); }

extern "C" { fn BeginDrawing(); }
extern "C" { fn EndDrawing(); }
extern "C" { fn ClearBackground(color: Color); }

extern "C" { fn SetTextLineSpacing(spacing: i32); }