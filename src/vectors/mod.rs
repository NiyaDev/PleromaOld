

//= Allows


//= Imports


//= Structures & Enumerations

/// 
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}

/// 
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

/// 
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}


//= Procedures

//pub fn init_window(width: i32, height: i32, title: &str) {
//	extern "C" {
//		pub
//	}
//}
//Test
extern "C" {
	pub fn InitWindow(
		width:	i32,
		height:	i32,
		title:	*const u8,
	);
}