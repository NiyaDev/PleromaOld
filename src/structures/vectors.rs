

use std::{
	f32::EPSILON,
	ops::{
		Add, Sub,
		Mul, Div,
		Not,
	}
};

use super::matrix::{self, Matrix};

/// Vector2
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}
impl Add<Self> for Vector2 {
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
impl Sub<Self> for Vector2 {
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
impl Mul<Self> for Vector2 {
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
impl Div<Self> for Vector2 {
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
impl Add<Self> for Vector3 {
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
impl Sub<Self> for Vector3 {
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
impl Mul<Self> for Vector3 {
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
impl Div<Self> for Vector3 {
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
#[derive(Debug, Clone, Copy)]
pub struct Vector4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}
impl Add<Self> for Vector4 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
			w: self.w + rhs.w,
		}
	}
}
impl Add<f32> for Vector4 {
	type Output = Self;

	fn add(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x + rhs,
			y: self.y + rhs,
			z: self.z + rhs,
			w: self.w + rhs,
		}
	}
}
impl Sub<Self> for Vector4 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
			w: self.w - rhs.w,
		}
	}
}
impl Sub<f32> for Vector4 {
	type Output = Self;

	fn sub(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x - rhs,
			y: self.y - rhs,
			z: self.z - rhs,
			w: self.w - rhs,
		}
	}
}
impl Mul<Self> for Vector4 {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
			z: self.z * rhs.z,
			w: self.w * rhs.w,
		}
	}
}
impl Mul<f32> for Vector4 {
	type Output = Self;

	fn mul(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
			w: self.w * rhs,
		}
	}
}
impl Div<Self> for Vector4 {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
			z: self.z / rhs.z,
			w: self.w / rhs.w,
		}
	}
}
impl Div<f32> for Vector4 {
	type Output = Self;

	fn div(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
			z: self.z / rhs,
			w: self.w / rhs,
		}
	}
}
impl PartialEq for Vector4 {
	fn eq(&self, other: &Self) -> bool {
		(self.x - other.x).abs() <= (EPSILON * 1.0_f32.max(self.x.abs().max(other.x.abs())))
			&& (self.y - other.y).abs() <= (EPSILON * 1.0_f32.max(self.y.abs().max(other.y.abs())))
			&& (self.z - other.z).abs() <= (EPSILON * 1.0_f32.max(self.z.abs().max(other.z.abs())))
			&& (self.w - other.w).abs() <= (EPSILON * 1.0_f32.max(self.w.abs().max(other.w.abs())))
	}
}
impl Not for Vector4 {
	type Output = Self;

	fn not(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
			z: -self.z,
			w: -self.w,
		}
	}
}

/// Quaternion
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}
impl Add<Self> for Quaternion {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
			w: self.w + rhs.w,
		}
	}
}
impl Add<f32> for Quaternion {
	type Output = Self;

	fn add(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x + rhs,
			y: self.y + rhs,
			z: self.z + rhs,
			w: self.w + rhs,
		}
	}
}
impl Sub<Self> for Quaternion {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
			w: self.w - rhs.w,
		}
	}
}
impl Sub<f32> for Quaternion {
	type Output = Self;

	fn sub(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x - rhs,
			y: self.y - rhs,
			z: self.z - rhs,
			w: self.w - rhs,
		}
	}
}
impl Mul<Self> for Quaternion {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
			z: self.z * rhs.z,
			w: self.w * rhs.w,
		}
	}
}
impl Mul<f32> for Quaternion {
	type Output = Self;

	fn mul(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
			z: self.z * rhs,
			w: self.w * rhs,
		}
	}
}
impl Div<Self> for Quaternion {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
			z: self.z / rhs.z,
			w: self.w / rhs.w,
		}
	}
}
impl Div<f32> for Quaternion {
	type Output = Self;

	fn div(self, rhs: f32) -> Self::Output {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
			z: self.z / rhs,
			w: self.w / rhs,
		}
	}
}
impl PartialEq for Quaternion {
	fn eq(&self, other: &Self) -> bool {
		(self.x - other.x).abs() <= (EPSILON * 1.0_f32.max(self.x.abs().max(other.x.abs())))
			&& (self.y - other.y).abs() <= (EPSILON * 1.0_f32.max(self.y.abs().max(other.y.abs())))
			&& (self.z - other.z).abs() <= (EPSILON * 1.0_f32.max(self.z.abs().max(other.z.abs())))
			&& (self.w - other.w).abs() <= (EPSILON * 1.0_f32.max(self.w.abs().max(other.w.abs())))
	}
}
impl Not for Quaternion {
	type Output = Self;

	fn not(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
			z: -self.z,
			w: -self.w,
		}
	}
}
impl From<Matrix> for Quaternion {
	fn from(mat: Matrix) -> Self {
		let result;

		let four_w_squ_minus1 = mat.m0  + mat.m5 + mat.m10;
		let four_x_squ_minus1 = mat.m0  + mat.m5 + mat.m10;
		let four_y_squ_minus1 = mat.m5  + mat.m0 + mat.m10;
		let four_z_squ_minus1 = mat.m10 + mat.m0 + mat.m5;

		let mut biggest_index = 0;
		let mut four_biggest_squ_minus1 = four_w_squ_minus1;
		if four_x_squ_minus1 > four_biggest_squ_minus1 {
			four_biggest_squ_minus1 = four_x_squ_minus1;
			biggest_index = 1;
		}
		if four_y_squ_minus1 > four_biggest_squ_minus1 {
			four_biggest_squ_minus1 = four_y_squ_minus1;
			biggest_index = 2;
		}
		if four_z_squ_minus1 > four_biggest_squ_minus1 {
			four_biggest_squ_minus1 = four_z_squ_minus1;
			biggest_index = 3;
		}

		let biggest_val = (four_biggest_squ_minus1 + 1.0).sqrt() * 0.5;
		let mult = 0.25 / biggest_val;

		match biggest_index {
			1 => {
				result = Self {
					x: biggest_val,
					y: (mat.m1 - mat.m4) * mult,
					z: (mat.m8 - mat.m2) * mult,
					w: (mat.m6 - mat.m9) * mult,
				}
			}
			2 => {
				result = Self {
					x: (mat.m1 - mat.m4) * mult,
					y: biggest_val,
					z: (mat.m6 - mat.m9) * mult,
					w: (mat.m8 - mat.m2) * mult,
				}
			}
			3 => {
				result = Self {
					x: (mat.m8 - mat.m2) * mult,
					y: (mat.m6 - mat.m9) * mult,
					z: biggest_val,
					w: (mat.m1 - mat.m4) * mult,
				}
			}
			_ => {
				result = Self {
					x: (mat.m6 - mat.m9) * mult,
					y: (mat.m8 - mat.m2) * mult,
					z: (mat.m1 - mat.m4) * mult,
					w: biggest_val,
				}
			}
		}

		result
	}
}
impl Into<Matrix> for Quaternion {
	fn into(self) -> Matrix {
		let mut result = matrix::IDENTITY;

		let a2 = self.x.powi(2);
		let b2 = self.y.powi(2);
		let c2 = self.z.powi(2);
		let ac = self.x * self.z;
		let ab = self.x * self.y;
		let bc = self.y * self.z;
		let ad = self.w * self.x;
		let bd = self.w * self.y;
		let cd = self.w * self.z;

		result.m0 = 1.0 - 2.0 * (b2 + c2);
		result.m1 = 2.0 * (ab + cd);
		result.m2 = 2.0 * (ac + bd);
		
		result.m4 = 2.0 * (ab + cd);
		result.m5 = 1.0 - 2.0 * (a2 + c2);
		result.m6 = 2.0 * (bc + ad);
		
		result.m8  = 2.0 * (ac + bd);
		result.m9  = 2.0 * (bc + ad);
		result.m10 = 1.0 - 2.0 * (a2 + b2);

		result
	}
}


pub const ZERO_2: Vector2 = Vector2{x: 0.0, y: 0.0};
pub const ZERO_3: Vector3 = Vector3{x: 0.0, y: 0.0, z: 0.0};
pub const ZERO_4: Vector4 = Vector4{x: 0.0, y: 0.0, z: 0.0, w: 0.0};
pub const ZERO_Q: Quaternion = Quaternion{x: 0.0, y: 0.0, z: 0.0, w: 0.0};

pub const ONE_2: Vector2 = Vector2{x: 1.0, y: 1.0};
pub const ONE_3: Vector3 = Vector3{x: 1.0, y: 1.0, z: 1.0};
pub const ONE_4: Vector4 = Vector4{x: 1.0, y: 1.0, z: 1.0, w: 1.0};
pub const ONE_Q: Quaternion = Quaternion{x: 1.0, y: 1.0, z: 1.0, w: 1.0};

pub const NEG_ONE_2: Vector2 = Vector2{x: -1.0, y: -1.0};
pub const NEG_ONE_3: Vector3 = Vector3{x: -1.0, y: -1.0, z: -1.0};
pub const NEG_ONE_4: Vector4 = Vector4{x: -1.0, y: -1.0, z: -1.0, w: -1.0};
pub const NEG_ONE_Q: Quaternion = Quaternion{x: -1.0, y: -1.0, z: -1.0, w: -1.0};

pub const TEN_2: Vector2 = Vector2{x: 10.0, y: 10.0};
pub const TEN_3: Vector3 = Vector3{x: 10.0, y: 10.0, z: 10.0};
pub const TEN_4: Vector4 = Vector4{x: 10.0, y: 10.0, z: 10.0, w: 10.0};
pub const TEN_Q: Quaternion = Quaternion{x: 10.0, y: 10.0, z: 10.0, w: 10.0};

pub const IDENTITY: Quaternion = Quaternion{ x: 0.0, y: 0.0, z: 0.0, w: 1.0 };


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
	/// Transform a vector by quaternion rotation
	pub fn rotate_quaternion(&self, q: Quaternion) -> Self {
		Self {
			x: self.x * (q.x.powi(2) + q.w.powi(2) - q.y.powi(2) - q.z.powi(2)) + self.y * (2.0 * q.x * q.y - 2.0 * q.w * q.z) + self.z * (2.0 * q.x * q.z + 2.0 * q.w * q.y),
			y: self.x * (2.0 * q.w * q.z + 2.0 * q.x * q.y) + self.y * (q.w.powi(2) - q.x.powi(2) + q.y.powi(2) - q.z.powi(2)) + self.z * (-2.0 * q.w * q.x + 2.0 * q.y * q.z),
			z: self.x * (-2.0 * q.w * q.y + 2.0 * q.x * q.z) + self.y * (2.0 * q.w * q.x + 2.0 * q.y * q.z) + self.z * (q.w.powi(2) - q.x.powi(2) - q.y.powi(2) + q.z.powi(2)),
		}
	}
	/// Rotates a vector around an axis
	/// 
	/// Using Euler-Rodrigues Formula
	/// 
	/// Ref.: https://en.wikipedia.org/w/index.php?title=Euler%E2%80%93Rodrigues_formula
	pub fn rotate_axis(&self, axis: Self, angle: f32) -> Self {
		//* normalize */
		let mut len = axis.length();
		len = if len == 0.0 { 1.0 } else { len };
		let ilen = 1.0 / len;
		let new_axis = Self{
			x: axis.x * ilen,
			y: axis.y * ilen,
			z: axis.z * ilen,
		};
		let new_angle = angle / 2.0;
		let mut a = new_angle.sin();
		let b = new_axis.x * a;
		let c = new_axis.y * a;
		let d = new_axis.z * a;
		a = angle.cos();
		let w = Self{ x: b, y: c, z: d };

		//* Cross product */
		let mut wv = w.cross_product(*self);
		let wwv = w.cross_product(wv);

		//* Scale */
		a *= 2.0;
		wv = wv * a;
		wv = wv * 2.0;

		*self + wv + wwv

	}
	/// Move Vector towards target
	pub fn move_towards(&self, target: Self, max_distance: f32) -> Self {
		let dx = target.x - self.x;
		let dy = target.y - self.y;
		let dz = target.z - self.z;
		let value = dx.powi(2) + dy.powi(2) + dz.powi(2);

		if value == 0.0 || (max_distance >= 0.0 && value <= max_distance.powi(2)) { return ZERO_3 }

		let dist = value.sqrt();

		Self {
			x: self.x + dx / dist * max_distance,
			y: self.y + dy / dist * max_distance,
			z: self.z + dz / dist * max_distance,
		}
	}
	/// Calculate linear interpolation between two vectors
	pub fn lerp(&self, target: Self, amount: f32) -> Self {
		Self {
			x: self.x + amount * (target.x - self.x),
			y: self.y + amount * (target.y - self.y),
			z: self.z + amount * (target.z - self.z),
		}
	}
	/// Calculate reflected vector to normal
	pub fn reflect(&self, normal: Self) -> Self {
		let dot = self.dot_product(normal);

		Self {
			x: self.x - (2.0 * normal.x) * dot,
			y: self.y - (2.0 * normal.y) * dot,
			z: self.z - (2.0 * normal.z) * dot,
		}
	}
	/// Get min value for each pair of components
	pub fn min(&self, v2: Self) -> Self {
		Self { x: self.x.min(v2.x), y: self.y.min(v2.y), z: self.z.min(v2.z) }
	}
	/// Get max value for each pair of components
	pub fn max(&self, v2: Self) -> Self {
		Self { x: self.x.max(v2.x), y: self.y.max(v2.y), z: self.z.max(v2.z) }
	}
	/// Compute barycenter coordinates (u, v, w) for point p with respect to triangle (a, b, c)
	/// 
	/// NOTE: Assumes P is on the plane of the triangle
	pub fn barycenter(&self, a: Self, b: Self, c: Self) -> Self {
		let v0 = b - a;
		let v1 = c - a;
		let v2 = *self - a;
		let d00 = v0.dot_product(v0);
		let d01 = v0.dot_product(v1);
		let d11 = v1.dot_product(v1);
		let d20 = v2.dot_product(v0);
		let d21 = v2.dot_product(v1);

		let denom = d00 * d11 - d01.powi(2);

		let z = (d00 * d21 - d01 * d20) / denom;
		let y = (d11 * d20 - d01 * d21) / denom;

		Self { x: 1.0 - (z + y), y, z }
	}
	/// Projects a Vector3 from screen space into object space
	/// 
	/// NOTE: We are avoiding calling other raymath functions despite available
	pub fn unproject(&self, projection: Matrix, view: Matrix) -> Self {

		//* Calculate unprojected matrix (multiply view matrix by projection matrix) and invert it */
		let mat_view_proj = Matrix {
			m0:  view.m0 * projection.m0 + view.m1 * projection.m4 + view.m2 * projection.m8 + view.m3 * projection.m12,
			m1:  view.m0*projection.m1 + view.m1*projection.m5 + view.m2*projection.m9 + view.m3*projection.m13,
			m2:  view.m0*projection.m2 + view.m1*projection.m6 + view.m2*projection.m10 + view.m3*projection.m14,
			m3:  view.m0*projection.m3 + view.m1*projection.m7 + view.m2*projection.m11 + view.m3*projection.m15,
			m4:  view.m4*projection.m0 + view.m5*projection.m4 + view.m6*projection.m8 + view.m7*projection.m12,
			m5:  view.m4*projection.m1 + view.m5*projection.m5 + view.m6*projection.m9 + view.m7*projection.m13,
			m6:  view.m4*projection.m2 + view.m5*projection.m6 + view.m6*projection.m10 + view.m7*projection.m14,
			m7:  view.m4*projection.m3 + view.m5*projection.m7 + view.m6*projection.m11 + view.m7*projection.m15,
			m8:  view.m8*projection.m0 + view.m9*projection.m4 + view.m10*projection.m8 + view.m11*projection.m12,
			m9:  view.m8*projection.m1 + view.m9*projection.m5 + view.m10*projection.m9 + view.m11*projection.m13,
			m10: view.m8*projection.m2 + view.m9*projection.m6 + view.m10*projection.m10 + view.m11*projection.m14,
			m11: view.m8*projection.m3 + view.m9*projection.m7 + view.m10*projection.m11 + view.m11*projection.m15,
			m12: view.m12*projection.m0 + view.m13*projection.m4 + view.m14*projection.m8 + view.m15*projection.m12,
			m13: view.m12*projection.m1 + view.m13*projection.m5 + view.m14*projection.m9 + view.m15*projection.m13,
			m14: view.m12*projection.m2 + view.m13*projection.m6 + view.m14*projection.m10 + view.m15*projection.m14,
			m15: view.m12*projection.m3 + view.m13*projection.m7 + view.m14*projection.m11 + view.m15*projection.m1,
		};

		//* Calculate inverted matrix -> MatrixInvert(matViewProj); */
		let b00 = mat_view_proj.m0  * mat_view_proj.m5  - mat_view_proj.m1  * mat_view_proj.m4;
		let b01 = mat_view_proj.m0  * mat_view_proj.m6  - mat_view_proj.m2  * mat_view_proj.m4;
		let b02 = mat_view_proj.m0  * mat_view_proj.m7  - mat_view_proj.m3  * mat_view_proj.m4;
		let b03 = mat_view_proj.m1  * mat_view_proj.m6  - mat_view_proj.m2  * mat_view_proj.m5;
		let b04 = mat_view_proj.m1  * mat_view_proj.m7  - mat_view_proj.m3  * mat_view_proj.m5;
		let b05 = mat_view_proj.m2  * mat_view_proj.m7  - mat_view_proj.m3  * mat_view_proj.m6;
		let b06 = mat_view_proj.m8  * mat_view_proj.m13 - mat_view_proj.m9  * mat_view_proj.m12;
		let b07 = mat_view_proj.m8  * mat_view_proj.m14 - mat_view_proj.m10 * mat_view_proj.m12;
		let b08 = mat_view_proj.m8  * mat_view_proj.m15 - mat_view_proj.m11 * mat_view_proj.m12;
		let b09 = mat_view_proj.m9  * mat_view_proj.m14 - mat_view_proj.m10 * mat_view_proj.m13;
		let b10 = mat_view_proj.m9  * mat_view_proj.m15 - mat_view_proj.m11 * mat_view_proj.m13;
		let b11 = mat_view_proj.m10 * mat_view_proj.m15 - mat_view_proj.m11 * mat_view_proj.m14;

		//* Calculate the invert determinant (inlined to avoid double-caching) */
		let inv_det = 1.0 / ((b00 * b11) - (b01 * b10) + (b02 * b09) + (b03 * b08) - (b04 * b07) + (b05 * b06));

		let mat_view_proj_inv = Matrix{
			m0:  (( mat_view_proj.m5  * b11) - (mat_view_proj.m6  * b10) + (mat_view_proj.m7  * b09)) * inv_det,
			m1:  ((-mat_view_proj.m1  * b11) - (mat_view_proj.m2  * b10) + (mat_view_proj.m3  * b09)) * inv_det,
			m2:  (( mat_view_proj.m13 * b05) - (mat_view_proj.m14 * b04) + (mat_view_proj.m15 * b03)) * inv_det,
			m3:  ((-mat_view_proj.m9  * b05) + (mat_view_proj.m10 * b04) - (mat_view_proj.m11 * b03)) * inv_det,
			m4:  ((-mat_view_proj.m4  * b11) + (mat_view_proj.m6  * b08) - (mat_view_proj.m7  * b07)) * inv_det,
			m5:  (( mat_view_proj.m0  * b11) - (mat_view_proj.m2  * b08) + (mat_view_proj.m3  * b07)) * inv_det,
			m6:  ((-mat_view_proj.m12 * b05) + (mat_view_proj.m14 * b02) - (mat_view_proj.m15 * b01)) * inv_det,
			m7:  (( mat_view_proj.m8  * b05) - (mat_view_proj.m10 * b02) + (mat_view_proj.m11 * b01)) * inv_det,
			m8:  (( mat_view_proj.m4  * b10) - (mat_view_proj.m5  * b08) + (mat_view_proj.m7  * b06)) * inv_det,
			m9:  ((-mat_view_proj.m0  * b10) + (mat_view_proj.m1  * b08) - (mat_view_proj.m3  * b06)) * inv_det,
			m10: (( mat_view_proj.m12 * b04) - (mat_view_proj.m13 * b02) + (mat_view_proj.m15 * b00)) * inv_det,
			m11: ((-mat_view_proj.m8  * b04) + (mat_view_proj.m9  * b02) - (mat_view_proj.m11 * b00)) * inv_det,
			m12: ((-mat_view_proj.m4  * b09) + (mat_view_proj.m5  * b07) - (mat_view_proj.m6  * b06)) * inv_det,
			m13: (( mat_view_proj.m0  * b09) - (mat_view_proj.m1  * b07) + (mat_view_proj.m2  * b06)) * inv_det,
			m14: ((-mat_view_proj.m12 * b03) + (mat_view_proj.m13 * b01) - (mat_view_proj.m14 * b00)) * inv_det,
			m15: (( mat_view_proj.m8  * b03) - (mat_view_proj.m9  * b01) + (mat_view_proj.m10 * b00)) * inv_det,
		};

		//* Create quaternion from source point */
		let quat = Quaternion{ x: self.x, y: self.y, z: self.z, w: 1.0 };

		//* Multiply quat point by unprojected matrix */
		let q_tranformed = mat_view_proj_inv.quaternion_transform(quat);

		//* Normalized world points in vectors */
		Self {
			x: q_tranformed.x / q_tranformed.w,
			y: q_tranformed.y / q_tranformed.w,
			z: q_tranformed.z / q_tranformed.w,
		}

	}
	/// Invert the given vector
	pub fn invert(&self) -> Self {
		Self { x: 1.0 / self.x, y: 1.0 / self.y, z: 1.0 / self.z }
	}
	/// Clamp the components of the vector between
	/// 
	/// min and max values specified by the given vectors
	pub fn clamp(&self, min: Self, max: Self) -> Self {
		Self {
			x: max.x.min(min.x.max(self.x)),
			y: max.y.min(min.y.max(self.y)),
			z: max.z.min(min.z.max(self.z)),
		}
	}
	/// Clamp the magnitude of the vector between two values
	pub fn clamp_value(&self, min: f32, max: f32) -> Self {
		let mut result = *self;

		let mut len = self.length_sqr();
		if len > 0.0 {
			len = len.sqrt();

			let scale = if len < min { min / len } else if len > max { max / len } else { 1.0 };

			result = Self {
				x: self.x * scale,
				y: self.y * scale,
				z: self.z * scale,
			}
		}

		Self {
			x: result.x,
			y: result.y,
			z: result.z,
		}
	}
	/// Compute the direction of a refracted ray
	/// 
	/// v: normalized direction of the incoming ray
	/// 
	/// n: normalized normal vector of the interface of two optical media
	/// 
	/// r: ratio of the refractive index of the medium from where the ray comes to the refractive index of the medium on the other side of the surface
	pub fn refract(v: Self, n: Self, r: f32) -> Self {
		let mut result = ZERO_3;

		let dot = v.dot_product(n);
		let mut d = 1.0 - r.powi(2) * (1.0 - dot.powi(2));

		if d >= 0.0 {
			d = d.sqrt();
			
			result = Self {
				x: r * v.x - (r * dot + d) * n.x,
				y: r * v.y - (r * dot + d) * n.y,
				z: r * v.z - (r * dot + d) * n.z,
			}
		}

		result
	}

}

impl Vector4 {
	
	/// Calculate vector length
	pub fn length(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
	}
	/// Calculate vector square length
	pub fn length_square(&self) -> f32 {
		self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)
	}
	/// Calculate two vectors dot product
	pub fn dot_product(&self, v2: Self) -> f32 {
		self.x * v2.x + self.y * v2.y + self.z * v2.z + self.w * v2.w
	}
	/// Calculate distance between two vectors
	pub fn distance(&self, v2: Self) -> f32 {
		((self.x - v2.x).powi(2) + (self.y - v2.y).powi(2) + (self.z - v2.z).powi(2) + (self.w - v2.w).powi(2)).sqrt()
	}
	/// Calculate square distance between two vectors
	pub fn distance_square(&self, v2: Self) -> f32 {
		(self.x - v2.x).powi(2) + (self.y - v2.y).powi(2) + (self.z - v2.z).powi(2) + (self.w - v2.w).powi(2)
	}
	/// Normalize provided vector
	pub fn normalize(&self) -> Self {
		let mut result = ZERO_4;

		let len = self.length();
		if len > 0.0 {
			let ilen = 1.0 / len;
			result = Self {
				x: self.x * ilen,
				y: self.y * ilen,
				z: self.z * ilen,
				w: self.w * ilen,
			}
		}

		result
	}
	/// Get min value for each pair of components
	pub fn min(&self, v2: Self) -> Self {
		Self {
			x: self.x.min(v2.x),
			y: self.y.min(v2.y),
			z: self.z.min(v2.z),
			w: self.w.min(v2.w),
		}
	}
	/// Get max value for each pair of components
	pub fn max(&self, v2: Self) -> Self {
		Self {
			x: self.x.max(v2.x),
			y: self.y.max(v2.y),
			z: self.z.max(v2.z),
			w: self.w.max(v2.w),
		}
	}
	/// Calculate linear interpolation between two vectors
	pub fn lerp(&self, target: Self, amount: f32) -> Self {
		Self {
			x: self.x + amount * (target.x - self.x),
			y: self.y + amount * (target.y - self.y),
			z: self.z + amount * (target.z - self.z),
			w: self.w + amount * (target.w - self.w),
		}
	}
	/// Move Vector towards target
	pub fn move_towards(&self, target: Self, max_distance: f32) -> Self {
		let dx = target.x - self.x;
		let dy = target.y - self.y;
		let dz = target.z - self.z;
		let dw = target.w - self.w;
		let value = dx.powi(2) + dy.powi(2) + dz.powi(2) + dw.powi(2);

		if value == 0.0 || (max_distance >= 0.0 && value <= max_distance.powi(2)) { return ZERO_4 }

		let dist = value.sqrt();

		Self {
			x: self.x + dx / dist * max_distance,
			y: self.y + dy / dist * max_distance,
			z: self.z + dz / dist * max_distance,
			w: self.w + dw / dist * max_distance,
		} 
	}
	/// Invert the given vector
	pub fn invert(&self) -> Self {
		Self {
			x: 1.0 / self.x,
			y: 1.0 / self.y,
			z: 1.0 / self.z,
			w: 1.0 / self.w,
		}
	}

}

impl Quaternion {

	/// Computes the length of a quaternion
	pub fn length(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
	}
	/// Calculate two quaternions dot product
	pub fn dot_product(&self, v2: Self) -> f32 {
		self.x * v2.x + self.y * v2.y + self.z * v2.z + self.w * v2.w
	}
	/// Normalize provided quaternion
	pub fn normalize(&self) -> Self {
		let mut result = ZERO_Q;

		let len = self.length();
		if len > 0.0 {
			let ilen = 1.0 / len;
			result = Self {
				x: self.x * ilen,
				y: self.y * ilen,
				z: self.z * ilen,
				w: self.w * ilen,
			}
		}

		result
	}
	/// Invert provided quaternion
	pub fn invert(&self) -> Self {
		Self {
			x: 1.0 / self.x,
			y: 1.0 / self.y,
			z: 1.0 / self.z,
			w: 1.0 / self.w,
		}
	}
	/// Calculate linear interpolation between two quaternions
	pub fn lerp(&self, target: Self, amount: f32) -> Self {
		Self {
			x: self.x + amount * (target.x - self.x),
			y: self.y + amount * (target.y - self.y),
			z: self.z + amount * (target.z - self.z),
			w: self.w + amount * (target.w - self.w),
		}
	}
	/// Calculate slerp-optimized interpolation between two quaternions
	pub fn n_lerp(&self, target: Self, amount: f32) -> Self {
		let result = Self {
			x: self.x + amount * (target.x - self.x),
			y: self.y + amount * (target.y - self.y),
			z: self.z + amount * (target.z - self.z),
			w: self.w + amount * (target.w - self.w),
		};

		let mut len = result.length();
		len = if len == 0.0 { 1.0 } else { len };
		let ilen = 1.0 / len;

		Self {
			x: result.x * ilen,
			y: result.y * ilen,
			z: result.z * ilen,
			w: result.w * ilen,
		}
	}
	/// Calculates spherical linear interpolation between two quaternions
	pub fn slerp(&self, target: Self, amount: f32) -> Self {
		let result;

		let mut cos_half_theta = self.dot_product(target);
		let mut new_target = target;
		if cos_half_theta < 0.0 {
			new_target.x = -target.x;
			new_target.y = -target.y;
			new_target.z = -target.z;
			new_target.w = -target.w;
			cos_half_theta = -cos_half_theta;
		}

		if cos_half_theta.abs() >= 1.0 { result = *self; }
		else if cos_half_theta > 0.95 { result = self.n_lerp(target, amount); }
		else {
			let half_theta = cos_half_theta.acos();
			let sin_half_theta = (1.0 - cos_half_theta.powi(2)).sqrt();

			if sin_half_theta.abs() < EPSILON {
				result = Self {
					x: (self.x * 0.5 + target.x * 0.5),
					y: (self.y * 0.5 + target.y * 0.5),
					z: (self.z * 0.5 + target.z * 0.5),
					w: (self.w * 0.5 + target.w * 0.5),
				};
			} else {
				let ratio_a = ((1.0 - amount) * half_theta).sin() / sin_half_theta;
				let ratio_b = (amount * half_theta).sin() / sin_half_theta;

				result = Self {
					x: (self.x * ratio_a + target.x * ratio_b),
					y: (self.y * ratio_a + target.y * ratio_b),
					z: (self.z * ratio_a + target.z * ratio_b),
					w: (self.w * ratio_a + target.w * ratio_b),
				};
			}
		}

		result
	}
	/// Calculate quaternion based on the rotation from one vector to another
	pub fn from_v3_to_v3(from: Vector3, to: Vector3) -> Self {
		let result;

		let cos2_theta = from.dot_product(to);
		let cross = from.cross_product(to);

		result = Self {
			x: cross.x,
			y: cross.y,
			z: cross.z,
			w: 1.0 + cos2_theta,
		};

		let mut len = result.length();
		len = if len == 0.0 { 1.0 } else { len };
		let ilen = 1.0 / len;

		Self {
			x: result.x * ilen,
			y: result.y * ilen,
			z: result.z * ilen,
			w: result.w * ilen,
		}
	}
	/// Get rotation quaternion for an angle and axis
	/// 
	/// NOTE: Angle must be provided in radians
	pub fn from_axis_angle(axis: Vector3, angle: f32) -> Self {
		let mut result = IDENTITY;

		let axis_len = axis.length();

		if axis_len != 0.0 {
			let new_angle = angle * 0.5;

			let mut len = axis_len;
			len = if len == 0.0 { 1.0 } else { len };
			let mut ilen = 1.0 / len;

			let new_axis = Vector3{
				x: axis.x * ilen,
				y: axis.y * ilen,
				z: axis.z * ilen,
			};
			
			let sinres = new_angle.sin();
			let cosres = new_angle.cos();

			result.x = new_axis.x * sinres;
			result.y = new_axis.y * sinres;
			result.z = new_axis.z * sinres;
			result.w = cosres;

			len = result.length();
			len = if len == 0.0 { 1.0 } else { len };
			ilen = 1.0 / len;

			result.x = result.x * ilen;
			result.y = result.y * ilen;
			result.z = result.z * ilen;
			result.w = result.w * ilen;
		}

		result
	}
	/// Get the rotation angle and axis for a given quaternion
	pub fn to_axis_angle(&self) -> (Vector3, f32) {
		let mut new_quat = *self;

		if self.w.abs() > 1.0 {
			let mut len = self.length();
			len = if len == 0.0 { 1.0 } else { len };
			let ilen = 1.0 / len;

			new_quat.x = self.x * ilen;
			new_quat.y = self.y * ilen;
			new_quat.z = self.z * ilen;
			new_quat.w = self.w * ilen;
		}

		let mut res_axis = ZERO_3;
		let res_angle = 2.0 * self.w.acos();
		let den = (1.0 - self.w.powi(2)).sqrt();

		if den > EPSILON {
			res_axis.x = self.x / den;
			res_axis.y = self.y / den;
			res_axis.z = self.z / den;
		} else {
			res_axis.x = 1.0;
		}

		(res_axis, res_angle)
	}
	/// Get the quaternion equivalent to Euler angles
	/// 
	/// NOTE: Rotation order is ZYX
	pub fn from_euler(pitch: f32, yaw: f32, roll: f32) -> Self {
		let mut result = ZERO_Q;

		let x0 = (pitch * 0.5).cos();
		let x1 = (pitch * 0.5).sin();
		let y0 = (yaw   * 0.5).cos();
		let y1 = (yaw   * 0.5).sin();
		let z0 = (roll  * 0.5).cos();
		let z1 = (roll  * 0.5).sin();

		result.x = x1 * y0 * z0 - x0 * y1 * z1;
		result.y = x0 * y1 * z0 - x1 * y0 * z1;
		result.z = x0 * y0 * z1 - x1 * y1 * z0;
		result.w = x0 * y0 * z0 - x1 * y1 * z1;

		result
	}
	/// Get the Euler angles equivalent to quaternion (roll, pitch, yaw)
	/// 
	/// NOTE: Angles are returned in a Vector3 struct in radians
	pub fn to_euler(&self) -> (f32, f32, f32) {
		let pitch;
		let yaw;
		let roll;

		//* Roll */
		let x0 = 2.0 * (self.w * self.x + self.y * self.z);
		let x1 = 1.0 - (2.0 * (self.x.powi(2) + self.y.powi(2)));
		roll = x0.atan2(x1);

		//* Pitch */
		let mut y0 = 2.0 * (self.w * self.y - self.z * self.x);
		y0 = if y0 > 1.0 { 1.0 } else { y0 };
		y0 = if y0 < -1.0 { -1.0 } else { y0 };
		pitch = y0.asin();

		//* Yaw */
		let z0 = 2.0 * (self.w * self.z + self.x * self.y);
		let z1 = 1.0 - 2.0 * (self.y.powi(2) + self.z.powi(2));
		yaw = z0.atan2(z1);

		(pitch, yaw, roll)
	}
	/// Transform a quaternion given a transformation matrix
	pub fn transform(&self, mat: Matrix) -> Self {
		Self {
			x: mat.m0 * self.x + mat.m4 * self.y + mat.m8  * self.z + mat.m12 * self.w,
			y: mat.m1 * self.x + mat.m5 * self.y + mat.m9  * self.z + mat.m13 * self.w,
			z: mat.m2 * self.x + mat.m6 * self.y + mat.m10 * self.z + mat.m14 * self.w,
			w: mat.m3 * self.x + mat.m7 * self.y + mat.m11 * self.z + mat.m15 * self.w,
		}
	}

}
