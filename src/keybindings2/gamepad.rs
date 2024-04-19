

/// Gamepad Keys stolen from raylib
#[derive(Debug, Clone, Copy)]
pub enum GamepadButton {
	/// Unknown button, just for error checking
	Unknown			= 0,
	/// Gamepad left DPAD up button
	LeftFaceUp		= 1,
	/// Gamepad left DPAD right button
	LeftFaceRight	= 2,
	/// Gamepad left DPAD down button
	LeftFaceDown	= 3,
	/// Gamepad left DPAD left button
	LeftFaceLeft	= 4,
	/// Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
	RightFaceUp		= 5,
	/// Gamepad right button right (i.e. PS3: Square, Xbox: X)
	RightFaceRight	= 6,
	/// Gamepad right button down (i.e. PS3: Cross, Xbox: A)
	RightFaceDown	= 7,
	/// Gamepad right button left (i.e. PS3: Circle, Xbox: B)
	RightFaceLeft	= 8,
	/// Gamepad top/back trigger left (first), it could be a trailing button
	LeftTrigger1	= 9,
	/// Gamepad top/back trigger left (second), it could be a trailing button
	LeftTrigger2	= 10,
	/// Gamepad top/back trigger right (one), it could be a trailing button
	RightTrigger1	= 11,
	/// Gamepad top/back trigger right (second), it could be a trailing button
	RightTrigger2	= 12,
	/// Gamepad center buttons, left one (i.e. PS3: Select)
	MiddleLeft		= 13,
	/// Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
	Middle			= 14,
	/// Gamepad center buttons, right one (i.e. PS3: Start)
	MiddleRight		= 15,
	/// Gamepad joystick pressed button left
	LeftThumb		= 16,
	/// Gamepad joystick pressed button right
	RightThumb		= 17,
}

/// Gamepad Axis stolen from raylib
#[derive(Debug, Clone, Copy)]
pub enum GamepadAxis {
	/// Gamepad left stick X axis
	LeftX			= 0,
	/// Gamepad left stick Y axis
	LeftY			= 1,
	/// Gamepad right stick X axis
	RightX			= 2,
	/// Gamepad right stick Y axis
	RightY			= 3,
	/// Gamepad back trigger left, pressure level: [1..-1]
	LeftTrigger		= 4,
	/// Gamepad back trigger right, pressure level: [1..-1]
	RightTrigger	= 5,
}