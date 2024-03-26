

pub mod keyboard;
pub mod mouse;
pub mod gamepad;
pub mod raylib;

use std::collections::HashMap;

use self::keyboard::KeyboardKey;


/// Keybinding system
pub struct Keybindings(HashMap<String, Vec<Binding>>);

/// Bindings
pub enum Binding {
	KeyboardKey(keyboard::KeyboardKey),
	MouseButton(mouse::MouseButton),
	MouseAxis(mouse::MouseAxis),
	GamepadButton{id: i32, key: gamepad::GamepadButton},
	GamepadAxis{id: i32, key: gamepad::GamepadAxis},
}


impl Keybindings {
	
	//= Creation
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	//= Manipulation
	/// Adding a new binding
	pub fn insert(&mut self, id: &str, keys: Vec<Binding>) -> &mut Self {
		self.0.insert(id.to_string(), keys);

		self
	}
	/// Are keys currently pressed down / Axis at max
	pub fn key_down(&self, id: &str) -> bool {
		if !self.0.contains_key(&id.to_string()) {
			// TODO Debug
		}

		let key = self.0.get(&id.to_string()).unwrap();
		let mut result = true;
		for i in key {
			match i {
				Binding::KeyboardKey(key) => {
					unsafe { if !raylib::IsKeyDown(*key as i32) { result = false } }
				}
				Binding::MouseButton(key) => {
					unsafe { if !raylib::IsMouseButtonDown(*key as i32) { result = false } }
				}
				Binding::MouseAxis(_key) => {
					// TODO: DEBUG doesn't work here
				}
				Binding::GamepadButton{id, key} => {
					unsafe { if !raylib::IsGamepadButtonDown(*id, *key as i32) { result = false } }
				}
				Binding::GamepadAxis{id, key} => {
					unsafe {
						let value = raylib::GetGamepadAxisMovement(*id, *key as i32);
						if value != 1.0 && value != -1.0 { result = false }
					}
				}
			}
		}

		result
	}
	/// Are keys currently pressed down for the first time / Axis at max
	pub fn key_pressed(&self, id: &str) -> bool {
		if !self.0.contains_key(&id.to_string()) {
			// TODO Debug
		}

		let key = self.0.get(&id.to_string()).unwrap();
		let mut result = true;
		for i in key {
			match i {
				Binding::KeyboardKey(key) => {
					unsafe {
						if *key == KeyboardKey::LeftShift || *key == KeyboardKey::RightShift ||
							*key == KeyboardKey::LeftAlt || *key == KeyboardKey::LeftAlt {
								if !raylib::IsKeyDown(*key as i32) { result = false }
							} else {
							if !raylib::IsKeyPressed(*key as i32) { result = false }
						}
					}
				}
				Binding::MouseButton(key) => {
					unsafe { if !raylib::IsMouseButtonPressed(*key as i32) { result = false } }
				}
				Binding::MouseAxis(_key) => {
					// TODO: DEBUG doesn't work here
				}
				Binding::GamepadButton{id, key} => {
					unsafe { if !raylib::IsGamepadButtonPressed(*id, *key as i32) { result = false } }
				}
				Binding::GamepadAxis{id, key} => {
					unsafe {
						let value = raylib::GetGamepadAxisMovement(*id, *key as i32);
						if value != 1.0 && value != -1.0 { result = false }
					}
				}
			}
		}

		result
	}
	/// Are keys currently released for the first time / Axis not at max
	pub fn key_released(&self, id: &str) -> bool {
		if !self.0.contains_key(&id.to_string()) {
			// TODO Debug
		}

		let key = self.0.get(&id.to_string()).unwrap();
		let mut result = true;
		for i in key {
			match i {
				Binding::KeyboardKey(key) => {
					unsafe { if !raylib::IsKeyReleased(*key as i32) { result = false } }
				}
				Binding::MouseButton(key) => {
					unsafe { if !raylib::IsMouseButtonReleased(*key as i32) { result = false } }
				}
				Binding::MouseAxis(_key) => {
					// TODO: DEBUG doesn't work here
				}
				Binding::GamepadButton{id, key} => {
					unsafe { if !raylib::IsGamepadButtonReleased(*id, *key as i32) { result = false } }
				}
				Binding::GamepadAxis{id, key} => {
					unsafe {
						let value = raylib::GetGamepadAxisMovement(*id, *key as i32);
						if value != 0.0 { result = false }
					}
				}
			}
		}

		result
	}
	/// Are keys currently up / Axis  at max
	pub fn key_up(&self, id: &str) -> bool {
		if !self.0.contains_key(&id.to_string()) {
			// TODO Debug
		}

		let key = self.0.get(&id.to_string()).unwrap();
		let mut result = true;
		for i in key {
			match i {
				Binding::KeyboardKey(key) => {
					unsafe { if !raylib::IsKeyUp(*key as i32) { result = false } }
				}
				Binding::MouseButton(key) => {
					unsafe { if !raylib::IsMouseButtonUp(*key as i32) { result = false } }
				}
				Binding::MouseAxis(_key) => {
					// TODO: DEBUG doesn't work here
				}
				Binding::GamepadButton{id, key} => {
					unsafe { if !raylib::IsGamepadButtonUp(*id, *key as i32) { result = false } }
				}
				Binding::GamepadAxis{id, key} => {
					unsafe {
						let value = raylib::GetGamepadAxisMovement(*id, *key as i32);
						if value != 0.0 { result = false }
					}
				}
			}
		}

		result
	}
	
}