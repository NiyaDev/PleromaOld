

pub mod keyboard;
pub mod mouse;
pub mod gamepad;

use std::collections::HashMap;


//
pub struct Keybindings(HashMap<String, Binding>);

//
pub enum Binding {
	Keyboard(Vec<keyboard::Key>),
	MouseButton(mouse::Button),
	MouseAxis(mouse::Axis),
	GamepadButton(i32, Vec<gamepad::GamepadButton>),
	GamepadAxis(i32, gamepad::GamepadAxis),
}

pub enum Results {
	None,
	Success,
	
}


impl Keybindings {
	
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	//= Manipulation

	///
	pub fn insert_press(&mut self, key: String) -> Results {
		//* Check if key already exists, delete it if it does. */
		if self.0.contains_key(&key) {
			// TODO Debug
			// TODO "[WARNING] - Overwrote keybinding {:?} with {:?}.", (key, self.0.get(&key)), (key, binding)
			self.0.remove(&key);
		}

		//* Get keyboard queue */
		let mut queue: Vec<keyboard::Key> = Vec::new();
		loop {
			unsafe {
				let member = keyboard::Key::from(GetKeyPressed());
				if member != keyboard::Key::Null {
					if !queue.contains(&member) { queue.push(member) }
				} else { break }
			}
		}
		//* Insert keybinding */
		self.0.insert(key, Binding::Keyboard(queue));


		Results::None
	}

	///
	pub fn test() {
		unsafe {
			let mut queue: Vec<keyboard::Key> = Vec::new();
			loop {
				let member = keyboard::Key::from(GetKeyPressed());
				if member != keyboard::Key::Null {
					
					queue.push(member)
				} else { break }
			}
			
			if queue.len() != 0 { println!("{:?}",queue) }
		}
	}

	/// Insert a new key into the bindings
	pub fn insert(&mut self, key: String, binding: Binding) -> Results {
		//* Check if binding already exists, delete it if it does. */
		if self.0.contains_key(&key) {
			// TODO Debug
			// TODO "[WARNING] - Overwrote keybinding {:?} with {:?}.", (key, self.0.get(&key)), (key, binding)
			self.0.remove(&key);
		}
		
		//* Insert new key */
		self.0.insert(key, binding);
		
		Results::Success
	}

}

extern "C" { pub fn GetKeyPressed() -> i32; }