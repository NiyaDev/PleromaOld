

//= Imports


//= Structures and Enumerations

/// Keyboard Keys stolen from raylib
#[derive(Debug, Clone)]
pub enum KeyboardKey {
	Null		= 0,	/// Key: NULL, used for no key pressed
	Apostrophe	= 39,	/// Key: '
	Comma		= 44,	/// Key: ,
	Minus		= 45,	/// Key: -
	Period		= 46,	/// Key: .
	Slash		= 47,	/// Key: /
	Zero		= 48,	/// Key: 0
	One			= 49,	/// Key: 1
	Two			= 50,	/// Key: 2
	Three		= 51,	/// Key: 3
	Four		= 52,	/// Key: 4
	Five		= 53,	/// Key: 5
	Six			= 54,	/// Key: 6
	Seven		= 55,	/// Key: 7
	Eight		= 56,	/// Key: 8
	Nine		= 57,	/// Key: 9
	Semicolon	= 59,	/// Key: ;
	Equal		= 61,	/// Key: =
	A			= 65,	/// Key: A | a
	B			= 66,	/// Key: B | b
	C			= 67,	/// Key: C | c
	D			= 68,	/// Key: D | d
	E			= 69,	/// Key: E | e
	F			= 70,	/// Key: F | f
	G			= 71,	/// Key: G | g
	H			= 72,	/// Key: H | h
	I			= 73,	/// Key: I | i
	J			= 74,	/// Key: J | j
	K			= 75,	/// Key: K | k
	L			= 76,	/// Key: L | l
	M			= 77,	/// Key: M | m
	N			= 78,	/// Key: N | n
	O			= 79,	/// Key: O | o
	P			= 80,	/// Key: P | p
	Q			= 81,	/// Key: Q | q
	R			= 82,	/// Key: R | r
	S			= 83,	/// Key: S | s
	T			= 84,	/// Key: T | t
	U			= 85,	/// Key: U | u
	V			= 86,	/// Key: V | v
	W			= 87,	/// Key: W | w
	X			= 88,	/// Key: X | x
	Y			= 89,	/// Key: Y | y
	Z			= 90,	/// Key: Z | z
	LeftBracket	= 91,	/// Key: [
	Backslash	= 92,	/// Key: '\'
	RightBracket = 93,	/// Key: ]
	Grave		= 96,	/// Key: `
	Space		= 32,	/// Key: Space
	Escape		= 256,	/// Key: Esc
	Enter		= 257,	/// Key: Enter
	Tab			= 258,	/// Key: Tab
	Backspace 	= 259,	/// Key: Backspace
	Insert		= 260,	/// Key: Ins
	Delete		= 261,	/// Key: Del
	Right		= 262,	/// Key: Cursor right
	Left		= 263,	/// Key: Cursor left
	Down		= 264,	/// Key: Cursor down
	Up			= 265,	/// Key: Cursor up
	PageUp		= 266,	/// Key: Page up
	PageDown	= 267,	/// Key: Page down
	Home		= 268,	/// Key: Home
	End			= 269,	/// Key: End
	CapsLock	= 280,	/// Key: Caps lock
	ScrollLock	= 281,	/// Key: Scroll down
	NumLock		= 282,	/// Key: Num lock
	PrintScreen	= 283,	/// Key: Print screen
	Pause		= 284,	/// Key: Pause
	F1			= 290,	/// Key: F1
	F2			= 291,	/// Key: F2
	F3			= 292,	/// Key: F3
	F4			= 293,	/// Key: F4
	F5			= 294,	/// Key: F5
	F6			= 295,	/// Key: F6
	F7			= 296,	/// Key: F7
	F8			= 297,	/// Key: F8
	F9			= 298,	/// Key: F9
	F10			= 299,	/// Key: F10
	F11			= 300,	/// Key: F11
	F12			= 301,	/// Key: F12
	LeftShift	= 340,	/// Key: Shift left
	LeftControl	= 341,	/// Key: Control left
	LeftAlt		= 342,	/// Key: Alt left
	LeftSuper	= 343,	/// Key: Super left
	RightShift	= 344,	/// Key: Shift right
	RightControl = 345,	/// Key: Control right
	RightAlt	= 346,	/// Key: Alt right
	RightSuper	= 347,	/// Key: Super right
	KbMenu		= 348,	/// Key: KB menu
	Kp0			= 320,	/// Key: Keypad 0
	Kp1			= 321,	/// Key: Keypad 1
	Kp2			= 322,	/// Key: Keypad 2
	Kp3			= 323,	/// Key: Keypad 3
	Kp4			= 324,	/// Key: Keypad 4
	Kp5			= 325,	/// Key: Keypad 5
	Kp6			= 326,	/// Key: Keypad 6
	Kp7			= 327,	/// Key: Keypad 7
	Kp8			= 328,	/// Key: Keypad 8
	Kp9			= 329,	/// Key: Keypad 9
	KpDecimal	= 330,	/// Key: Keypad .
	KpDivide	= 331,	/// Key: Keypad /
	KpMultiply	= 332,	/// Key: Keypad *
	KpSubtract	= 333,	/// Key: Keypad -
	KpAdd		= 334,	/// Key: Keypad +
	KpEnter		= 335,	/// Key: Keypad Enter
	KpEqual		= 336,	/// Key: Keypad =
	Back		= 4,	/// Key: Android back button
	VolumeUp	= 24,	/// Key: Android volume up button
	VolumeDown	= 25,	// Key: Android volume down button
}
impl From<i64> for KeyboardKey {
	fn from(value: i64) -> Self {
		match value {
			4   => { KeyboardKey::Back }
			24  => { KeyboardKey::VolumeUp }
			25  => { KeyboardKey::VolumeDown }
			39  => { KeyboardKey::Apostrophe }
			44  => { KeyboardKey::Comma }
			45  => { KeyboardKey::Minus }
			46  => { KeyboardKey::Period }
			47  => { KeyboardKey::Slash }
			48  => { KeyboardKey::Zero }
			49  => { KeyboardKey::One }
			50  => { KeyboardKey::Two }
			51  => { KeyboardKey::Three }
			52  => { KeyboardKey::Four }
			53  => { KeyboardKey::Five }
			54  => { KeyboardKey::Six }
			55  => { KeyboardKey::Seven }
			56  => { KeyboardKey::Eight }
			57  => { KeyboardKey::Nine }
			59 => { KeyboardKey::Semicolon }
			61  => { KeyboardKey::Equal }
			65  => { KeyboardKey::A }
			66  => { KeyboardKey::B }
			67  => { KeyboardKey::C }
			68  => { KeyboardKey::D }
			69  => { KeyboardKey::E }
			70  => { KeyboardKey::F }
			71  => { KeyboardKey::G }
			72  => { KeyboardKey::H }
			73  => { KeyboardKey::I }
			74  => { KeyboardKey::J }
			75  => { KeyboardKey::K }
			76  => { KeyboardKey::L }
			77  => { KeyboardKey::M }
			78  => { KeyboardKey::N }
			79  => { KeyboardKey::O }
			80  => { KeyboardKey::P }
			81  => { KeyboardKey::Q }
			82  => { KeyboardKey::R }
			83  => { KeyboardKey::S }
			84  => { KeyboardKey::T }
			85  => { KeyboardKey::U }
			86  => { KeyboardKey::V }
			87  => { KeyboardKey::W }
			88  => { KeyboardKey::X }
			89  => { KeyboardKey::Y }
			90  => { KeyboardKey::Z }
			91  => { KeyboardKey::LeftBracket }
			92  => { KeyboardKey::Backslash }
			93  => { KeyboardKey::RightBracket }
			96  => { KeyboardKey::Grave }
			32  => { KeyboardKey::Space }
			256 => { KeyboardKey::Escape }
			257 => { KeyboardKey::Enter }
			258 => { KeyboardKey::Tab }
			259 => { KeyboardKey::Backspace }
			260 => { KeyboardKey::Insert }
			261 => { KeyboardKey::Delete }
			262 => { KeyboardKey::Right }
			263 => { KeyboardKey::Left }
			264 => { KeyboardKey::Down }
			265 => { KeyboardKey::Up }
			266 => { KeyboardKey::PageUp }
			267 => { KeyboardKey::PageDown }
			268 => { KeyboardKey::Home }
			269 => { KeyboardKey::End }
			280 => { KeyboardKey::CapsLock }
			281 => { KeyboardKey::ScrollLock }
			282 => { KeyboardKey::NumLock }
			283 => { KeyboardKey::PrintScreen }
			284 => { KeyboardKey::Pause }
			290 => { KeyboardKey::F1 }
			291 => { KeyboardKey::F2 }
			292 => { KeyboardKey::F3 }
			293 => { KeyboardKey::F4 }
			294 => { KeyboardKey::F5 }
			295 => { KeyboardKey::F6 }
			296 => { KeyboardKey::F7 }
			297 => { KeyboardKey::F8 }
			298 => { KeyboardKey::F9 }
			299 => { KeyboardKey::F10 }
			300 => { KeyboardKey::F11 }
			301 => { KeyboardKey::F12 }
			340 => { KeyboardKey::LeftShift }
			341 => { KeyboardKey::LeftControl }
			342 => { KeyboardKey::LeftAlt }
			343 => { KeyboardKey::LeftSuper }
			344 => { KeyboardKey::RightShift }
			345 => { KeyboardKey::RightControl }
			346 => { KeyboardKey::RightAlt }
			347 => { KeyboardKey::RightSuper }
			348 => { KeyboardKey::KbMenu }
			320 => { KeyboardKey::Kp0 }
			321 => { KeyboardKey::Kp1 }
			322 => { KeyboardKey::Kp2 }
			323 => { KeyboardKey::Kp3 }
			324 => { KeyboardKey::Kp4 }
			325 => { KeyboardKey::Kp5 }
			326 => { KeyboardKey::Kp6 }
			327 => { KeyboardKey::Kp7 }
			328 => { KeyboardKey::Kp8 }
			329 => { KeyboardKey::Kp9 }
			330 => { KeyboardKey::KpDecimal }
			331 => { KeyboardKey::KpDivide }
			332 => { KeyboardKey::KpMultiply }
			333 => { KeyboardKey::KpSubtract }
			334 => { KeyboardKey::KpAdd }
			335 => { KeyboardKey::KpEnter }
			336 => { KeyboardKey::KpEqual }

			_ => { KeyboardKey::Null }
		}
	}
}