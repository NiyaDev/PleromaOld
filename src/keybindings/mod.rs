

//= Imports
pub mod keyboard_keys;
pub mod mouse_keys;
pub mod gamepad_keys;

use std::{collections::HashMap, fs};

use self::keyboard_keys::KeyboardKey;


//= Structures and Enumerations

/// Keybind
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Keybind {
	Keyboard{
		key: keyboard_keys::KeyboardKey,
	},
	MouseButton{
		key: mouse_keys::MouseButton,
	},
	GamepadButtons{
		pad_number: i32,
		key: gamepad_keys::GamepadButton,
	},
	GamepadAxis{
		pad_number: i32,
		key: gamepad_keys::GamepadAxis,
	}
}

/// Keybindings errors
pub enum KeybindingErrors {
	AxisWhileTestingButtons,
	ButtonsWhileTestingAxis,
	KeybindingDoesntExist,
}

/// Keybinding System
pub struct Keybindings(HashMap<String, Keybind>);


//= Implementations

impl Keybindings {
	
	/// Create empty system
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	/// Load from file
	pub fn load_from_file(&mut self, filename: &str) {
		let file = fs::read_to_string(filename);
		if file.is_err() {
			// TODO Error
			return;
		}
		let file_string = file.unwrap();

		let json: serde_json::Value = serde_json::from_str(&file_string).unwrap();
		for i in json.as_array().unwrap() {
			let key_type = i[1].as_str().unwrap();
			let mut key = Keybind::Keyboard { key: KeyboardKey::Null };
			match key_type {
				"keyboard" => { key = Keybind::Keyboard { key: KeyboardKey::from(i[2].as_i64().unwrap()) }}
				"mouse" => {}
				"gamepad_button" => {}
				"gamepad_axis" => {}
				_ => {}
			}
			self.insert(i[0].as_str().unwrap(), key);
		}
	}

	/// Inserting a new keybinding
	pub fn insert(&mut self, name: &str, key: Keybind) {
		self.0.insert(name.to_string(), key);
	}

	/// Key pressed
	pub fn key_pressed(&self, key: &str) -> Result<bool, KeybindingErrors> {
		if self.0.contains_key(&key.to_string()) {
			let keybind = self.0[&key.to_string()].clone();
			match keybind {
				Keybind::Keyboard { key } => {
					unsafe { Ok(raylib_ffi::IsKeyPressed(key as i32)) }
				}
				Keybind::MouseButton { key } => {
					unsafe { Ok(raylib_ffi::IsMouseButtonPressed(key as i32)) }
				}
				Keybind::GamepadButtons { pad_number, key } => {
					unsafe { Ok(raylib_ffi::IsGamepadButtonPressed(pad_number, key as i32)) }
				}
				_ => { Err(KeybindingErrors::AxisWhileTestingButtons) }
			}
		} else { Err(KeybindingErrors::KeybindingDoesntExist) }
	}
	/// Key down
	pub fn key_down(&self, key: String) -> Result<bool, KeybindingErrors> {
		if self.0.contains_key(&key.to_string()) {
			let keybind = self.0[&key.to_string()].clone();
			match keybind {
				Keybind::Keyboard { key } => {
					unsafe { Ok(raylib_ffi::IsKeyDown(key as i32)) }
				}
				Keybind::MouseButton { key } => {
					unsafe { Ok(raylib_ffi::IsMouseButtonDown(key as i32)) }
				}
				Keybind::GamepadButtons { pad_number, key } => {
					unsafe { Ok(raylib_ffi::IsGamepadButtonDown(pad_number, key as i32)) }
				}
				_ => { Err(KeybindingErrors::AxisWhileTestingButtons) }
			}
		} else { Err(KeybindingErrors::KeybindingDoesntExist) }
	}
	/// Key released
	pub fn key_released(&self, key: String) -> Result<bool, KeybindingErrors> {
		if self.0.contains_key(&key.to_string()) {
			let keybind = self.0[&key.to_string()].clone();
			match keybind {
				Keybind::Keyboard { key } => {
					unsafe { Ok(raylib_ffi::IsKeyReleased(key as i32)) }
				}
				Keybind::MouseButton { key } => {
					unsafe { Ok(raylib_ffi::IsMouseButtonReleased(key as i32)) }
				}
				Keybind::GamepadButtons { pad_number, key } => {
					unsafe { Ok(raylib_ffi::IsGamepadButtonReleased(pad_number, key as i32)) }
				}
				_ => { Err(KeybindingErrors::AxisWhileTestingButtons) }
			}
		} else { Err(KeybindingErrors::KeybindingDoesntExist) }
	}
	/// Key up
	pub fn key_up(&self, key: String) -> Result<bool, KeybindingErrors> {
		if self.0.contains_key(&key.to_string()) {
			let keybind = self.0[&key.to_string()].clone();
			match keybind {
				Keybind::Keyboard { key } => {
					unsafe { Ok(raylib_ffi::IsKeyUp(key as i32)) }
				}
				Keybind::MouseButton { key } => {
					unsafe { Ok(raylib_ffi::IsMouseButtonUp(key as i32)) }
				}
				Keybind::GamepadButtons { pad_number, key } => {
					unsafe { Ok(raylib_ffi::IsGamepadButtonUp(pad_number, key as i32)) }
				}
				_ => { Err(KeybindingErrors::AxisWhileTestingButtons) }
			}
		} else { Err(KeybindingErrors::KeybindingDoesntExist) }
	}
	/// Get Axis
	pub fn get_axis(&self, key: String) -> Result<f32, KeybindingErrors> {
		if self.0.contains_key(&key.to_string()) {
			let keybind = self.0[&key.to_string()].clone();
			match keybind {
				Keybind::GamepadAxis { pad_number, key } => {
					unsafe { Ok(raylib_ffi::GetGamepadAxisMovement(pad_number, key as i32)) }
				}
				_ => { Err(KeybindingErrors::ButtonsWhileTestingAxis) }
			}
		} else { Err(KeybindingErrors::KeybindingDoesntExist) }
	}

}