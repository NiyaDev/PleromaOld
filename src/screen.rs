

//= Imports
use raylib_ffi::{rl_str, Rectangle, Vector2};

use crate::render_texture::RenderTexture;


//= Structures and Enumerations

/// WindowState
#[derive(Debug, PartialEq)]
pub enum WindowState {
	Windowed,
	Fullscreen,
	Borderless,
}

/// Screen data
pub struct Screen {
	pub screen_width: i32,
	pub screen_height: i32,
	pub window_state: WindowState,

	pub render_width: i32,
	pub render_height: i32,
	pub render_ratio: f32,
	pub render_texture: Option<RenderTexture>,

	pub raylib_init: bool,
}


//= Constants

/// Default: screen_width
pub const DEF_SCREEN_WIDTH: i32 = 1280;
/// Default: screen_height
pub const DEF_SCREEN_HEIGHT: i32 = 720;


//= Implementations

impl Screen {
	
	//= Creation
	/// Creates basic structure for Screen
	pub fn new() -> Self {
		Self {
			screen_width:	DEF_SCREEN_WIDTH,
			screen_height:	DEF_SCREEN_HEIGHT,

			window_state:	WindowState::Windowed,

			render_width:	DEF_SCREEN_WIDTH,
			render_height:	DEF_SCREEN_HEIGHT,
			render_ratio:	1.0,
			render_texture: None,

			raylib_init:	false,
		}
	}

	//= Manipulation
	/// Wrapper for InitWindow telling the screen that raylib is now on and update render.
	pub fn init(&mut self, title: &str) {
		unsafe { raylib_ffi::InitWindow(self.screen_width, self.screen_height, rl_str!(title)) }
		self.raylib_init = true;

		self.update_render();
	}
	/// Wrapper for CloseWindow that tells the screen that raylib is off
	pub fn close(&mut self) {
		unsafe { raylib_ffi::CloseWindow() }

		if self.render_texture.is_some() { self.render_texture.as_mut().unwrap().unload() }
		self.raylib_init = false;
	}
	/// Wrapper for IsWindowReady
	pub fn window_ready(&self) -> bool {
		if self.raylib_init { unsafe { raylib_ffi::IsWindowReady() } }
		else { false }
	}
	/// Wrapper for ToggleFullscreen
	pub fn toggle_fullscreen(&mut self) {
		if self.window_state != WindowState::Fullscreen {
			unsafe {
				raylib_ffi::ToggleFullscreen();
				self.window_state = WindowState::Fullscreen;
				self.screen_width = raylib_ffi::GetScreenWidth();
				self.screen_height = raylib_ffi::GetScreenHeight();
				self.update_render();
			}
		}
	}
	/// Wrapper for ToggleBorderlessWindowed
	pub fn toggle_borderless(&mut self) {
		if self.window_state != WindowState::Borderless {
			unsafe {
				raylib_ffi::ToggleBorderlessWindowed();
				self.window_state = WindowState::Borderless;
				self.screen_width = raylib_ffi::GetScreenWidth();
				self.screen_height = raylib_ffi::GetScreenHeight();
				self.update_render();
			}
		}
	}
	/// Wrapper for SetWindowSize
	pub fn set_resolution(&mut self, width: i32, height: i32) {
		self.screen_width = width;
		self.screen_height = height;

		self.render_width = ((width as f32) * self.render_ratio) as i32;
		self.render_height = ((height as f32) * self.render_ratio) as i32;

		if self.raylib_init { unsafe { raylib_ffi::SetWindowSize(width, height) } }

		self.update_render();
	}
	/// Sets the render scale and creates a new render texture for that resolution.
	pub fn set_render_scale(&mut self, scale: f32) {
		self.render_ratio = scale;
		self.render_width = ((self.screen_width as f32) * self.render_ratio) as i32;
		self.render_height = ((self.screen_height as f32) * self.render_ratio) as i32;

		self.update_render();
	}
	/// Starts rendering to texture if it exists
	pub fn start_draw(&mut self) {
		if self.render_texture.is_none() {
			// TODO: Error reporting
			return;
		}

		self.render_texture.as_mut().unwrap().begin_texture_mode();
	}
	/// End rendering to texture if it exists and draws it to screen
	pub fn end_draw(&mut self) {
		if self.render_texture.is_none() {
			// TODO: Error reporting
			return;
		}

		self.render_texture.as_mut().unwrap().end_texture_mode();

		unsafe {
			raylib_ffi::BeginDrawing();
			raylib_ffi::DrawTexturePro(
				self.render_texture.as_mut().unwrap().0.texture,
				Rectangle{
					x: 0.0,
					y: 0.0,
					width: self.render_width as f32,
					height: -self.render_height as f32,
				},
				Rectangle{
					x: 0.0,
					y: 0.0,
					width: self.screen_width as f32,
					height: self.screen_height as f32,
				},
				Vector2{x: 0.0, y: 0.0},
				0.0,
				raylib_ffi::colors::WHITE,
			);
			raylib_ffi::EndDrawing();
		}
	}
	/// Unloads previous texture if it exists and 
	fn update_render(&mut self) {
		if self.render_texture.is_some() { self.render_texture.as_mut().unwrap().unload() }
		self.render_texture = Some(RenderTexture::load(self.render_width, self.render_height))
	}

}