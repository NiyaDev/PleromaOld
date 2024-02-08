

//= Imports
use std::fmt::Display;


//= Constants
pub const MAX_KEYBOARD_KEYS: usize = 512;
pub const MAX_KEY_PRESSED_QUEUE: usize = 16;
pub const MAX_CHAR_PRESSED_QUEUE: usize = 16;
pub const MAX_MOUSE_BUTTONS: usize = 8;
pub const MAX_GAMEPADS: usize = 4;
pub const MAX_GAMEPAD_BUTTONS: usize = 32;
pub const MAX_GAMEPAD_AXIS: usize = 8;
pub const MAX_FILEPATH_LENGTH: usize = 4096;


//= Structures & Enumerations

/// Keyboard keys
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyboardKey {
	Null			= 0, 		// Key: NULL, used for no key pressed
	// Alphanumeric keys
	Apostrophe		= 39,		// Key: '
	Comma			= 44,		// Key: ,
	Minus			= 45,		// Key: -
	Period			= 46,		// Key: .
	Slash			= 47,		// Key: /
	Zero			= 48,		// Key: 0
	One				= 49,		// Key: 1
	Two				= 50,		// Key: 2
	Three			= 51,		// Key: 3
	Four			= 52,		// Key: 4
	Five			= 53,		// Key: 5
	Six				= 54,		// Key: 6
	Seven			= 55,		// Key: 7
	Eight			= 56,		// Key: 8
	Nine			= 57,		// Key: 9
	Semicolon		= 59,		// Key: ;
	Equal			= 61,		// Key: =
	A				= 65,		// Key: A | a
	B				= 66,		// Key: B | b
	C				= 67,		// Key: C | c
	D				= 68,		// Key: D | d
	E				= 69,		// Key: E | e
	F				= 70,		// Key: F | f
	G				= 71,		// Key: G | g
	H				= 72,		// Key: H | h
	I				= 73,		// Key: I | i
	J				= 74,		// Key: J | j
	K				= 75,		// Key: K | k
	L				= 76,		// Key: L | l
	M				= 77,		// Key: M | m
	N				= 78,		// Key: N | n
	O				= 79,		// Key: O | o
	P				= 80,		// Key: P | p
	Q				= 81,		// Key: Q | q
	R				= 82,		// Key: R | r
	S				= 83,		// Key: S | s
	T				= 84,		// Key: T | t
	U				= 85,		// Key: U | u
	V				= 86,		// Key: V | v
	W				= 87,		// Key: W | w
	X				= 88,		// Key: X | x
	Y				= 89,		// Key: Y | y
	Z				= 90,		// Key: Z | z
	LeftBracket		= 91,		// Key: [
	Backslash		= 92,		// Key: '\'
	RightBracket	= 93,		// Key: ]
	Grave			= 96,		// Key: `
	// Function keys
	Space			= 32, 		// Key: Space
	Escape			= 256,		// Key: Esc
	Enter			= 257,		// Key: Enter
	Tab				= 258,		// Key: Tab
	Backspace		= 259,		// Key: Backspace
	Insert			= 260,		// Key: Ins
	Delete			= 261,		// Key: Del
	Right			= 262,		// Key: Cursor right
	Left			= 263,		// Key: Cursor left
	Down			= 264,		// Key: Cursor down
	Up				= 265,		// Key: Cursor up
	PageUp			= 266,		// Key: Page up
	PageDown		= 267,		// Key: Page down
	Home			= 268,		// Key: Home
	End				= 269,		// Key: End
	CapsLock		= 280,		// Key: Caps lock
	ScrollLock		= 281,		// Key: Scroll down
	NumLock			= 282,		// Key: Num lock
	PrintScreen		= 283,		// Key: Print screen
	Pause			= 284,		// Key: Pause
	F1				= 290,		// Key: F1
	F2				= 291,		// Key: F2
	F3				= 292,		// Key: F3
	F4				= 293,		// Key: F4
	F5				= 294,		// Key: F5
	F6				= 295,		// Key: F6
	F7				= 296,		// Key: F7
	F8				= 297,		// Key: F8
	F9				= 298,		// Key: F9
	F10				= 299,		// Key: F10
	F11				= 300,		// Key: F11
	F12				= 301,		// Key: F12
	LeftShift		= 340,		// Key: Shift left
	LeftControl		= 341,		// Key: Control left
	LeftAlt			= 342,		// Key: Alt left
	LeftSuper		= 343,		// Key: Super left
	RightShift		= 344,		// Key: Shift right
	RightControl	= 345,		// Key: Control right
	RightAlt		= 346,		// Key: Alt right
	RightSuper		= 347,		// Key: Super right
	KbMenu			= 348,		// Key: KB menu
	// Keypad keys
	Kp0				= 320,		// Key: Keypad 0
	Kp1				= 321,		// Key: Keypad 1
	Kp2				= 322,		// Key: Keypad 2
	Kp3				= 323,		// Key: Keypad 3
	Kp4				= 324,		// Key: Keypad 4
	Kp5				= 325,		// Key: Keypad 5
	Kp6				= 326,		// Key: Keypad 6
	Kp7				= 327,		// Key: Keypad 7
	Kp8				= 328,		// Key: Keypad 8
	Kp9				= 329,		// Key: Keypad 9
	KpDecimal		= 330,		// Key: Keypad .
	KpDivide		= 331,		// Key: Keypad /
	KpMultiply		= 332,		// Key: Keypad *
	KpSubtract		= 333,		// Key: Keypad -
	KpAdd			= 334,		// Key: Keypad +
	KpEnter			= 335,		// Key: Keypad Enter
	KpEqual			= 336,		// Key: Keypad =
	// Android key buttons
	Back			= 4,		// Key: Android back button
	Menu			= 5,		// Key: Android menu button
	VolumeUp		= 24,		// Key: Android volume up button
	VolumeDown		= 25		// Key: Android volume down button
}
impl From<glfw::Key> for KeyboardKey {
	fn from(value: glfw::Key) -> Self {
		match value {
			glfw::Key::Space => return KeyboardKey::Space,
			glfw::Key::Apostrophe => KeyboardKey::Apostrophe,
			glfw::Key::Comma => KeyboardKey::Comma,
			glfw::Key::Minus => KeyboardKey::Minus,
			glfw::Key::Period => KeyboardKey::Period,
			glfw::Key::Slash => KeyboardKey::Slash,
			glfw::Key::Num0 => KeyboardKey::Zero,
			glfw::Key::Num1 => KeyboardKey::One,
			glfw::Key::Num2 => KeyboardKey::Two,
			glfw::Key::Num3 => KeyboardKey::Three,
			glfw::Key::Num4 => KeyboardKey::Four,
			glfw::Key::Num5 => KeyboardKey::Five,
			glfw::Key::Num6 => KeyboardKey::Six,
			glfw::Key::Num7 => KeyboardKey::Seven,
			glfw::Key::Num8 => KeyboardKey::Eight,
			glfw::Key::Num9 => KeyboardKey::Nine,
			glfw::Key::Semicolon => KeyboardKey::Semicolon,
			glfw::Key::Equal => KeyboardKey::Equal,
			glfw::Key::A => KeyboardKey::A,
			glfw::Key::B => KeyboardKey::B,
			glfw::Key::C => KeyboardKey::C,
			glfw::Key::D => KeyboardKey::D,
			glfw::Key::E => KeyboardKey::E,
			glfw::Key::F => KeyboardKey::F,
			glfw::Key::G => KeyboardKey::G,
			glfw::Key::H => KeyboardKey::H,
			glfw::Key::I => KeyboardKey::I,
			glfw::Key::J => KeyboardKey::J,
			glfw::Key::K => KeyboardKey::K,
			glfw::Key::L => KeyboardKey::L,
			glfw::Key::M => KeyboardKey::M,
			glfw::Key::N => KeyboardKey::N,
			glfw::Key::O => KeyboardKey::O,
			glfw::Key::P => KeyboardKey::P,
			glfw::Key::Q => KeyboardKey::Q,
			glfw::Key::R => KeyboardKey::R,
			glfw::Key::S => KeyboardKey::S,
			glfw::Key::T => KeyboardKey::T,
			glfw::Key::U => KeyboardKey::U,
			glfw::Key::V => KeyboardKey::V,
			glfw::Key::W => KeyboardKey::W,
			glfw::Key::X => KeyboardKey::X,
			glfw::Key::Y => KeyboardKey::Y,
			glfw::Key::Z => KeyboardKey::Z,
			glfw::Key::LeftBracket => KeyboardKey::LeftBracket,
			glfw::Key::Backslash => KeyboardKey::Backslash,
			glfw::Key::RightBracket => KeyboardKey::RightBracket,
			glfw::Key::GraveAccent => KeyboardKey::Grave,
			glfw::Key::World1 => KeyboardKey::Null, // TODO
			glfw::Key::World2 => KeyboardKey::Null, // TODO
			glfw::Key::Escape => KeyboardKey::Escape,
			glfw::Key::Enter => KeyboardKey::Enter,
			glfw::Key::Tab => KeyboardKey::Tab,
			glfw::Key::Backspace => KeyboardKey::Backspace,
			glfw::Key::Insert => KeyboardKey::Insert,
			glfw::Key::Delete => KeyboardKey::Delete,
			glfw::Key::Right => KeyboardKey::Right,
			glfw::Key::Left => KeyboardKey::Left,
			glfw::Key::Down => KeyboardKey::Down,
			glfw::Key::Up => KeyboardKey::Up,
			glfw::Key::PageUp => KeyboardKey::PageUp,
			glfw::Key::PageDown => KeyboardKey::PageDown,
			glfw::Key::Home => KeyboardKey::Home,
			glfw::Key::End => KeyboardKey::End,
			glfw::Key::CapsLock => KeyboardKey::CapsLock,
			glfw::Key::ScrollLock => KeyboardKey::ScrollLock,
			glfw::Key::NumLock => KeyboardKey::NumLock,
			glfw::Key::PrintScreen => KeyboardKey::PrintScreen,
			glfw::Key::Pause => KeyboardKey::Pause,
			glfw::Key::F1 => KeyboardKey::F1,
			glfw::Key::F2 => KeyboardKey::F2,
			glfw::Key::F3 => KeyboardKey::F3,
			glfw::Key::F4 => KeyboardKey::F4,
			glfw::Key::F5 => KeyboardKey::F5,
			glfw::Key::F6 => KeyboardKey::F6,
			glfw::Key::F7 => KeyboardKey::F7,
			glfw::Key::F8 => KeyboardKey::F8,
			glfw::Key::F9 => KeyboardKey::F9,
			glfw::Key::F10 => KeyboardKey::F10,
			glfw::Key::F11 => KeyboardKey::F11,
			glfw::Key::F12 => KeyboardKey::F12,
			glfw::Key::F13 => KeyboardKey::Null, // TODO
			glfw::Key::F14 => KeyboardKey::Null, // TODO
			glfw::Key::F15 => KeyboardKey::Null, // TODO
			glfw::Key::F16 => KeyboardKey::Null, // TODO
			glfw::Key::F17 => KeyboardKey::Null, // TODO
			glfw::Key::F18 => KeyboardKey::Null, // TODO
			glfw::Key::F19 => KeyboardKey::Null, // TODO
			glfw::Key::F20 => KeyboardKey::Null, // TODO
			glfw::Key::F21 => KeyboardKey::Null, // TODO
			glfw::Key::F22 => KeyboardKey::Null, // TODO
			glfw::Key::F23 => KeyboardKey::Null, // TODO
			glfw::Key::F24 => KeyboardKey::Null, // TODO
			glfw::Key::F25 => KeyboardKey::Null, // TODO
			glfw::Key::Kp0 => KeyboardKey::Kp0,
			glfw::Key::Kp1 => KeyboardKey::Kp1,
			glfw::Key::Kp2 => KeyboardKey::Kp2,
			glfw::Key::Kp3 => KeyboardKey::Kp3,
			glfw::Key::Kp4 => KeyboardKey::Kp4,
			glfw::Key::Kp5 => KeyboardKey::Kp5,
			glfw::Key::Kp6 => KeyboardKey::Kp6,
			glfw::Key::Kp7 => KeyboardKey::Kp7,
			glfw::Key::Kp8 => KeyboardKey::Kp8,
			glfw::Key::Kp9 => KeyboardKey::Kp9,
			glfw::Key::KpDecimal => KeyboardKey::KpDecimal,
			glfw::Key::KpDivide => KeyboardKey::KpDivide,
			glfw::Key::KpMultiply => KeyboardKey::KpMultiply,
			glfw::Key::KpSubtract => KeyboardKey::KpSubtract,
			glfw::Key::KpAdd => KeyboardKey::KpAdd,
			glfw::Key::KpEnter => KeyboardKey::KpEnter,
			glfw::Key::KpEqual => KeyboardKey::KpEqual,
			glfw::Key::LeftShift => KeyboardKey::LeftShift,
			glfw::Key::LeftControl => KeyboardKey::LeftControl,
			glfw::Key::LeftAlt => KeyboardKey::LeftAlt,
			glfw::Key::LeftSuper => KeyboardKey::LeftSuper,
			glfw::Key::RightShift => KeyboardKey::RightShift,
			glfw::Key::RightControl => KeyboardKey::RightControl,
			glfw::Key::RightAlt => KeyboardKey::RightAlt,
			glfw::Key::RightSuper => KeyboardKey::RightSuper,
			glfw::Key::Menu => KeyboardKey::Menu,
			glfw::Key::Unknown => KeyboardKey::Null,
		}
	}
}

/// Mouse buttons
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseButton {
	Left	= 0,	// Mouse button left
	Right	= 1,	// Mouse button right
	Middle	= 2,	// Mouse button middle (pressed wheel)
	Side	= 3,	// Mouse button side (advanced mouse device)
	Extra	= 4,	// Mouse button extra (advanced mouse device)
	Forward	= 5,	// Mouse button forward (advanced mouse device)
	Back	= 6,	// Mouse button back (advanced mouse device)
}

/// Current mouse cursor appearance
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseCursor {
	Default			= 0,	// Default pointer shape
	Arrow			= 1,	// Arrow shape
	Ibeam			= 2,	// Text writing cursor shape
	Crosshair		= 3,	// Cross shape
	PointingHand	= 4,	// Pointing hand cursor
	ResizeEw		= 5,	// Horizontal resize/move arrow shape
	ResizeNs		= 6,	// Vertical resize/move arrow shape
	ResizeNwse		= 7,	// Top-left to bottom-right diagonal resize/move arrow shape
	ResizeNesw		= 8,	// The top-right to bottom-left diagonal resize/move arrow shape
	ResizeAll		= 9,	// The omnidirectional resize/move cursor shape
	NotAllowed		= 10	// The operation-not-allowed shape
}

/// Button actions
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Action {
	Release,
	Press,
	Repeat,
}

/// Platforms to run on
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Platform {
	Desktop,
	DesktopSdl,	// TODO
	Web,		// TODO
	DRM,		// TODO
	Android,	// TODO ?? Don't really care enough
}

/// A point on the screen
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
	pub x: i32,
	pub y: i32,
}
impl Display for Point {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}x{}", self.x, self.y)
	}
}

/// The size of the screen
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size {
	pub width: u32,
	pub height: u32,
}
impl Display for Size {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}x{}", self.width, self.height)
	}
}
