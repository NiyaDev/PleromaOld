

/// ### GamepadButton
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
	/// Gamepad joystick left axis is greater than 0
	LeftXp			= 18,
	/// Gamepad joystick left axis is lesser than 0
	LeftXn			= 19,
	/// Gamepad joystick left axis is greater than 0
	LeftYp			= 20,
	/// Gamepad joystick left axis is lesser than 0
	LeftYn			= 21,
	/// Gamepad joystick right axis is greater than 0
	RightXp			= 22,
	/// Gamepad joystick right axis is lesser than 0
	RightXn			= 23,
	/// Gamepad joystick right axis is greater than 0
	RightYp			= 24,
	/// Gamepad joystick right axis is lesser than 0
	RightYn			= 25,
	/// Gamepad trigger axis is greater than
	LeftTrigger		= 26,
	/// Gamepad trigger axis is greater than
	RightTrigger	= 27,
}
impl Into<i32> for GamepadButton {
	fn into(self) -> i32 {
		match self {
			GamepadButton::Unknown			=> {  0 }
			GamepadButton::LeftFaceUp		=> {  1 }
			GamepadButton::LeftFaceRight	=> {  2 }
			GamepadButton::LeftFaceDown		=> {  3 }
			GamepadButton::LeftFaceLeft		=> {  4 }
			GamepadButton::RightFaceUp		=> {  5 }
			GamepadButton::RightFaceRight	=> {  6 }
			GamepadButton::RightFaceDown	=> {  7 }
			GamepadButton::RightFaceLeft	=> {  8 }
			GamepadButton::LeftTrigger1		=> {  9 }
			GamepadButton::LeftTrigger2		=> { 10 }
			GamepadButton::RightTrigger1	=> { 11 }
			GamepadButton::RightTrigger2	=> { 12 }
			GamepadButton::MiddleLeft		=> { 13 }
			GamepadButton::Middle			=> { 14 }
			GamepadButton::MiddleRight		=> { 15 }
			GamepadButton::LeftThumb		=> { 16 }
			GamepadButton::RightThumb		=> { 17 }
			GamepadButton::LeftXp			=> { 18 }
			GamepadButton::LeftXn			=> { 19 }
			GamepadButton::LeftYp			=> { 20 }
			GamepadButton::LeftYn			=> { 21 }
			GamepadButton::RightXp			=> { 22 }
			GamepadButton::RightXn			=> { 23 }
			GamepadButton::RightYp			=> { 24 }
			GamepadButton::RightYn			=> { 25 }
			GamepadButton::LeftTrigger		=> { 26 }
			GamepadButton::RightTrigger		=> { 27 }
		}
	}
}