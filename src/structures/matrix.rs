

use super::vectors::{Quaternion, Vector3, Vector4};


/// Matrix type (OpenGL style 4x4 - right handed, column major)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
	pub m0: f32, pub m4: f32, pub  m8: f32, pub m12: f32,
	pub m1: f32, pub m5: f32, pub  m9: f32, pub m13: f32,
	pub m2: f32, pub m6: f32, pub m10: f32, pub m14: f32,
	pub m3: f32, pub m7: f32, pub m11: f32, pub m15: f32,
}
//impl Add<Self> for Matrix {
//	
//}
//impl Sub<Self> for Matrix {
//	
//}
//impl Mul<Self> for Matrix {
//	
//}

pub const IDENTITY: Matrix = Matrix {
	m0: 1.0,  m4: 0.0, m8:  0.0, m12: 0.0,
	m1: 0.0,  m5: 1.0, m9:  0.0, m13: 0.0,
	m2: 0.0,  m6: 0.0, m10: 1.0, m14: 0.0,
	m3: 0.0,  m7: 0.0, m11: 0.0, m15: 1.0,
};

impl Matrix {
	
	/// Compute matrix determinant
	pub fn determinant(&self) -> f32 {}
	/// Get the trace of the matrix (sum of the values along the diagonal)
	pub fn trace(&self) -> f32 {}
	/// Transposes provided matrix
	pub fn transpose(&self) -> Self {}
	/// Invert provided matrix
	pub fn invert(&self) -> Self {}
	/// Get translation matrix
	pub fn translate(v: Vector3) -> Self {}
	/// Create rotation matrix from axis and angle
	/// 
	/// NOTE: Angle should be provided in radians
	pub fn rotate(axis: Vector3, angle: f32) -> Self {}
	/// Get x-rotation matrix
	/// 
	/// NOTE: Angle should be provided in radians
	pub fn rotate_x(angle: f32) -> Self {}
	/// Get y-rotation matrix
	/// 
	/// NOTE: Angle should be provided in radians
	pub fn rotate_y(angle: f32) -> Self {}
	/// Get z-rotation matrix
	/// 
	/// NOTE: Angle should be provided in radians
	pub fn rotate_z(angle: f32) -> Self {}
	/// Get xyz-rotation matrix
	/// 
	/// NOTE: Angle should be provided in radians
	pub fn rotate_xyz(angle: f32) -> Self {}
	/// Get zyx-rotation matrix
	/// 
	/// NOTE: Angle should be provided in radians
	pub fn rotate_zyx(angle: f32) -> Self {}
	/// Get scaling matrix
	pub fn scale(scale: Vector3) -> Self {}
	/// Get perspective projection matrix
	pub fn frustum(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> Self {}
	/// Get perspective projection matrix
	/// 
	/// NOTE: Fovy angle must be provided in radians
	pub fn perspective(fov_y: f64, aspect: f64, near_plane: f64, far_plane: f64) -> Self {}
	/// Get orthographic projection matrix
	pub fn ortho(left: f64, right: f64, bottom: f64, top: f64, near_plane: f64, far_plane: f64) -> Self {}
	/// Get camera look-at matrix (view matrix)
	pub fn look_at(eye: Vector3, target: Vector3, up: Vector3) -> Self {}
	/// Transform a quaternion given a transformation matrix
	pub fn quaternion_transform(&self, q: Quaternion) -> Vector4 {
		Vector4 {
			x: self.m0 * q.x + self.m4 * q.y + self.m8  * q.z + self.m12 * q.w,
			y: self.m1 * q.x + self.m5 * q.y + self.m9  * q.z + self.m13 * q.w,
			z: self.m2 * q.x + self.m6 * q.y + self.m10 * q.z + self.m14 * q.w,
			w: self.m3 * q.x + self.m7 * q.y + self.m11 * q.z + self.m15 * q.w,
		}
	}

}