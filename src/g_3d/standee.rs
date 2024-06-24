

use crate::{image::*, model::*, rectangle::Rectangle, texture::*};


pub struct Standee {
	pub model: Option<ModelRl>,
	pub frames: Option<Vec<TextureRl>>,
}
impl Standee {
	
	/// #### new
	/// Creates a new standee.
	pub fn new() -> Self {
		Self {
			model: None,
			frames: None,
		}
	}
	/// #### model
	/// moves model into structure.
	pub fn model(&mut self, model: ModelRl) -> &mut Self {
		self.model = Some(model);
		
		self
	}
	//
	pub fn texture(&mut self, image: Image, size: [i32;2]) -> &mut Self {
		let sprites_hori = image.0.width / size[0];
		let sprites_vert = image.0.height / size[1];
		let sprites_total = sprites_hori * sprites_vert;
		
		let mut textures = Vec::new();
		for i in 0..sprites_total {
			let image = image.from_image(Rectangle{
				x: (i % sprites_hori) as f32,
				y: (i / sprites_vert) as f32,
				width: size[0] as f32,
				height: size[1] as f32,
			});
			textures.push(image.texture());
			image.unload();
		}
		
		self
	}
	
}