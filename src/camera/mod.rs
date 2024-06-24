

use std::f32::consts::PI;

use crate::{vectors::*, texture::*, color::*, rectangle::*, rays::*, matrix::*};


/// #### Camera
/// 
#[derive(Debug, Clone, Copy)]
pub struct Camera {
	pub camera_mode: CameraMode,
	
	pub distance:	Vector3,
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
			distance:		Vector3 { x: 0.0, y: 10.0, z: 20.0 },
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
			offset:		Vector2 {x: self.distance.x, y: self.distance.y},
			target:		Vector2 {x: self.target.x, y: self.target.y},
			rotation:	self.rotation,
			zoom:		self.zoom,
		}
	}
}
impl Into<Camera3DRl> for Camera {
	fn into(self) -> Camera3DRl {
		let mut position = Vector3{x:0.0,y:0.0,z:0.0};
					
		position.x = self.distance.x * (self.rotation / 57.3).cos() - self.distance.z * (self.rotation / 57.3).sin();
		position.z = self.distance.x * (self.rotation / 57.3).sin() + self.distance.z * (self.rotation / 57.3).cos();
			
		position.x += self.target.x;
		position.y  = self.target.y + self.distance.y;
		position.z += self.target.z;
		
		Camera3DRl {
			position,
			target:		self.target,
			up:			self.up,
			fovy:		self.fovy,
			projection:	self.projection,
		}
	}
}

impl Camera {
	
	/// #### mode
	/// Sets the camera mode
	pub fn mode(&mut self, mode: CameraMode) -> &mut Self {
		self.camera_mode = mode;
		
		self
	}
	/// #### pan
	/// Moves the camera target and postion by the same amount.
	pub fn pan(&mut self, direction: Vector3) -> &mut Self {
		self.target += direction;
		
		self
	}
	/// #### rotate
	/// Rotate the camera by input around the target.
	pub fn rotate(&mut self, amount: f32) -> &mut Self {
		self.rotation += amount / PI;
		
		self
	}
	
	/// #### billboard
	/// Wrapper for Raylib::DrawBillboard(camera: Camera3DRl, texture: TextureRl, position: Vector3, size: f32, tint: Color).
	pub fn billboard(&mut self, texture: TextureRl, position: Vector3, size: f32, tint: Color) -> &mut Self {
		unsafe{ DrawBillboard((*self).into(), texture, position, size, tint) }
		
		self
	}
	/// #### billboard_rec
	/// Wrapper for Raylib::DrawBillboardEx(camera: Camera3DRl, texture: TextureRl, source: Rectangle, position: Vector3, size: Vector3, tint: Color) -> BoundingBox.
	pub fn billboard_rec(&mut self, texture: TextureRl, source: Rectangle, position: Vector3, size: Vector3, tint: Color) -> &mut Self {
		unsafe{ DrawBillboardRec((*self).into(), texture, source, position, size, tint) }
		
		self
	}
	/// #### billboard_pro
	/// Wrapper for Raylib::GetModelBoundingBox(camera: Camera3DRl, texture: TextureRl, source: Rectangle, position: Vector3, up: Vector3, size: Vector3, origin: Vector3, rotation: f32) -> BoundingBox.
	pub fn billboard_pro(&mut self, texture: TextureRl, source: Rectangle, position: Vector3, up: Vector3, size: Vector3, origin: Vector3, rotation: f32, tint: Color) -> &mut Self {
		unsafe{ DrawBillboardPro((*self).into(), texture, source, position, up, size, origin, rotation, tint) }
		
		self
	}
	
	/// #### mouse_ray
	/// Creates a ray using the current mouse position.
	pub fn mouse_ray(&mut self) -> Ray {
		unsafe{
			let pos = GetMousePosition();
			GetMouseRay(pos, (*self).into())
		}
	}
	/// #### get_mouse
	/// Wrapper for getting the camera matrix
	pub fn get_matrix(&mut self) -> Matrix {
		unsafe{
			match self.camera_mode {
				CameraMode::Mode2D => {
					GetCameraMatrix2D((*self).into())
				}
				CameraMode::Mode3D => {
					GetCameraMatrix((*self).into())
				}
			}
		}
	}
	
}

//= Model drawing functions
extern "C" { fn DrawBillboard(camera: Camera3DRl, texture: TextureRl, position: Vector3, size: f32, tint: Color); }
extern "C" { fn DrawBillboardRec(camera: Camera3DRl, texture: TextureRl, source: Rectangle, position: Vector3, size: Vector3, tint: Color); }
extern "C" { fn DrawBillboardPro(camera: Camera3DRl, texture: TextureRl, source: Rectangle, position: Vector3, up: Vector3, size: Vector3, origin: Vector3, rotation: f32, tint: Color); }

//= Screen-space-related functions
extern "C" { fn GetMouseRay(mousePosition: Vector2, camera: Camera2DRl) -> Ray; }
extern "C" { fn GetCameraMatrix(camera: Camera3DRl) -> Matrix; }
extern "C" { fn GetCameraMatrix2D(camera: Camera2DRl) -> Matrix; }

//= Input-related functions: mouse
extern "C" { fn GetMousePosition() -> Vector2; }


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