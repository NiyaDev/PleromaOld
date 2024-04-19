

use crate::{
	rl_str,
	structures::{
		color::{self, *},
		misc::clear_background,
		rectangle::*,
		vectors::*,
		render_texture::*,
		texture::*,
	}
};


/// WindowState
#[derive(Debug, PartialEq)]
pub enum WindowState {
	Windowed,
	Fullscreen,
	Borderless,
}

/// Resolution
#[derive(Debug, PartialEq)]
pub struct Resolution {
	pub width: i32,
	pub height: i32,
}

/// Screen data
pub struct Screen {
	pub screen: Resolution,
	pub window_state: WindowState,

	pub render: Resolution,
	pub render_ratio: f32,
	pub render_texture: Option<RenderTexture>,

	pub raylib_init: bool,
	pub background_color: Color,
}


/// Default: screen_width
pub const DEF_SCREEN_WIDTH: i32 = 1280;
/// Default: screen_height
pub const DEF_SCREEN_HEIGHT: i32 = 720;


impl Screen {
	
	//= Creation
	/// Creates basic structure for Screen
	pub fn new() -> Self {
		Self {
			screen: Resolution{
				width:		DEF_SCREEN_WIDTH,
				height:		DEF_SCREEN_HEIGHT,
			},

			window_state:	WindowState::Windowed,

			render: Resolution{
				width:		DEF_SCREEN_WIDTH,
				height:		DEF_SCREEN_HEIGHT,
			},
			render_ratio:	1.0,
			render_texture: None,

			raylib_init:	false,
			background_color: color::DARKGRAY,
		}
	}

	//= Manipulation
	/// Wrapper for InitWindow telling the screen that raylib is now on and update render.
	pub fn init(&mut self, title: &str) -> &mut Self {
		unsafe { InitWindow(self.screen.width, self.screen.height, rl_str!(title)) }
		self.raylib_init = true;

		self.update_render()
	}
	/// Wrapper for CloseWindow that tells the screen that raylib is off
	pub fn close(&mut self) {
		unsafe { CloseWindow() }

		if self.render_texture.is_some() { self.render_texture.as_mut().unwrap().unload() }
		self.raylib_init = false;
	}
	/// Wrapper for IsWindowReady
	pub fn window_ready(&self) -> bool {
		if self.raylib_init { unsafe { IsWindowReady() } }
		else { false }
	}
	/// Wrapper for ToggleFullscreen
	pub fn toggle_fullscreen(&mut self) {
		if self.window_state != WindowState::Fullscreen {
			self.window_state = WindowState::Fullscreen
		} else { self.window_state = WindowState::Windowed }
		unsafe {
			ToggleFullscreen();
			self.window_state = WindowState::Fullscreen;
			self.screen.width = GetScreenWidth();
			self.screen.height = GetScreenHeight();
			self.update_render();
		}
	}
	/// Wrapper for ToggleBorderlessWindowed
	pub fn toggle_borderless(&mut self) {
		if self.window_state != WindowState::Borderless {
			self.window_state = WindowState::Borderless
		} else { self.window_state = WindowState::Windowed }
		unsafe {
			ToggleBorderlessWindowed();
			self.window_state = WindowState::Borderless;
			self.screen.width = GetScreenWidth();
			self.screen.height = GetScreenHeight();
			self.update_render();
		}
	}
	/// Wrapper for SetWindowSize
	pub fn set_resolution(&mut self, width: i32, height: i32) -> &mut Self {
		self.screen.width = width;
		self.screen.height = height;

		self.render.width = ((width as f32) * self.render_ratio) as i32;
		self.render.height = ((height as f32) * self.render_ratio) as i32;

		if self.raylib_init { unsafe { SetWindowSize(width, height) } }

		self.update_render()
	}
	/// Sets the render scale and creates a new render texture for that resolution.
	pub fn set_render_scale(&mut self, scale: f32) -> &mut Self {
		self.render_ratio = scale;
		self.render.width = ((self.screen.width as f32) * self.render_ratio) as i32;
		self.render.height = ((self.screen.height as f32) * self.render_ratio) as i32;

		self.update_render();

		self
	}
	/// Starts rendering to texture if it exists
	pub fn start_draw(&mut self) {
		if self.render_texture.is_none() {
			// TODO: Error reporting
			return;
		}

		self.render_texture.as_mut().unwrap().begin_texture_mode();
		clear_background(self.background_color.into());
	}
	/// End rendering to texture if it exists and draws it to screen
	pub fn end_draw(&mut self) {
		if self.render_texture.is_none() {
			// TODO: Error reporting
			return;
		}

		self.render_texture.as_mut().unwrap().end_texture_mode();

		unsafe {
			BeginDrawing();

			Texture(self.render_texture.as_mut().unwrap().0.texture, WHITE).draw_pro(
				Rectangle{
					x: 0.0,
					y: 0.0,
					width: self.render.width as f32,
					height: -self.render.height as f32,
				},
				Rectangle{
					x: 0.0,
					y: 0.0,
					width: self.screen.width as f32,
					height: self.screen.height as f32,
				},
				Vector2{x: 0.0, y: 0.0},
				0.0,
			);
			
			EndDrawing();
		}
	}
	/// Unloads previous texture if it exists and ends the drawing cycle
	fn update_render(&mut self) -> &mut Self {
		if self.render_texture.is_some() { self.render_texture.as_mut().unwrap().unload() }
		if self.raylib_init { self.render_texture = Some(RenderTexture::load(self.render.width, self.render.height)) }
		
		self
	}

}


extern "C" { fn InitWindow(width: i32, height: i32, title: *const i8); }
extern "C" { fn CloseWindow(); }
extern "C" { fn IsWindowReady() -> bool; }
extern "C" { fn ToggleFullscreen(); }
extern "C" { fn ToggleBorderlessWindowed(); }
extern "C" { fn GetScreenWidth() -> i32; }
extern "C" { fn GetScreenHeight() -> i32; }
extern "C" { fn SetWindowSize(width: i32, height: i32); }
extern "C" { fn BeginDrawing(); }
extern "C" { fn EndDrawing(); }