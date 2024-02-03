

//= Allows


//= Imports


//= Structures & Enumerations


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix {
	pub m0: f32,
	pub m4: f32,
	pub m8: f32,
	pub m12: f32,
	pub m1: f32,
	pub m5: f32,
	pub m9: f32,
	pub m13: f32,
	pub m2: f32,
	pub m6: f32,
	pub m10: f32,
	pub m14: f32,
	pub m3: f32,
	pub m7: f32,
	pub m11: f32,
	pub m15: f32,
}


//= Procedures

impl Matrix {

	//
	pub fn identity() -> Self {
		Self {
			m0:  1.0, m4:  0.0, m8:  0.0, m12: 0.0,
			m1:  0.0, m5:  1.0, m9:  0.0, m13: 0.0,
			m2:  0.0, m6:  0.0, m10: 1.0, m14: 0.0,
			m3:  0.0, m7:  0.0, m11: 0.0, m15: 1.0,
		}
	}
	
	/// Get scaling matrix
	pub fn scale(x: f32, y: f32, z: f32) -> Self {
		Self {
			m0:   x, m4: 0.0,  m8: 0.0, m12: 0.0,
			m1: 0.0, m5:   y,  m9: 0.0, m13: 0.0,
			m2: 0.0, m6: 0.0, m10:   z, m14: 0.0,
			m3: 0.0, m7: 0.0, m11: 0.0, m15: 1.0,
		}
	}

}