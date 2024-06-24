

use std::{collections::HashMap, str::from_utf8};
use crate::{color::*, model::*, vectors::*};


#[derive(Debug)]
pub struct Model {
	pub model: ModelRl,
	
	pub position: Vector3,
	pub rotation_axis: Vector3,
	pub rotation_angle: f32,
	pub scale: Vector3,
	
	pub animations: HashMap<String, ModelAnimation>,
	pub current_animation: String,
	pub timer: i32,
	pub current_frame: i32,
	
	pub tint: Color,
}
impl Model {
	
	/// #### load
	/// Loads model from file.
	pub fn load(filename: &str) -> Self {
		let model = ModelRl::load(filename);
		
		let animations_list = ModelAnimation::load(filename);
		let mut animations = HashMap::new();
		for i in animations_list.iter() {
			let mut len = 0;
			for l in i.name.iter() { if *l != b'\0' { len += 1 } }
			animations.insert(from_utf8(&i.name[0..len]).unwrap().to_string(), i.clone());
		}
		
		Self{
			model,
			position: Vector3{x: 0.0, y: 0.0, z: 0.0},
			rotation_axis: Vector3{x: 0.0, y: 1.0, z: 0.0},
			rotation_angle: 0.0,
			scale: Vector3{x: 1.0, y: 1.0, z: 1.0},
			animations,
			current_animation: "".to_string(),
			timer: 0,
			current_frame: 0,
			tint: WHITE,
		}
	}
	
	/// #### set_animation
	/// sets the current animation if it exists and resets the current frame and timer. If it doesn't, send error.
	pub fn set_animation(&mut self, name: &str) -> &mut Self {
		if self.animations.contains_key(&name.to_string()) {
			self.current_animation = name.to_string();
			self.current_frame = 0;
			self.timer = 0;
		} else {
			println!("set_animation: Failed to find model animation named: {}.", name);
		}
		
		
		self
	}
	
	/// #### draw
	/// Draws the model and uopdates animation.
	pub fn draw(&mut self) -> &mut Self {
		self.model.draw_ex(self.position, self.rotation_axis, self.rotation_angle, self.scale, self.tint);
		
		if self.current_animation != "".to_string() {
			self.timer += 1;
			if self.timer >= 1 {
				self.timer = 0;
				self.current_frame += 1;
				
				let anim = self.animations.get(&self.current_animation);
				if anim.is_some() {
					if self.current_frame >= anim.unwrap().frame_count { self.current_frame = 0 }
					self.animations.get_mut(&self.current_animation).unwrap().update(self.model, self.current_frame);
				} else {
					println!("draw: Failed to find model animation named: {}.", self.current_animation);
				}
			}
		}
		
		self
	}
	
}