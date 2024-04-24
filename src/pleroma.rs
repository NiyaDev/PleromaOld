

use std::collections::HashMap;
use crate::{
	screen::*,
	keybindings::*,
	structures::{
		texture::*,
		font::*,
	},
};


//
pub struct Pleroma {
	pub screen: Screen,
	//* Sound */
	pub textures: HashMap<String, Texture>,
	pub fonts: HashMap<String, Font>,
	//* Models */
	pub keys: Keybindings,
	//* File Loading / Saving */
}

impl Pleroma {
	
	/// Create new gamesystem
	pub fn new() -> Self {
		Self {
			screen:		Screen::new(),
			textures:	HashMap::new(),
			fonts:		HashMap::new(),
			keys:		Keybindings::new(),
		}
	}
	
}