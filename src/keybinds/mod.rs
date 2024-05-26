

use crate::{pleroma::Pleroma, structures::vectors::Vector2};

pub mod keyboard;
pub mod mouse;
pub mod gamepad;


pub struct Keybind {
	pub device: Device,
	pub keys: [i32;2],
}

pub enum Device {
	Keyboard,
	Mouse,
	Gamepad(i32),
}

impl Pleroma {
	
	/// ### add_keybind
	/// Adds the inputted keybinding into the system.
	pub fn add_keybind(&mut self, name: &str, device: Device, keys: [i32;2]) -> &mut Self {
		let keybind = Keybind{ device, keys };
		self.keybindings.insert(name.to_string(), keybind);
		
		self
	}
	//= Checking
	/// ### is_pressed
	/// Checks input based on device.
	/// 
	/// For Keyboards, it simply checks if the mod key is down and if the main key is pressed once.
	/// 
	/// For Mice, if the key is a button on the mouse it checks if it's been pressed.
	/// Otherwise it checks if the mouse delta has changed since the las frame.
	/// 
	/// For controllers it does a combination of both. Checking modifiers if its a button and delta if it was an axis.
	pub fn is_pressed(&self, name: &str) -> bool {
		unsafe{
			let binding = self.keybindings.get(name).unwrap();
			match binding.device {
				Device::Keyboard => {
					let is_mod = if binding.keys[0] != 0 { IsKeyDown(binding.keys[0]) } else { true };
					return is_mod && IsKeyPressed(binding.keys[1])
				}
				Device::Mouse => {
					match binding.keys[1] {
						7  => { GetMouseDelta().x > 0.0 }
						8  => { GetMouseDelta().x < 0.0 }
						9  => { GetMouseDelta().y > 0.0 }
						10 => { GetMouseDelta().y < 0.0 }
						11 => { GetMouseWheelMove() > 0.0 }
						12 => { GetMouseWheelMove() < 0.0 }
						_  => { IsMouseButtonPressed(binding.keys[1]) }
					}
				}
				Device::Gamepad(id) => {
					if !IsGamepadAvailable(id) { return false }
					match binding.keys[1] {
						18 => { GetGamepadAxisMovement(id, 0) > 0.0 }
						19 => { GetGamepadAxisMovement(id, 0) < 0.0 }
						20 => { GetGamepadAxisMovement(id, 1) > 0.0 }
						21 => { GetGamepadAxisMovement(id, 1) < 0.0 }
						22 => { GetGamepadAxisMovement(id, 2) > 0.0 }
						23 => { GetGamepadAxisMovement(id, 2) < 0.0 }
						24 => { GetGamepadAxisMovement(id, 3) > 0.0 }
						25 => { GetGamepadAxisMovement(id, 3) < 0.0 }
						_ => {
							let is_mod = if binding.keys[0] != 0 { IsGamepadButtonDown(id, binding.keys[0]) } else { true };
							is_mod && IsGamepadButtonPressed(id, binding.keys[1])
						}
					}
				}
			}
		}
	}
	/// ### is_down
	/// Checks input based on device.
	/// 
	/// For Keyboards, it simply checks if the mod key is down and if the main key is pressed once.
	/// 
	/// For Mice, if the key is a button on the mouse it checks if it's been pressed.
	/// Otherwise it checks if the mouse delta has changed since the las frame.
	/// 
	/// For controllers it does a combination of both. Checking modifiers if its a button and delta if it was an axis.
	pub fn is_down(&self, name: &str) -> bool {
		unsafe{
			let binding = self.keybindings.get(name).unwrap();
			match binding.device {
				Device::Keyboard => {
					let is_mod = if binding.keys[0] != 0 { IsKeyDown(binding.keys[0]) } else { true };
					return is_mod && IsKeyDown(binding.keys[1])
				}
				Device::Mouse => {
					match binding.keys[1] {
						7  => { GetMouseDelta().x > 0.0 }
						8  => { GetMouseDelta().x < 0.0 }
						9  => { GetMouseDelta().y > 0.0 }
						10 => { GetMouseDelta().y < 0.0 }
						11 => { GetMouseWheelMove() > 0.0 }
						12 => { GetMouseWheelMove() < 0.0 }
						_  => { IsMouseButtonDown(binding.keys[1]) }
					}
				}
				Device::Gamepad(id) => {
					if !IsGamepadAvailable(id) { return false }
					match binding.keys[1] {
						18 => { GetGamepadAxisMovement(id, 0) > 0.0 }
						19 => { GetGamepadAxisMovement(id, 0) < 0.0 }
						20 => { GetGamepadAxisMovement(id, 1) > 0.0 }
						21 => { GetGamepadAxisMovement(id, 1) < 0.0 }
						22 => { GetGamepadAxisMovement(id, 2) > 0.0 }
						23 => { GetGamepadAxisMovement(id, 2) < 0.0 }
						24 => { GetGamepadAxisMovement(id, 3) > 0.0 }
						25 => { GetGamepadAxisMovement(id, 3) < 0.0 }
						_ => {
							let is_mod = if binding.keys[0] != 0 { IsGamepadButtonDown(id, binding.keys[0]) } else { true };
							is_mod && IsGamepadButtonDown(id, binding.keys[1])
						}
					}
				}
			}
		}
	}
	/// ### is_released
	/// Checks input based on device.
	/// 
	/// For Keyboards, it simply checks if the mod key is down and if the main key is pressed once.
	/// 
	/// For Mice, if the key is a button on the mouse it checks if it's been pressed.
	/// Otherwise it checks if the mouse delta has changed since the las frame.
	/// 
	/// For controllers it does a combination of both. Checking modifiers if its a button and delta if it was an axis.
	pub fn is_released(&self, name: &str) -> bool {
		unsafe{
			let binding = self.keybindings.get(name).unwrap();
			match binding.device {
				Device::Keyboard => {
					let is_mod = if binding.keys[0] != 0 { IsKeyDown(binding.keys[0]) } else { true };
					return is_mod && IsKeyReleased(binding.keys[1])
				}
				Device::Mouse => {
					match binding.keys[1] {
						7  => { GetMouseDelta().x > 0.0 }
						8  => { GetMouseDelta().x < 0.0 }
						9  => { GetMouseDelta().y > 0.0 }
						10 => { GetMouseDelta().y < 0.0 }
						11 => { GetMouseWheelMove() > 0.0 }
						12 => { GetMouseWheelMove() < 0.0 }
						_  => { IsMouseButtonReleased(binding.keys[1]) }
					}
				}
				Device::Gamepad(id) => {
					if !IsGamepadAvailable(id) { return false }
					match binding.keys[1] {
						18 => { GetGamepadAxisMovement(id, 0) > 0.0 }
						19 => { GetGamepadAxisMovement(id, 0) < 0.0 }
						20 => { GetGamepadAxisMovement(id, 1) > 0.0 }
						21 => { GetGamepadAxisMovement(id, 1) < 0.0 }
						22 => { GetGamepadAxisMovement(id, 2) > 0.0 }
						23 => { GetGamepadAxisMovement(id, 2) < 0.0 }
						24 => { GetGamepadAxisMovement(id, 3) > 0.0 }
						25 => { GetGamepadAxisMovement(id, 3) < 0.0 }
						_ => {
							let is_mod = if binding.keys[0] != 0 { IsGamepadButtonDown(id, binding.keys[0]) } else { true };
							is_mod && IsGamepadButtonReleased(id, binding.keys[1])
						}
					}
				}
			}
		}
	}
	/// ### is_up
	/// Checks input based on device.
	/// 
	/// For Keyboards, it simply checks if the mod key is down and if the main key is pressed once.
	/// 
	/// For Mice, if the key is a button on the mouse it checks if it's been pressed.
	/// Otherwise it checks if the mouse delta has changed since the las frame.
	/// 
	/// For controllers it does a combination of both. Checking modifiers if its a button and delta if it was an axis.
	pub fn is_up(&self, name: &str) -> bool {
		unsafe{
			let binding = self.keybindings.get(name).unwrap();
			match binding.device {
				Device::Keyboard => {
					let is_mod = if binding.keys[0] != 0 { IsKeyDown(binding.keys[0]) } else { true };
					return is_mod && IsKeyUp(binding.keys[1])
				}
				Device::Mouse => {
					match binding.keys[1] {
						7  => { GetMouseDelta().x > 0.0 }
						8  => { GetMouseDelta().x < 0.0 }
						9  => { GetMouseDelta().y > 0.0 }
						10 => { GetMouseDelta().y < 0.0 }
						11 => { GetMouseWheelMove() > 0.0 }
						12 => { GetMouseWheelMove() < 0.0 }
						_  => { IsMouseButtonUp(binding.keys[1]) }
					}
				}
				Device::Gamepad(id) => {
					if !IsGamepadAvailable(id) { return false }
					match binding.keys[1] {
						18 => { GetGamepadAxisMovement(id, 0) > 0.0 }
						19 => { GetGamepadAxisMovement(id, 0) < 0.0 }
						20 => { GetGamepadAxisMovement(id, 1) > 0.0 }
						21 => { GetGamepadAxisMovement(id, 1) < 0.0 }
						22 => { GetGamepadAxisMovement(id, 2) > 0.0 }
						23 => { GetGamepadAxisMovement(id, 2) < 0.0 }
						24 => { GetGamepadAxisMovement(id, 3) > 0.0 }
						25 => { GetGamepadAxisMovement(id, 3) < 0.0 }
						_ => {
							let is_mod = if binding.keys[0] != 0 { IsGamepadButtonDown(id, binding.keys[0]) } else { true };
							is_mod && IsGamepadButtonUp(id, binding.keys[1])
						}
					}
				}
			}
		}
	}
	/// ### get_axis
	/// Checks axis-based input based on device.
	/// 
	/// Ignores keyboards.
	/// 
	/// For Mice, returns the value of the delta of that direction.
	/// 
	/// For controllers, returns the movement of each axis.
	pub fn get_axis(&self, name: &str) -> f32 {
		unsafe{
			let binding = self.keybindings.get(name).unwrap();
			match binding.device {
				Device::Keyboard => { return 0.0 }
				Device::Mouse => {
					if binding.keys[1] == 7 || binding.keys[1] == 8 {
						GetMouseDelta().x
					} else if binding.keys[1] == 9 || binding.keys[1] == 10 {
						GetMouseDelta().y
					} else { 0.0 }
				}
				Device::Gamepad(id) => {
					match binding.keys[1] {
						18 | 19 => { GetGamepadAxisMovement(id, 0) }
						20 | 21 => { GetGamepadAxisMovement(id, 1) }
						22 | 23 => { GetGamepadAxisMovement(id, 2) }
						24 | 25 => { GetGamepadAxisMovement(id, 3) }
						26 => { GetGamepadAxisMovement(id, 4) }
						27 => { GetGamepadAxisMovement(id, 5) }
						_ => { 0.0 }
					}
				}
			}
		}
	}
	
}

//= Input-related functions: keyboard
extern "C" { fn IsKeyPressed(key: i32) -> bool; }
extern "C" { fn IsKeyDown(key: i32) -> bool; }
extern "C" { fn IsKeyReleased(key: i32) -> bool; }
extern "C" { fn IsKeyUp(key: i32) -> bool; }

//= Input-related functions: gamepads
extern "C" { fn IsGamepadAvailable(gamepad: i32) -> bool; }
extern "C" { fn IsGamepadButtonPressed(gamepad: i32, button: i32) -> bool; }
extern "C" { fn IsGamepadButtonDown(gamepad: i32, button: i32) -> bool; }
extern "C" { fn IsGamepadButtonReleased(gamepad: i32, button: i32) -> bool; }
extern "C" { fn IsGamepadButtonUp(gamepad: i32, button: i32) -> bool; }
extern "C" { fn GetGamepadAxisMovement(gamepad: i32, axis: i32) -> f32; }

//= Input-related functions: mouse
extern "C" { fn IsMouseButtonPressed(button: i32) -> bool; }
extern "C" { fn IsMouseButtonDown(button: i32) -> bool; }
extern "C" { fn IsMouseButtonReleased(button: i32) -> bool; }
extern "C" { fn IsMouseButtonUp(button: i32) -> bool; }
extern "C" { fn GetMouseDelta() -> Vector2; }
extern "C" { fn GetMouseWheelMove() -> f32; }
