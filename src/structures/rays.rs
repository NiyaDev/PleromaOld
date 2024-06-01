

//use crate::camera::Camera2DRl;

use super::vectors::*;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ray {
	pub position: Vector3,
	pub direction: Vector3,
}

impl Ray {
	
	//
	//pub fn mouse()
	
}

//= Screen-space-related functions


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RayCollision {
	pub hit: bool,
	pub distance: f32,
	pub point: Vector3,
	pub normal: Vector3,
}