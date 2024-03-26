

use crate::{screen::*, keybindings::*};


//
pub struct Pleroma {
	//* Screen */
	pub screen: Screen,
	//* Sound */
	//* Textures */
	//* Models */
	//* Keybindings */
	pub keys: Keybindings,
	//* Files Loading / Saving */
}

impl Pleroma {
	
	/// Create new gamesystem
	pub fn new() -> Self {
		Self {
			screen: Screen::new(),
			keys: Keybindings::new(),
		}
	}
}