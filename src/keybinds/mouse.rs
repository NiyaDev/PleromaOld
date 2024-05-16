

/// ### Mouse Keys
/// Stolen from raylib.
#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
	/// Mouse button left
	Left	= 0,
	/// Mouse button right
	Right	= 1,
	/// Mouse button middle (pressed wheel)
	Middle	= 2,
	/// Mouse button side (advanced mouse device)
	Side	= 3,
	/// Mouse button extra (advanced mouse device)
	Extra	= 4,
	/// Mouse button forward (advanced mouse device)
	Forward	= 5,
	/// Mouse button back (advanced mouse device)
	Back	= 6,
	/// Positive X axis
	Xp		= 7,
	/// Negative X axis
	Xn		= 8,
	/// Positive Y axis
	Yp		= 9,
	/// Negative Y axis
	Yn		= 10,
	/// Positive mouse wheel
	Wheelp	= 11,
	/// Negative mouse wheel
	Wheeln	= 12,
}
impl Into<i32> for MouseButton {
	fn into(self) -> i32 {
		match self {
			MouseButton::Left		=> {  0 }
			MouseButton::Right		=> {  1 }
			MouseButton::Middle		=> {  2 }
			MouseButton::Side		=> {  3 }
			MouseButton::Extra		=> {  4 }
			MouseButton::Forward	=> {  5 }
			MouseButton::Back		=> {  6 }
			MouseButton::Xp			=> {  7 }
			MouseButton::Xn			=> {  8 }
			MouseButton::Yp			=> {  9 }
			MouseButton::Yn			=> { 10 }
			MouseButton::Wheelp		=> { 11 }
			MouseButton::Wheeln		=> { 12 }
		}
	}
}