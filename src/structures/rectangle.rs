

/// Rectangle
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}

pub const ZERO: Rectangle = Rectangle { x: 0.0, y: 0.0, width: 0.0, height: 0.0};
