use std::ops::Div;



pub struct Resolution {
	pub width: i32,
	pub height: i32,
}
impl Div<i32> for Resolution {
	type Output = Self;

	fn div(self, rhs: i32) -> Self::Output {
		Self {
			width:  self.width  / rhs,
			height: self.height / rhs,
		}
	}
}