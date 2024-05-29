

use crate::structures::vectors::*;


/// #### Camera
/// 
#[derive(Debug, Clone, Copy)]
pub struct Camera {
	pub camera_mode: CameraMode,
	
	pub position:	Vector3,
	pub target:		Vector3,
	pub up:			Vector3,
	pub rotation:	f32,
	pub zoom:		f32,
	pub fovy:		f32,
	pub projection:	CameraProjection,
}
impl Default for Camera {
	fn default() -> Self {
		Self {
			camera_mode:	Default::default(),
			position:		Vector3 { x: 0.0, y: 0.0, z: 5.0 },
			target:			ZERO_3,
			up:				Vector3{ x: 0.0, y: 1.0, z: 0.0 },
			rotation:		0.0,
			zoom:			1.0,
			fovy:			45.0,
			projection:		Default::default(),
		}
	}
}
impl Into<Camera2DRl> for Camera {
	fn into(self) -> Camera2DRl {
		Camera2DRl {
			offset:		Vector2 {x: self.position.x, y: self.position.y},
			target:		Vector2 {x: self.target.x, y: self.target.y},
			rotation:	self.rotation,
			zoom:		self.zoom,
		}
	}
}
impl Into<Camera3DRl> for Camera {
	fn into(self) -> Camera3DRl {
		Camera3DRl {
			position:	self.position,
			target:		self.target,
			up:			self.up,
			fovy:		self.fovy,
			projection:	self.projection,
		}
	}
}

impl Camera {
	
	/// #### pan
	/// Moves the camera target and postion by the same amount.
	pub fn pan(&mut self, direction: Vector3) -> &mut Self {
		self.position += direction;
		self.target += direction;
		
		self
	}
	
}


/// #### Camera mode
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub enum CameraMode {
	#[default]
	Mode2D,
	Mode3D,
}

//= Raylib structures

/// #### Camera2D raylib
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera2DRl {
	pub offset: Vector2,
	pub target: Vector2,
	pub rotation: f32,
	pub zoom: f32,
}

/// #### Camera3D raylib
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera3DRl {
	pub position: Vector3,
	pub target: Vector3,
	pub up: Vector3,
	pub fovy: f32,
	pub projection: CameraProjection,
}

/// #### CameraProjection raylib
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub enum CameraProjection {
	#[default]
	Perspective = 0,
	Orthographic = 1,
}