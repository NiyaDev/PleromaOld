

//= Imports
pub mod keyboard_keys;
pub mod mouse_keys;
pub mod gamepad_keys;

use std::{collections::HashMap, fs};
use self::keyboard_keys::KeyboardKey;


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
	MouseAxis{
		key: mouse_keys::MouseAxis,
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
					unsafe { Ok(IsKeyPressed(key as i32)) }
				}
				Keybind::MouseButton { key } => {
					unsafe { Ok(IsMouseButtonPressed(key as i32)) }
				}
				Keybind::GamepadButtons { pad_number, key } => {
					unsafe { Ok(IsGamepadButtonPressed(pad_number, key as i32)) }
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
					unsafe { Ok(IsKeyDown(key as i32)) }
				}
				Keybind::MouseButton { key } => {
					unsafe { Ok(IsMouseButtonDown(key as i32)) }
				}
				Keybind::GamepadButtons { pad_number, key } => {
					unsafe { Ok(IsGamepadButtonDown(pad_number, key as i32)) }
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
					unsafe { Ok(IsKeyReleased(key as i32)) }
				}
				Keybind::MouseButton { key } => {
					unsafe { Ok(IsMouseButtonReleased(key as i32)) }
				}
				Keybind::GamepadButtons { pad_number, key } => {
					unsafe { Ok(IsGamepadButtonReleased(pad_number, key as i32)) }
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
					unsafe { Ok(IsKeyUp(key as i32)) }
				}
				Keybind::MouseButton { key } => {
					unsafe { Ok(IsMouseButtonUp(key as i32)) }
				}
				Keybind::GamepadButtons { pad_number, key } => {
					unsafe { Ok(IsGamepadButtonUp(pad_number, key as i32)) }
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
					unsafe { Ok(GetGamepadAxisMovement(pad_number, key as i32)) }
				}
				Keybind::MouseAxis { key } => {
					unsafe {
						match key {
								mouse_keys::MouseAxis::X => Ok(GetMouseX()),
								mouse_keys::MouseAxis::Y => Ok(GetMouseY()),
						}
					}
				}
				_ => { Err(KeybindingErrors::ButtonsWhileTestingAxis) }
			}
		} else { Err(KeybindingErrors::KeybindingDoesntExist) }
	}

}


extern "C" { fn IsKeyPressed(key: i32) -> bool; }
extern "C" { fn IsKeyDown(key: i32) -> bool; }
extern "C" { fn IsKeyReleased(key: i32) -> bool; }
extern "C" { fn IsKeyUp(key: i32) -> bool; }
extern "C" { fn IsMouseButtonPressed(button: i32) -> bool; }
extern "C" { fn IsMouseButtonDown(button: i32) -> bool; }
extern "C" { fn IsMouseButtonReleased(button: i32) -> bool; }
extern "C" { fn IsMouseButtonUp(button: i32) -> bool; }
extern "C" { fn GetMouseX() -> f32; }
extern "C" { fn GetMouseY() -> f32; }
extern "C" { fn IsGamepadButtonPressed(gamepad: i32, button: i32) -> bool; }
extern "C" { fn IsGamepadButtonDown(gamepad: i32, button: i32) -> bool; }
extern "C" { fn IsGamepadButtonReleased(gamepad: i32, button: i32) -> bool; }
extern "C" { fn IsGamepadButtonUp(gamepad: i32, button: i32) -> bool; }
extern "C" { fn GetGamepadAxisMovement(gamepad: i32, axis: i32) -> f32; }