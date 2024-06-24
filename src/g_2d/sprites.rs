

use super::animator::*;
use crate::{color::*, texture::*, vectors::*};


/// #### Sprite
/// An animatable sprite
pub struct Sprite {
	pub texture: TextureRl,
	pub animator: Animator,
	pub tint: Color,
}
impl Sprite {
	
	/// #### draw
	/// Draws the current frame of the animation to screen.
	pub fn draw(&mut self, position: Vector2) -> &mut Self {
		self.texture.draw_rec(self.animator.animations[&self.animator.current_animation].0[self.animator.current_frame], position, self.tint);
		
		self.update()
	}
	/// #### update
	/// Updates animation controller. Called automatically at the end of draw.
	pub fn update(&mut self) -> &mut Self {
		self.animator.timer += 1;
		if self.animator.timer >= self.animator.tick {
			self.animator.timer = 0;
			self.animator.current_frame += 1;
			if self.animator.current_frame >= self.animator.animations[&self.animator.current_animation].0.len() {
				self.animator.current_frame = 0;
			}
		}
		
		self
	}
	
}