

use std::ops::{
	Add, Sub,
	Mul,
};

use super::vectors::{
	Vector3,
	Vector4,
	Quaternion,
};


/// Matrix type (OpenGL style 4x4 - right handed, column major)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
	pub m0: f32, pub m4: f32, pub  m8: f32, pub m12: f32,
	pub m1: f32, pub m5: f32, pub  m9: f32, pub m13: f32,
	pub m2: f32, pub m6: f32, pub m10: f32, pub m14: f32,
	pub m3: f32, pub m7: f32, pub m11: f32, pub m15: f32,
}
impl Add<Self> for Matrix {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let mut result = ZERO;

		result.m0  = self.m0  + rhs.m0;
		result.m1  = self.m1  + rhs.m1;
		result.m2  = self.m2  + rhs.m2;
		result.m3  = self.m3  + rhs.m3;
		result.m4  = self.m4  + rhs.m4;
		result.m5  = self.m5  + rhs.m5;
		result.m6  = self.m6  + rhs.m6;
		result.m7  = self.m7  + rhs.m7;
		result.m8  = self.m8  + rhs.m8;
		result.m9  = self.m9  + rhs.m9;
		result.m10 = self.m10 + rhs.m10;
		result.m11 = self.m11 + rhs.m11;
		result.m12 = self.m12 + rhs.m12;
		result.m13 = self.m13 + rhs.m13;
		result.m14 = self.m14 + rhs.m14;
		result.m15 = self.m15 + rhs.m15;

		result
	}
}
impl Sub<Self> for Matrix {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		let mut result = ZERO;

		result.m0  = self.m0  - rhs.m0;
		result.m1  = self.m1  - rhs.m1;
		result.m2  = self.m2  - rhs.m2;
		result.m3  = self.m3  - rhs.m3;
		result.m4  = self.m4  - rhs.m4;
		result.m5  = self.m5  - rhs.m5;
		result.m6  = self.m6  - rhs.m6;
		result.m7  = self.m7  - rhs.m7;
		result.m8  = self.m8  - rhs.m8;
		result.m9  = self.m9  - rhs.m9;
		result.m10 = self.m10 - rhs.m10;
		result.m11 = self.m11 - rhs.m11;
		result.m12 = self.m12 - rhs.m12;
		result.m13 = self.m13 - rhs.m13;
		result.m14 = self.m14 - rhs.m14;
		result.m15 = self.m15 - rhs.m15;

		result
	}
}
impl Mul<Self> for Matrix {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		let mut result = ZERO;
 
		result.m0  = self.m0  * rhs.m0 + self.m1  * rhs.m4 + self.m2  * rhs.m8  + self.m3  * rhs.m12;
		result.m1  = self.m0  * rhs.m1 + self.m1  * rhs.m5 + self.m2  * rhs.m9  + self.m3  * rhs.m13;
		result.m2  = self.m0  * rhs.m2 + self.m1  * rhs.m6 + self.m2  * rhs.m10 + self.m3  * rhs.m14;
		result.m3  = self.m0  * rhs.m3 + self.m1  * rhs.m7 + self.m2  * rhs.m11 + self.m3  * rhs.m15;
		result.m4  = self.m4  * rhs.m0 + self.m5  * rhs.m4 + self.m6  * rhs.m8  + self.m7  * rhs.m12;
		result.m5  = self.m4  * rhs.m1 + self.m5  * rhs.m5 + self.m6  * rhs.m9  + self.m7  * rhs.m13;
		result.m6  = self.m4  * rhs.m2 + self.m5  * rhs.m6 + self.m6  * rhs.m10 + self.m7  * rhs.m14;
		result.m7  = self.m4  * rhs.m3 + self.m5  * rhs.m7 + self.m6  * rhs.m11 + self.m7  * rhs.m15;
		result.m8  = self.m8  * rhs.m0 + self.m9  * rhs.m4 + self.m10 * rhs.m8  + self.m11 * rhs.m12;
		result.m9  = self.m8  * rhs.m1 + self.m9  * rhs.m5 + self.m10 * rhs.m9  + self.m11 * rhs.m13;
		result.m10 = self.m8  * rhs.m2 + self.m9  * rhs.m6 + self.m10 * rhs.m10 + self.m11 * rhs.m14;
		result.m11 = self.m8  * rhs.m3 + self.m9  * rhs.m7 + self.m10 * rhs.m11 + self.m11 * rhs.m15;
		result.m12 = self.m12 * rhs.m0 + self.m13 * rhs.m4 + self.m14 * rhs.m8  + self.m15 * rhs.m12;
		result.m13 = self.m12 * rhs.m1 + self.m13 * rhs.m5 + self.m14 * rhs.m9  + self.m15 * rhs.m13;
		result.m14 = self.m12 * rhs.m2 + self.m13 * rhs.m6 + self.m14 * rhs.m10 + self.m15 * rhs.m14;
		result.m15 = self.m12 * rhs.m3 + self.m13 * rhs.m7 + self.m14 * rhs.m11 + self.m15 * rhs.m15;

		result
	}
}

pub const ZERO: Matrix = Matrix {
	m0: 0.0,  m4: 0.0, m8:  0.0, m12: 0.0,
	m1: 0.0,  m5: 0.0, m9:  0.0, m13: 0.0,
	m2: 0.0,  m6: 0.0, m10: 0.0, m14: 0.0,
	m3: 0.0,  m7: 0.0, m11: 0.0, m15: 0.0,
};

pub const IDENTITY: Matrix = Matrix {
	m0: 1.0,  m4: 0.0, m8:  0.0, m12: 0.0,
	m1: 0.0,  m5: 1.0, m9:  0.0, m13: 0.0,
	m2: 0.0,  m6: 0.0, m10: 1.0, m14: 0.0,
	m3: 0.0,  m7: 0.0, m11: 0.0, m15: 1.0,
};

impl Matrix {
	
	/// Compute matrix determinant
	pub fn determinant(&self) -> f32 {
		self.m12 * self.m9 * self.m6*self.m3 - self.m8 * self.m13 * self.m6 * self.m3 - self.m12 * self.m5 * self.m10 * self.m3 + self.m4*self.m13*self.m10*self.m3 +
			self.m8  *self.m5 * self.m14 * self.m3  - self.m4 * self.m9  * self.m14 * self.m3  - self.m12 * self.m9 * self.m2  * self.m7  + self.m8 * self.m13 * self.m2  * self.m7  +
			self.m12 *self.m1 * self.m10 * self.m7  - self.m0 * self.m13 * self.m10 * self.m7  - self.m8  * self.m1 * self.m14 * self.m7  + self.m0 * self.m9  * self.m14 * self.m7  +
			self.m12 *self.m5 * self.m2  * self.m11 - self.m4 * self.m13 * self.m2  * self.m11 - self.m12 * self.m1 * self.m6  * self.m11 + self.m0 * self.m13 * self.m6  * self.m11 +
			self.m4  *self.m1 * self.m14 * self.m11 - self.m0 * self.m5  * self.m14 * self.m11 - self.m8  * self.m5 * self.m2  * self.m15 + self.m4 * self.m9  * self.m2  * self.m15 +
			self.m8  *self.m1 * self.m6  * self.m15 - self.m0 * self.m9  * self.m6  * self.m15 - self.m4  * self.m1 * self.m10 * self.m15 + self.m0 * self.m5  * self.m10 * self.m15
	}
	/// Get the trace of the matrix (sum of the values along the diagonal)
	pub fn trace(&self) -> f32 {
		self.m0 + self.m5 + self.m10 + self.m15
	}
	/// Transposes provided matrix
	pub fn transpose(&self) -> Self {
		let mut result = ZERO;

		result.m0  = self.m0;
		result.m1  = self.m4;
		result.m2  = self.m8;
		result.m3  = self.m12;
		result.m4  = self.m1;
		result.m5  = self.m5;
		result.m6  = self.m9;
		result.m7  = self.m13;
		result.m8  = self.m2;
		result.m9  = self.m6;
		result.m10 = self.m10;
		result.m11 = self.m14;
		result.m12 = self.m3;
		result.m13 = self.m7;
		result.m14 = self.m11;
		result.m15 = self.m15;

		result
	}
	/// Invert provided matrix
	pub fn invert(&self) -> Self {
		let mut result = ZERO;

		// Cache the matrix values (speed optimization)
		let a00 = self.m0;
		let a01 = self.m1;
		let a02 = self.m2;
		let a03 = self.m3;
		let a10 = self.m4;
		let a11 = self.m5;
		let a12 = self.m6;
		let a13 = self.m7;
		let a20 = self.m8;
		let a21 = self.m9;
		let a22 = self.m10;
		let a23 = self.m11;
		let a30 = self.m12;
		let a31 = self.m13;
		let a32 = self.m14;
		let a33 = self.m15;

		let b00 = a00 * a11 - a01 * a10;
		let b01 = a00 * a12 - a02 * a10;
		let b02 = a00 * a13 - a03 * a10;
		let b03 = a01 * a12 - a02 * a11;
		let b04 = a01 * a13 - a03 * a11;
		let b05 = a02 * a13 - a03 * a12;
		let b06 = a20 * a31 - a21 * a30;
		let b07 = a20 * a32 - a22 * a30;
		let b08 = a20 * a33 - a23 * a30;
		let b09 = a21 * a32 - a22 * a31;
		let b10 = a21 * a33 - a23 * a31;
		let b11 = a22 * a33 - a23 * a32;

		// Calculate the invert determinant (inlined to avoid double-caching)
		let inv_det = 1.0 / (b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06);

		result.m0  = ( a11 * b11 - a12 * b10 + a13 * b09) * inv_det;
		result.m1  = (-a01 * b11 + a02 * b10 - a03 * b09) * inv_det;
		result.m2  = ( a31 * b05 - a32 * b04 + a33 * b03) * inv_det;
		result.m3  = (-a21 * b05 + a22 * b04 - a23 * b03) * inv_det;
		result.m4  = (-a10 * b11 + a12 * b08 - a13 * b07) * inv_det;
		result.m5  = ( a00 * b11 - a02 * b08 + a03 * b07) * inv_det;
		result.m6  = (-a30 * b05 + a32 * b02 - a33 * b01) * inv_det;
		result.m7  = ( a20 * b05 - a22 * b02 + a23 * b01) * inv_det;
		result.m8  = ( a10 * b10 - a11 * b08 + a13 * b06) * inv_det;
		result.m9  = (-a00 * b10 + a01 * b08 - a03 * b06) * inv_det;
		result.m10 = ( a30 * b04 - a31 * b02 + a33 * b00) * inv_det;
		result.m11 = (-a20 * b04 + a21 * b02 - a23 * b00) * inv_det;
		result.m12 = (-a10 * b09 + a11 * b07 - a12 * b06) * inv_det;
		result.m13 = ( a00 * b09 - a01 * b07 + a02 * b06) * inv_det;
		result.m14 = (-a30 * b03 + a31 * b01 - a32 * b00) * inv_det;
		result.m15 = ( a20 * b03 - a21 * b01 + a22 * b00) * inv_det;

		return result;
	}
	/// Get translation matrix
	pub fn translate(v: Vector3) -> Self {
		Self {
			m0:  1.0, m4:  0.0, m8:  0.0, m12: v.x,
			m1:  0.0, m5:  1.0, m9:  0.0, m13: v.y,
			m2:  0.0, m6:  0.0, m10: 1.0, m14: v.z,
			m3:  0.0, m7:  0.0, m11: 0.0, m15: 1.0,
		}
	}
	/// Create rotation matrix from axis and angle
	/// 
	/// NOTE: Angle should be provided in radians
	pub fn rotate(axis: Vector3, angle: f32) -> Self {
		let mut result = ZERO;

		let mut x = axis.x;
		let mut y = axis.y;
		let mut z = axis.z;

		let length_squared = x*x + y*y + z*z;
		if length_squared != 1.0 && (length_squared != 0.0) {
		    let ilength = 1.0 / length_squared.sqrt();
		    x *= ilength;
		    y *= ilength;
		    z *= ilength;
		}

		let sinres = angle.sin();
		let cosres = angle.cos();
		let t = 1.0 - cosres;

		result.m0 = x*x*t + cosres;
		result.m1 = y*x*t + z*sinres;
		result.m2 = z*x*t - y*sinres;
		result.m3 = 0.0;

		result.m4 = x*y*t - z*sinres;
		result.m5 = y*y*t + cosres;
		result.m6 = z*y*t + x*sinres;
		result.m7 = 0.0;

		result.m8 = x*z*t + y*sinres;
		result.m9 = y*z*t - x*sinres;
		result.m10 = z*z*t + cosres;
		result.m11 = 0.0;

		result.m12 = 0.0;
		result.m13 = 0.0;
		result.m14 = 0.0;
		result.m15 = 1.0;

		return result;
	}
	/// Get x-rotation matrix
	/// 
	/// NOTE: Angle should be provided in radians
	pub fn rotate_x(angle: f32) -> Self {
		let mut result = IDENTITY;

		let cosres = angle.cos();
		let sinres = angle.sin();

		result.m5  =  cosres;
		result.m6  =  sinres;
		result.m9  = -sinres;
		result.m10 =  cosres;

		result
	}
	/// Get y-rotation matrix
	/// 
	/// NOTE: Angle should be provided in radians
	pub fn rotate_y(angle: f32) -> Self {
		let mut result = IDENTITY;

		let cosres = angle.cos();
		let sinres = angle.sin();

		result.m0  =  cosres;
		result.m2  = -sinres;
		result.m8  =  sinres;
		result.m10 =  cosres;

		result
	}
	/// Get z-rotation matrix
	/// 
	/// NOTE: Angle should be provided in radians
	pub fn rotate_z(angle: f32) -> Self {
		let mut result = IDENTITY;

		let cosres = angle.cos();
		let sinres = angle.sin();

		result.m0 =  cosres;
		result.m1 =  sinres;
		result.m4 = -sinres;
		result.m5 =  cosres;

		result
	}
	/// Get xyz-rotation matrix
	/// 
	/// NOTE: Angle should be provided in radians
	pub fn rotate_xyz(angle: Vector3) -> Self {
		let mut result = IDENTITY;

		let cosz = -angle.z.cos();
		let sinz = -angle.z.sin();
		let cosy = -angle.y.cos();
		let siny = -angle.y.sin();
		let cosx = -angle.x.cos();
		let sinx = -angle.x.sin();

		result.m0 =  cosz * cosy;
		result.m1 = (cosz * siny * sinx) - (sinz * cosx);
		result.m2 = (cosz * siny * cosx) + (sinz * sinx);

		result.m4 =  sinz * cosy;
		result.m5 = (sinz * siny * sinx) + (cosz * cosx);
		result.m6 = (sinz * siny * cosx) - (cosz * sinx);

		result.m8 = -siny;
		result.m9 =  cosy * sinx;
		result.m10=  cosy * cosx;

		result
	}
	/// Get zyx-rotation matrix
	/// 
	/// NOTE: Angle should be provided in radians
	pub fn rotate_zyx(angle: Vector3) -> Self {
		let mut result = ZERO;

		let cz = angle.z.cos();
		let sz = angle.z.sin();
		let cy = angle.y.cos();
		let sy = angle.y.sin();
		let cx = angle.x.cos();
		let sx = angle.x.sin();

		result.m0  = cz * cy;
		result.m4  = cz * sy * sx - cx * sz;
		result.m8  = sz * sx + cz * cx * sy;
		result.m12 = 0.0;

		result.m1  = cy * sz;
		result.m5  = cz * cx + sz * sy * sx;
		result.m9  = cx * sz * sy - cz * sx;
		result.m13 = 0.0;

		result.m2  = -sy;
		result.m6  =  cy * sx;
		result.m10 =  cy * cx;
		result.m14 =  0.0;

		result.m3  = 0.0;
		result.m7  = 0.0;
		result.m11 = 0.0;
		result.m15 = 1.0;

		result
	}
	/// Get scaling matrix
	pub fn scale(scale: Vector3) -> Self {
		Self {
			m0: scale.x, m4:     0.0, m8:      0.0, m12: 0.0,
			m1:     0.0, m5: scale.y, m9:      0.0, m13: 0.0,
			m2:     0.0, m6:     0.0, m10: scale.z, m14: 0.0,
			m3:     0.0, m7:     0.0, m11:     0.0, m15: 1.0,
		}
	}
	/// Get perspective projection matrix
	pub fn frustum(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> Self {
		let mut result = ZERO;

		let rl = (right - left) as f32;
		let tb = (top - bottom) as f32;
		let f_n = (far - near) as f32;

		result.m0 = (near as f32 * 2.0) / rl;
		result.m1 = 0.0;
		result.m2 = 0.0;
		result.m3 = 0.0;

		result.m4 = 0.0;
		result.m5 = (near as f32 * 2.0) / tb;
		result.m6 = 0.0;
		result.m7 = 0.0;

		result.m8  =  (right + left) as f32 / rl;
		result.m9  =  (top + bottom) as f32 / tb;
		result.m10 = -(far + near) as f32 / f_n;
		result.m11 = -1.0;

		result.m12 = 0.0;
		result.m13 = 0.0;
		result.m14 = -(far * near * 2.0) as f32 / f_n;
		result.m15 = 0.0;

		result
	}
	/// Get perspective projection matrix
	/// 
	/// NOTE: Fovy angle must be provided in radians
	pub fn perspective(fov_y: f64, aspect: f64, near_plane: f64, far_plane: f64) -> Self {
		let mut result = ZERO;

		let top    =  near_plane * (fov_y*0.5).tan();
		let bottom = -top;
		let right  =  top * aspect;
		let left   = -right;

		//* MatrixFrustum(-right, right, -top, top, near, far);
		let rl = (right - left) as f32;
		let tb = (top - bottom) as f32;
		let f_n = (far_plane - near_plane) as f32;

		result.m0  =  (near_plane as f32 * 2.0) / rl;
		result.m5  =  (near_plane as f32 * 2.0) / tb;
		result.m8  =  (right + left) as f32 / rl;
		result.m9  =  (top + bottom) as f32 / tb;
		result.m10 = -(far_plane + near_plane) as f32 / f_n;
		result.m11 = -1.0;
		result.m14 = -(far_plane * near_plane * 2.0) as f32 / f_n;

		result
	}
	/// Get orthographic projection matrix
	pub fn ortho(left: f64, right: f64, bottom: f64, top: f64, near_plane: f64, far_plane: f64) -> Self {
		let mut result = ZERO;

		let rl = (right - left) as f32;
		let tb = (top - bottom) as f32;
		let f_n = (far_plane - near_plane) as f32;

		result.m0  =  2.0 / rl;
		result.m1  =  0.0;
		result.m2  =  0.0;
		result.m3  =  0.0;
		result.m4  =  0.0;
		result.m5  =  2.0 / tb;
		result.m6  =  0.0;
		result.m7  =  0.0;
		result.m8  =  0.0;
		result.m9  =  0.0;
		result.m10 = -2.0 / f_n;
		result.m11 =  0.0;
		result.m12 = -(left + right) as f32 / rl;
		result.m13 = -(top + bottom) as f32 / tb;
		result.m14 = -(far_plane + near_plane) as f32 / f_n;
		result.m15 =  1.0;

		result
	}
	/// Get camera look-at matrix (view matrix)
	pub fn look_at(eye: Vector3, target: Vector3, up: Vector3) -> Self {
		let mut result = ZERO;

		let mut length;
		let mut ilength;

		let mut vz = eye - target;

		// Vector3Normalize(vz)
		let mut v = vz;
		length = v.length();
		if length == 0.0 { length = 1.0 }
		ilength = 1.0 / length;
		vz.x *= ilength;
		vz.y *= ilength;
		vz.z *= ilength;

		// Vector3CrossProduct(up, vz)
		let mut vx = up.cross_product(vz);

		// Vector3Normalize(x)
		v = vx;
		length = v.length();
		if length == 0.0 { length = 1.0 }
		ilength = 1.0 / length;
		vx.x *= ilength;
		vx.y *= ilength;
		vx.z *= ilength;

		// Vector3CrossProduct(vz, vx)
		let vy = vz.cross_product(vx);

		result.m0  =  vx.x;
		result.m1  =  vy.x;
		result.m2  =  vz.x;
		result.m3  =   0.0;
		result.m4  =  vx.y;
		result.m5  =  vy.y;
		result.m6  =  vz.y;
		result.m7  =   0.0;
		result.m8  =  vx.z;
		result.m9  =  vy.z;
		result.m10 =  vz.z;
		result.m11 =   0.0;
		result.m12 = -(vx.x * eye.x + vx.y * eye.y + vx.z * eye.z);   // Vector3DotProduct(vx, eye)
		result.m13 = -(vy.x * eye.x + vy.y * eye.y + vy.z * eye.z);   // Vector3DotProduct(vy, eye)
		result.m14 = -(vz.x * eye.x + vz.y * eye.y + vz.z * eye.z);   // Vector3DotProduct(vz, eye)
		result.m15 =  1.0;

		result
	}
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