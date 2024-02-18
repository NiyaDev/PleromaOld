

//= Imports
use std::{f32::EPSILON, ops::{Add, Div, Mul, Sub}};

use super::matrix::Matrix;


//= Structure and Enumerations

///
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
		//self.x == other.x && self.y == other.y
		(self.x - other.x).abs() <= (EPSILON * 1.0_f32.max(self.x.abs().max(other.x.abs()))) && (self.y - other.y).abs() <= (EPSILON * 1.0_f32.max(self.y.abs().max(other.y.abs())))
	}
}


///
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


///
pub struct Vector4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}


//= Procedures

impl Vector2 {
	
	//= Creation
	/// Vector with components value 0.0
	pub fn zero() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
		}
	}
	/// Vector with components value 1.0
	pub fn one() -> Self {
		Self {
			x: 1.0,
			y: 1.0,
		}
	}

	//= Calculations
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
	pub fn angle_line(&self, end: Self) -> f32 {
		-(end.y - self.y).atan2(end.x - self.x)
	}
	/// Negate vector
	pub fn negate(&self) -> Self {
		Self {
			x: -self.x,
			y: -self.y,
		}
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
			x: self.x + dx / (dist * max_distance),
			y: self.y + dy / (dist * max_distance),
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
			x: self.x.min(min.x).max(max.x),
			y: self.y.min(min.y).max(max.y),
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

	//= Conversion
	/// Converting to FFI version
	pub fn into_ffi(&self) -> raylib_ffi::Vector2 {
		raylib_ffi::Vector2 {
			x: self.x,
			y: self.y,
		}
	}
	/// Converting from FFI version
	pub fn from_ffi(value: raylib_ffi::Vector2) -> Self {
		Self {
			x: value.x,
			y: value.y,
		}
	}

}

impl Vector3 {
	
	//= Creation
	/// Vector with components value 0.0
	pub fn zero() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
			z: 0.0,
		}
	}
	/// Vector with components value 1.0
	pub fn one() -> Self {
		Self {
			x: 1.0,
			y: 1.0,
			z: 1.0,
		}
	}

	//= Calculations
	/// Calculate two vectors cross product
	pub fn cross_product(&self, other: Self) -> Self {
		Self {
			x: (self.y * other.z) - (self.z * other.y),
			y: (self.z * other.x) - (self.x * other.z),
			z: (self.x * other.y) - (self.y * other.x),
		}
	}
	/// Calculate one vector perpendicular vector
	pub fn perpendicular(&self) -> Self {
		let mut min = self.x.abs();
		let mut cardinal_axis = Vector3{
			x: 1.0,
			y: 0.0,
			z: 0.0,
		};

		if self.y.abs() < min {
			min = self.y.abs();
			cardinal_axis = Vector3{
				x: 0.0,
				y: 1.0,
				z: 0.0,
			};
		}

		if self.z.abs() < min {
			cardinal_axis = Vector3{
				x: 0.0,
				y: 0.0,
				z: 1.0,
			};
		}

		Self {
			x: (self.y * cardinal_axis.z) - (self.z * cardinal_axis.y),
			y: (self.z * cardinal_axis.x) - (self.x * cardinal_axis.z),
			z: (self.x * cardinal_axis.y) - (self.y * cardinal_axis.x),
		}
	}
	/// Calculate vector length
	pub fn length(&self) -> f32 {
		((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
	}
	/// Calculate vector square length
	pub fn length_sqr(&self) -> f32 {
		(self.x * self.x) + (self.y * self.y) + (self.z * self.z)
	}
	/// Calculate two vectors dot product
	pub fn dot_product(&self, v2: Self) -> f32 {
		(self.x * v2.x) + (self.y * v2.y) + (self.z * v2.z)
	}
	/// Calculate distance between two vectors
	pub fn distance(&self, v2: Self) -> f32 {
		((self.x - v2.x).powi(2) + (self.y - v2.y).powi(2) + (self.z - v2.z).powi(2)).sqrt()
	}
	/// Calculate square distance between two vectors
	pub fn distance_sqr(&self, v2: Self) -> f32 {
		(self.x - v2.x).powi(2) + (self.y - v2.y).powi(2) + (self.z - v2.z).powi(2)
	}
	/// Calculate angle between two vectors
	// NOTE: Angle is calculated from origin point (0, 0)
	pub fn angle(&self, v2: Self) -> f32 {
		let cross = self.cross_product(v2);
		let len = cross.length();
		let dot = (self.x * v2.x) + (self.y * v2.y) + (self.z * v2.z);

		len.atan2(dot)
	}
	/// Negate vector
	pub fn negate(&self) -> Self {
		Self {
			x: -self.x,
			y: -self.y,
			z: -self.z,
		}
	}
	/// Normalize provided vector
	pub fn normalize(&self) -> Self {
		let length = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();

		if length != 0.0 {
			let ilength = 1.0 / length;
			return Self {
				x: self.x * ilength,
				y: self.y * ilength,
				z: self.z * ilength,
			}
		}

		Self {x: self.x, y: self.y, z: self.z}
	}
	///Calculate the projection of the vector v1 on to v2
	pub fn project(&self, other: Self) -> Self {
		let v1dv2 = (self.x * other.x) + (self.y * other.y) + (self.z * other.z);
		let v2dv2 = other.x.powi(2) + other.y.powi(2) + other.z.powi(2);

		let mag = v1dv2 / v2dv2;

		Self {
			x: other.x * mag,
			y: other.y * mag,
			z: other.z * mag,
		}
	}
	///Calculate the rejection of the vector v1 on to v2
	pub fn reject(&self, other: Self) -> Self {
		let v1dv2 = (self.x * other.x) + (self.y * other.y) + (self.z * other.z);
		let v2dv2 = other.x.powi(2) + other.y.powi(2) + other.z.powi(2);

		let mag = v1dv2 / v2dv2;

		Self {
			x: self.x - (other.x * mag),
			y: self.y - (other.y * mag),
			z: self.z - (other.z * mag),
		}
	}
	/// Orthonormalize provided vectors
	/// 
	/// Makes vectors normalized and orthogonal to each other
	/// 
	/// Gram-Schmidt function implementation
	// TODO: Do this
	pub fn ortho_normalize(&self, _other: Self) -> Self {
		Vector3::zero()
	}
	/// Transforms a Vector2 by a given Matrix
	pub fn transform(&self, mat: Matrix) -> Self {
		let x = self.x;
		let y = self.y;
		let z = 0.0;

		Self {
			x: (mat.m0 * x) + (mat.m4 * y) + (mat.m8  * z) + mat.m12,
			y: (mat.m1 * x) + (mat.m5 * y) + (mat.m9  * z) + mat.m13,
			z: (mat.m2 * x) + (mat.m6 * y) + (mat.m10 * z) + mat.m14,
		}
	}
	/// Transform a vector by quaternion rotation
	// TODO:
	pub fn rotate_by_quaternion(&self, _q: Vector4) -> Self {
		Vector3::zero()
	}
	/// Rotates a vector around an axis
	// TODO:
	pub fn rotate_by_angle(&self, _axis: Self, _angle: f32) -> Self {
		Vector3::zero()
	}
	/// Calculate linear interpolation between two vectors
	pub fn lerp(&self, v2: Self, amount: f32) -> Self {
		Self {
			x: self.x + (amount * (v2.x - self.x)),
			y: self.y + (amount * (v2.y - self.y)),
			z: self.z + (amount * (v2.z - self.z)),
		}
	}
	/// Calculate reflected vector to normal
	pub fn reflect(&self, normal: Self) -> Self {
		let dot_product = self.dot_product(normal.clone());

		Self {
			x: self.x - (2.0 * normal.x) * dot_product,
			y: self.y - (2.0 * normal.y) * dot_product,
			z: self.z - (2.0 * normal.z) * dot_product,
		}
	}
	/// Get min value for each pair of components
	// TODO
	pub fn min(&self, _other: Self) -> Self {
		Vector3::zero()
	}
	/// Get max value for each pair of components
	// TODO
	pub fn max(&self, _other: Self) -> Self {
		Vector3::zero()
	}
	/// Compute barycenter coordinates (u, v, w) for point p with respect to triangle (a, b, c)
	// NOTE: Assumes P is on the plane of the triangle
	// TODO
	pub fn barycenter(&self, _a: Self, _b: Self, _c: Self) -> Self {
		Vector3::zero()
	}
	/// Projects a Vector3 from screen space into object space
	// NOTE: We are avoiding calling other raymath functions despite available
	// TODO
	pub fn unproject(&self, _other: Self) -> Self {
		Vector3::zero()
	}
	/// Move Vector towards target
	pub fn move_towards(&self, target: Self, max_distance: f32) -> Self {
		let dx = target.x - self.x;
		let dy = target.y - self.y;
		let dz = target.z - self.z;
		let value = dx.powi(2) + dy.powi(2);

		if value == 0.0 || (max_distance >= 0.0 && value <= max_distance.powi(2)) { return target; }

		let dist = value.sqrt();

		Self {
			x: self.x + dx / (dist * max_distance),
			y: self.y + dy / (dist * max_distance),
			z: self.z + dz / (dist * max_distance),
		}
	}
	/// Invert the given vector
	pub fn invert(&self) -> Self {
		Self {
			x: 1.0 / self.x,
			y: 1.0 / self.y,
			z: 1.0 / self.z,
		}
	}
	/// Clamp the components of the vector between
	/// 
	/// min and max values specified by the given vectors
	pub fn clamp(&self, min: Self, max: Self) -> Self {
		Self {
			x: self.x.min(min.x).max(max.x),
			y: self.y.min(min.y).max(max.y),
			z: self.z.min(min.z).max(max.z),
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
				z: self.z * scale,
			}
		}

		Self {
			x: self.x,
			y: self.y,
			z: self.z,
		}
	}
	/// Compute the direction of a refracted ray
	/// 
	/// v: normalized direction of the incoming ray
	/// 
	/// n: normalized normal vector of the interface of two optical media
	/// 
	/// r: ratio of the refractive index of the medium from where the ray comes
	/// 
	///    to the refractive index of the medium on the other side of the surface
	// TODO
	pub fn refract(&self, _other: Self) -> Self {
		Vector3::zero()
	}

	//= Conversion
	/// Converting to FFI version
	pub fn into_ffi(&self) -> raylib_ffi::Vector3 {
		raylib_ffi::Vector3 {
			x: self.x,
			y: self.y,
			z: self.z,
		}
	}
	/// Converting from FFI version
	pub fn from_ffi(value: raylib_ffi::Vector3) -> Self {
		Self {
			x: value.x,
			y: value.y,
			z: value.z,
		}
	}

}

impl Vector4 {
	
	//= Conversion
	/// Converting to FFI version
	pub fn into_ffi(&self) -> raylib_ffi::Vector4 {
		raylib_ffi::Vector4 {
			x: self.x,
			y: self.y,
			z: self.z,
			w: self.w,
		}
	}
	/// Converting from FFI version
	pub fn from_ffi(value: raylib_ffi::Vector4) -> Self {
		Self {
			x: value.x,
			y: value.y,
			z: value.z,
			w: value.w,
		}
	}

}