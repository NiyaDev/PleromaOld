

use super::{vectors::*, color::*};


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
	pub min: Vector3,
	pub max: Vector3,
}

impl BoundingBox {
	
	/// #### draw
	/// Wrapper for Raylib::DrawBoundingBox(bounds: BoundingBox, color: Color).
	pub fn draw(&mut self, color: Color) -> &mut Self {
		unsafe{ DrawBoundingBox(*self, color) }
		
		self
	}
	
}

//= Model drawing functions
extern "C" { fn DrawBoundingBox(bounds: BoundingBox, color: Color); }
