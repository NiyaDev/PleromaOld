

use std::{
	f32::EPSILON,
	ops::{
		Add, Sub,
		Mul, Div,
		Not,
	}
};

use super::matrix::Matrix;

/// Vector2
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}
impl Add<Vector2> for Vector2 {
	type Output = Vector2;

	fn add(self, rhs: Vector2) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}
impl Add<f32> for Vector2 {
	type Output = Vector2;

	fn add(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x + rhs,
			y: self.y + rhs,
		}
	}
}
impl Sub<Vector2> for Vector2 {
	type Output = Vector2;

	fn sub(self, rhs: Vector2) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}
impl Sub<f32> for Vector2 {
	type Output = Vector2;

	fn sub(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x - rhs,
			y: self.y - rhs,
		}
	}
}
impl Mul<Vector2> for Vector2 {
	type Output = Vector2;

	fn mul(self, rhs: Vector2) -> Self::Output {
		Self {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
		}
	}
}
impl Mul<f32> for Vector2 {
	type Output = Vector2;

	fn mul(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}
impl Div<Vector2> for Vector2 {
	type Output = Vector2;

	fn div(self, rhs: Vector2) -> Self::Output {
		Self {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
		}
	}
}
impl Div<f32> for Vector2 {
	type Output = Vector2;

	fn div(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
		}
	}
}
impl PartialEq for Vector2 {
	fn eq(&self, other: &Self) -> bool {
		(self.x - other.x).abs() <= (EPSILON * 1.0_f32.max(self.x.abs().max(other.x.abs()))) && (self.y - other.y).abs() <= (EPSILON * 1.0_f32.max(self.y.abs().max(other.y.abs())))
	}
}
impl Not for Vector2 {
	type Output = Self;

	fn not(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
		}
	}
}

/// Vector3
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}
impl Add<Vector3> for Vector3 {
	type Output = Vector3;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}
impl Add<f32> for Vector3 {
	type Output = Self;

	fn add(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x + rhs,
			y: self.y + rhs,
			z: self.z + rhs,
		}
	}
}
impl Sub<Vector3> for Vector3 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
		}
	}
}
impl Sub<f32> for Vector3 {
	type Output = Self;

	fn sub(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x - rhs,
			y: self.y - rhs,
			z: self.z - rhs,
		}
	}
}
impl Mul<Vector3> for Vector3 {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
			z: self.z * rhs.z,
		}
	}
}
impl Mul<f32> for Vector3 {
	type Output = Self;

	fn mul(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
		}
	}
}
impl Div<Vector3> for Vector3 {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
			z: self.z / rhs.z,
		}
	}
}
impl Div<f32> for Vector3 {
	type Output = Self;

	fn div(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
			z: self.z / rhs,
		}
	}
}
impl PartialEq for Vector3 {
	fn eq(&self, other: &Self) -> bool {
		(self.x - other.x).abs() <= (EPSILON * 1.0_f32.max(self.x.abs().max(other.x.abs())))
			&& (self.y - other.y).abs() <= (EPSILON * 1.0_f32.max(self.y.abs().max(other.y.abs())))
			&& (self.z - other.z).abs() <= (EPSILON * 1.0_f32.max(self.z.abs().max(other.z.abs())))
	}
}
impl Not for Vector3 {
	type Output = Self;

	fn not(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
			z: -self.z,
		}
	}
}

/// Vector4
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}


pub const ZERO_2: Vector2 = Vector2{x: 0.0, y: 0.0};
pub const ZERO_3: Vector3 = Vector3{x: 0.0, y: 0.0, z: 0.0};
pub const ZERO_4: Vector4 = Vector4{x: 0.0, y: 0.0, z: 0.0, w: 0.0};

pub const ONE_2: Vector2 = Vector2{x: 1.0, y: 1.0};
pub const ONE_3: Vector3 = Vector3{x: 1.0, y: 1.0, z: 1.0};
pub const ONE_4: Vector4 = Vector4{x: 1.0, y: 1.0, z: 1.0, w: 1.0};

pub const TEN_2: Vector2 = Vector2{x: 10.0, y: 10.0};
pub const TEN_3: Vector3 = Vector3{x: 10.0, y: 10.0, z: 10.0};
pub const TEN_4: Vector4 = Vector4{x: 10.0, y: 10.0, z: 10.0, w: 10.0};


impl Vector2 {
	
	/// Calculate vector length
	pub fn length(&self) -> f32 {
		((self.x * self.x) + (self.y * self.y)).sqrt()
	}
	/// Calculate vector square length
	pub fn length_sqr(&self) -> f32 {
		(self.x * self.x) + (self.y * self.y)
	}
	/// Calculate two vectors dot product
	pub fn dot_product(&self, v2: Self) -> f32 {
		(self.x * v2.x) + (self.y * v2.y)
	}
	/// Calculate distance between two vectors
	pub fn distance(&self, v2: Self) -> f32 {
		((self.x - v2.x).powi(2) + (self.y - v2.y).powi(2)).sqrt()
	}
	/// Calculate square distance between two vectors
	pub fn distance_sqr(&self, v2: Self) -> f32 {
		(self.x - v2.x).powi(2) + (self.y - v2.y).powi(2)
	}
	/// Calculate angle between two vectors
	// NOTE: Angle is calculated from origin point (0, 0)
	pub fn angle(&self, v2: Self) -> f32 {
		let dot = (self.x * v2.x) + (self.y * v2.y);
		let det = (self.x * v2.x) - (self.y * v2.y);

		dot.atan2(det)
	}
	/// Calculate angle defined by a two vectors line
	// NOTE: Parameters need to be normalized
	// TODO: Maybe normalize them inside?
	pub fn angle_line(&self, end: Self) -> f32 {
		-(end.y - self.y).atan2(end.x - self.x)
	}
	/// Normalize provided vector
	pub fn normalize(&self) -> Self {
		let length = (self.x.powi(2) + self.y.powi(2)).sqrt();

		if length > 0.0 {
			let ilength = 1.0 / length;
			return Self {
				x: self.x * ilength,
				y: self.y * ilength,
			}
		}

		Self {x: self.x, y: self.y}
	}
	/// Transforms a Vector2 by a given Matrix
	pub fn transform(&self, mat: Matrix) -> Self {
		let x = self.x;
		let y = self.y;
		let z = 0.0;

		Self {
			x: (mat.m0 * x) + (mat.m4 * y) + (mat.m8 * z) + mat.m12,
			y: (mat.m1 * x) + (mat.m5 * y) + (mat.m9 * z) + mat.m13,
		}
	}
	/// Calculate linear interpolation between two vectors
	pub fn lerp(&self, v2: Self, amount: f32) -> Vector2 {
		Self {
			x: self.x + (amount * (v2.x - self.x)),
			y: self.y + (amount * (v2.y - self.y)),
		}
	}
	/// Calculate reflected vector to normal
	pub fn reflect(&self, normal: Self) -> Self {
		let dot_product = self.dot_product(normal.clone());

		Self {
			x: self.x - (2.0 * normal.x) * dot_product,
			y: self.y - (2.0 * normal.y) * dot_product,
		}
	}
	/// Rotate vector by angle
	pub fn rotate(&self, angle: f32) -> Self {
		let cosres = angle.cos();
		let sinres = angle.sin();

		Self {
			x: (self.x * cosres) - (self.y * sinres),
			y: (self.x * sinres) + (self.y * cosres),
		}
	}
	/// Move Vector towards target
	pub fn move_towards(&self, target: Self, max_distance: f32) -> Self {
		let dx = target.x - self.x;
		let dy = target.y - self.y;
		let value = dx.powi(2) + dy.powi(2);

		if value == 0.0 || (max_distance >= 0.0 && value <= max_distance.powi(2)) { return target; }

		let dist = value.sqrt();

		Self {
			x: self.x + (dx / dist) * max_distance,
			y: self.y + (dy / dist) * max_distance,
		}
	}
	/// Invert the given vector
	pub fn invert(&self) -> Self {
		Self {
			x: 1.0 / self.x,
			y: 1.0 / self.y,
		}
	}
	/// Clamp the components of the vector between
	/// 
	/// min and max values specified by the given vectors
	pub fn clamp(&self, min: Self, max: Self) -> Self {
		Self {
			x: self.x.clamp(min.x, max.x),
			y: self.y.clamp(min.y, max.y),
		}
	}
	/// Clamp the magnitude of the vector between two min and max values
	pub fn clamp_mag(&self, min: f32, max: f32) -> Self {
		let mut length = self.x.powi(2) + self.y.powi(2);

		if length > 0.0 {
			length = length.sqrt();

			let mut scale = 1.0;
			if length < min {
				scale = min / length;
			} else if length > max {
				scale = max / length;
			}

			return Self {
				x: self.x * scale,
				y: self.y * scale,
			}
		}

		Self {
			x: self.x,
			y: self.y,
		}
	}

}

impl Vector3 {

	/// Calculate two vectors cross product
	pub fn cross_product(&self, v2: Self) -> Self {
		Self {
			x: self.y * v2.z - self.z * v2.y,
			y: self.z * v2.x - self.x * v2.z,
			z: self.x * v2.y - self.y * v2.x,
		}
	}
	/// Calculate one vector perpendicular vector
	pub fn perpendicular(&self) -> Self {
		let mut min = self.x.abs();
		let mut cardinal_axis = Vector3 { x: 1.0, y: 0.0, z: 0.0 };

		if self.y.abs() < min {
			min = self.y.abs();
			cardinal_axis = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
		}
		if self.z.abs() < min {
			cardinal_axis = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
		}

		self.cross_product(cardinal_axis)
	}
	/// Calculate vector length
	pub fn length(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
	}
	/// Calculate vector square length
	pub fn length_sqr(&self) -> f32 {
		self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
	}
	/// Calculate two vectors dot product
	pub fn dot_product(&self, v2: Self) -> f32 {
		self.x * v2.x + self.y * v2.y + self.z * v2.z
	}
	/// Calculate distance between two vectors
	pub fn distance(&self, v2: Self) -> f32 {
		let result = Self {
			x: v2.x - self.x,
			y: v2.y - self.y,
			z: v2.z - self.z,
		};

		result.length()
	}
	/// Calculate square distance between two vectors
	pub fn distance_sqr(&self, v2: Self) -> f32 {
		let result = Self {
			x: v2.x - self.x,
			y: v2.y - self.y,
			z: v2.z - self.z,
		};

		result.length_sqr()
	}
	/// Calculate angle between two vectors
	pub fn angle(&self, v2: Self) -> f32 {
		let cross = self.cross_product(v2);
		let len = cross.length();
		let dot = self.dot_product(v2);

		len.atan2(dot)
	}
	/// Normalize provided vector
	pub fn normalize(&self) -> Self {
		let len = self.length();
		if len != 0.0 {
			let ilen = 1.0 / len;

			Self {
				x: self.x * ilen,
				y: self.y * ilen,
				z: self.z * ilen,
			}
		} else { *self }
	}
	///Calculate the projection of the vector v1 on to v2
	pub fn project(&self, v2: Self) -> Self {
		let v1dv2 = self.x*v2.x + self.y*v2.y + self.z*v2.z;
		let v2dv2 = v2.length_sqr();

		let mag = v1dv2 / v2dv2;

		v2 * mag
	}
	///Calculate the rejection of the vector v1 on to v2
	pub fn reject(&self, v2: Self) -> Self {
		let v1dv2 = self.x*v2.x + self.y*v2.y + self.z*v2.z;
		let v2dv2 = v2.length_sqr();

		let mag = v1dv2 / v2dv2;

		*self - (v2 * mag)
	}
	/// Orthonormalize provided vectors
	/// 
	/// Makes vectors normalized and orthogonal to each other
	/// 
	/// Gram-Schmidt function implementation
	pub fn orthonormalize(&mut self, v2: &mut Self) {
		let mut len: f32;
		let mut ilen: f32;

		//* Normalize self */
		len = self.length();
		len = if len == 0.0 { 1.0 } else { len };
		ilen = 1.0 / len;
		self.x *= ilen;
		self.y *= ilen;
		self.z *= ilen;

		//* Cross product */
		let mut vn1 = self.cross_product(*v2);

		//* Normalize */
		len = vn1.length();
		len = if len == 0.0 { 1.0 } else { len };
		ilen = 1.0 / len;
		vn1.x *= ilen;
		vn1.y *= ilen;
		vn1.z *= ilen;

		//* Cross product */
		let vn2 = vn1.cross_product(*self);

		v2.x = vn2.x;
		v2.y = vn2.y;
		v2.z = vn2.z;
	}
	/// Transforms a Vector3 by a given Matrix
	pub fn transform(&self, mat: Matrix) -> Self {
		Self {
			x: mat.m0 * self.x + mat.m4 * self.y + mat.m8  * self.z + mat.m12,
			y: mat.m1 * self.x + mat.m5 * self.y + mat.m9  * self.z + mat.m13,
			z: mat.m2 * self.x + mat.m6 * self.y + mat.m10 * self.z + mat.m14,
		}
	}

}