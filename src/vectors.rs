

//= Imports
use std::{f32::EPSILON, ops::{Add, Div, Mul, Not, Sub}};

use super::matrix::Matrix;


//= Structures and Enumerations

/// Vector2
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
impl Into<raylib_ffi::Vector2> for Vector2 {
	fn into(self) -> raylib_ffi::Vector2 {
		raylib_ffi::Vector2 {
			x: self.x,
			y: self.y,
		}
	}
}
impl From<raylib_ffi::Vector2> for Vector2 {
	fn from(value: raylib_ffi::Vector2) -> Self {
		Self {
			x: value.x,
			y: value.y,
		}
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
impl Into<raylib_ffi::Vector3> for Vector3 {
	fn into(self) -> raylib_ffi::Vector3 {
		raylib_ffi::Vector3 {
			x: self.x,
			y: self.y,
			z: self.z,
		}
	}
}
impl From<raylib_ffi::Vector3> for Vector3 {
	fn from(value: raylib_ffi::Vector3) -> Self {
		Self {
			x: value.x,
			y: value.y,
			z: value.z,
		}
	}
}

/// Vector4
pub struct Vector4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}
impl Into<raylib_ffi::Vector4> for Vector4 {
	fn into(self) -> raylib_ffi::Vector4 {
		raylib_ffi::Vector4 {
			x: self.x,
			y: self.y,
			z: self.z,
			w: self.w,
		}
	}
}
impl From<raylib_ffi::Vector4> for Vector4 {
	fn from(value: raylib_ffi::Vector4) -> Self {
		Self {
			x: value.x,
			y: value.y,
			z: value.z,
			w: value.w,
		}
	}
}


//= Constants

pub const ZERO_2: Vector2 = Vector2{x: 0.0, y: 0.0};
pub const ZERO_3: Vector3 = Vector3{x: 0.0, y: 0.0, z: 0.0};
pub const ZERO_4: Vector4 = Vector4{x: 0.0, y: 0.0, z: 0.0, w: 0.0};

pub const ONE_2: Vector2 = Vector2{x: 1.0, y: 1.0};
pub const ONE_3: Vector3 = Vector3{x: 1.0, y: 1.0, z: 1.0};
pub const ONE_4: Vector4 = Vector4{x: 1.0, y: 1.0, z: 1.0, w: 1.0};


//= Implementations

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