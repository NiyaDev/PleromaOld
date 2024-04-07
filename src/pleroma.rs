

use std::collections::HashMap;
use crate::{
	screen::*,
	keybindings::*,
	structures::texture::*,
};


//
pub struct Pleroma {
	//* Screen */
	pub screen: Screen,
	//* Sound */
	//* Textures */
	pub textures: HashMap<String, Texture>,
	//* Models */
	//* Keybindings */
	pub keys: Keybindings,
	//* Files Loading / Saving */
}

impl Pleroma {
	
	/// Create new gamesystem
	pub fn new() -> Self {
		Self {
			screen:		Screen::new(),
			textures:	HashMap::new(),
			keys:		Keybindings::new(),
		}
	}
	
}