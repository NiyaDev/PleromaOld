

use std::collections::HashMap;
use crate::structures::rectangle::*;


/// #### Animator
/// 2D animation handler.
#[derive(Debug, Default)]
pub struct Animator {
	pub animations: HashMap<String, Animation>,
	pub current_animation: String,
	pub current_frame: usize,
	pub timer: i32,
	pub tick: i32,
}

/// #### Animation
/// Simple wrapper for a vec that contains the animation frames in order.
#[derive(Debug, Default, Clone)]
pub struct Animation(pub Vec<Rectangle>);
impl Animation {
	
	/// #### new
	/// Creates a new animation using a slice of i32.
	pub fn new(input: &[Rectangle]) -> Self {
		let mut result = Animation(Vec::new());
		for i in input.iter() {
			result.0.push(*i);
		}
		
		result
	}
	
}