

/// Mouse Keys stolen from raylib
#[derive(Debug, Clone, Copy)]
pub enum Button {
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
}

/// Mouse axis
#[derive(Debug, Clone, Copy)]
pub enum Axis {
	X,
	Y,
}