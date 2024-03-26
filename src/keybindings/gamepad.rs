

/// Gamepad Keys stolen from raylib
#[derive(Debug, Clone, Copy)]
pub enum GamepadButton {
	Unknown			= 0,	// Unknown button, just for error checking
	LeftFaceUp		= 1,	// Gamepad left DPAD up button
	LeftFaceRight	= 2,	// Gamepad left DPAD right button
	LeftFaceDown	= 3,	// Gamepad left DPAD down button
	LeftFaceLeft	= 4,	// Gamepad left DPAD left button
	RightFaceUp		= 5,	// Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
	RightFaceRight	= 6,	// Gamepad right button right (i.e. PS3: Square, Xbox: X)
	RightFaceDown	= 7,	// Gamepad right button down (i.e. PS3: Cross, Xbox: A)
	RightFaceLeft	= 8,	// Gamepad right button left (i.e. PS3: Circle, Xbox: B)
	LeftTrigger1	= 9,	// Gamepad top/back trigger left (first), it could be a trailing button
	LeftTrigger2	= 10,	// Gamepad top/back trigger left (second), it could be a trailing button
	RightTrigger1	= 11,	// Gamepad top/back trigger right (one), it could be a trailing button
	RightTrigger2	= 12,	// Gamepad top/back trigger right (second), it could be a trailing button
	MiddleLeft		= 13,	// Gamepad center buttons, left one (i.e. PS3: Select)
	Middle			= 14,	// Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
	MiddleRight		= 15,	// Gamepad center buttons, right one (i.e. PS3: Start)
	LeftThumb		= 16,	// Gamepad joystick pressed button left
	RightThumb		= 17,	// Gamepad joystick pressed button right
}

/// Gamepad Axis stolen from raylib
#[derive(Debug, Clone, Copy)]
pub enum GamepadAxis {
	LeftX			= 0,	// Gamepad left stick X axis
	LeftY			= 1,	// Gamepad left stick Y axis
	RightX			= 2,	// Gamepad right stick X axis
	RightY			= 3,	// Gamepad right stick Y axis
	LeftTrigger		= 4,	// Gamepad back trigger left, pressure level: [1..-1]
	RightTrigger	= 5,	// Gamepad back trigger right, pressure level: [1..-1]
}