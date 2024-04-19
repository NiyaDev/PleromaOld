

/// Keyboard Keys stolen from raylib
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Key {
	/// Key: NULL, used for no key pressed
	Null		= 0,
	/// Key: '
	Apostrophe	= 39,
	/// Key: ,
	Comma		= 44,
	/// Key: -
	Minus		= 45,
	/// Key: .
	Period		= 46,
	/// Key: /
	Slash		= 47,
	/// Key: 0
	Zero		= 48,
	/// Key: 1
	One			= 49,
	/// Key: 2
	Two			= 50,
	/// Key: 3
	Three		= 51,
	/// Key: 4
	Four		= 52,
	/// Key: 5
	Five		= 53,
	/// Key: 6
	Six			= 54,
	/// Key: 7
	Seven		= 55,
	/// Key: 8
	Eight		= 56,
	/// Key: 9
	Nine		= 57,
	/// Key: ;
	Semicolon	= 59,
	/// Key: =
	Equal		= 61,
	/// Key: A | a
	A			= 65,
	/// Key: B | b
	B			= 66,
	/// Key: C | c
	C			= 67,
	/// Key: D | d
	D			= 68,
	/// Key: E | e
	E			= 69,
	/// Key: F | f
	F			= 70,
	/// Key: G | g
	G			= 71,
	/// Key: H | h
	H			= 72,
	/// Key: I | i
	I			= 73,
	/// Key: J | j
	J			= 74,
	/// Key: K | k
	K			= 75,
	/// Key: L | l
	L			= 76,
	/// Key: M | m
	M			= 77,
	/// Key: N | n
	N			= 78,
	/// Key: O | o
	O			= 79,
	/// Key: P | p
	P			= 80,
	/// Key: Q | q
	Q			= 81,
	/// Key: R | r
	R			= 82,
	/// Key: S | s
	S			= 83,
	/// Key: T | t
	T			= 84,
	/// Key: U | u
	U			= 85,
	/// Key: V | v
	V			= 86,
	/// Key: W | w
	W			= 87,
	/// Key: X | x
	X			= 88,
	/// Key: Y | y
	Y			= 89,
	/// Key: Z | z
	Z			= 90,
	/// Key: [
	LeftBracket	= 91,
	/// Key: '\'
	Backslash	= 92,
	/// Key: ]
	RightBracket = 93,
	/// Key: `,
	Grave		= 96,
	/// Key: Space
	Space		= 32,
	/// Key: Esc
	Escape		= 256,
	/// Key: Enter
	Enter		= 257,
	/// Key: Tab
	Tab			= 258,
	/// Key: Backspace
	Backspace 	= 259,
	/// Key: Ins
	Insert		= 260,
	/// Key: Del
	Delete		= 261,
	/// Key: Cursor right
	Right		= 262,
	/// Key: Cursor left
	Left		= 263,
	/// Key: Cursor down
	Down		= 264,
	/// Key: Cursor up
	Up			= 265,
	/// Key: Page up
	PageUp		= 266,
	/// Key: Page down
	PageDown	= 267,
	/// Key: Home
	Home		= 268,
	/// Key: End
	End			= 269,
	/// Key: Caps lock
	CapsLock	= 280,
	/// Key: Scroll down
	ScrollLock	= 281,
	/// Key: Num lock
	NumLock		= 282,
	/// Key: Print screen
	PrintScreen	= 283,
	/// Key: Pause
	Pause		= 284,
	/// Key: F1
	F1			= 290,
	/// Key: F2
	F2			= 291,
	/// Key: F3
	F3			= 292,
	/// Key: F4
	F4			= 293,
	/// Key: F5
	F5			= 294,
	/// Key: F6
	F6			= 295,
	/// Key: F7
	F7			= 296,
	/// Key: F8
	F8			= 297,
	/// Key: F9
	F9			= 298,
	/// Key: F10
	F10			= 299,
	/// Key: F11
	F11			= 300,
	/// Key: F12
	F12			= 301,
	/// Key: Shift left
	LeftShift	= 340,
	/// Key: Control left
	LeftControl	= 341,
	/// Key: Alt left
	LeftAlt		= 342,
	/// Key: Super left
	LeftSuper	= 343,
	/// Key: Shift right
	RightShift	= 344,
	/// Key: Control right
	RightControl = 345,
	/// Key: Alt right,
	RightAlt	= 346,
	/// Key: Super right
	RightSuper	= 347,
	/// Key: KB menu
	KbMenu		= 348,
	/// Key: Keypad 0
	Kp0			= 320,
	/// Key: Keypad 1
	Kp1			= 321,
	/// Key: Keypad 2
	Kp2			= 322,
	/// Key: Keypad 3
	Kp3			= 323,
	/// Key: Keypad 4
	Kp4			= 324,
	/// Key: Keypad 5
	Kp5			= 325,
	/// Key: Keypad 6
	Kp6			= 326,
	/// Key: Keypad 7
	Kp7			= 327,
	/// Key: Keypad 8
	Kp8			= 328,
	/// Key: Keypad 9
	Kp9			= 329,
	/// Key: Keypad .
	KpDecimal	= 330,
	/// Key: Keypad /
	KpDivide	= 331,
	/// Key: Keypad *
	KpMultiply	= 332,
	/// Key: Keypad -
	KpSubtract	= 333,
	/// Key: Keypad +
	KpAdd		= 334,
	/// Key: Keypad Enter
	KpEnter		= 335,
	/// Key: Keypad =
	KpEqual		= 33,
	/// Key: Android back button6,
	Back		= 4,
	/// Key: Android volume up button
	VolumeUp	= 24,
	// Key: Android volume down button
	VolumeDown	= 25,
}
impl From<i32> for Key {
	fn from(value: i32) -> Self {
		match value {
			4   => { Key::Back }
			24  => { Key::VolumeUp }
			25  => { Key::VolumeDown }
			39  => { Key::Apostrophe }
			44  => { Key::Comma }
			45  => { Key::Minus }
			46  => { Key::Period }
			47  => { Key::Slash }
			48  => { Key::Zero }
			49  => { Key::One }
			50  => { Key::Two }
			51  => { Key::Three }
			52  => { Key::Four }
			53  => { Key::Five }
			54  => { Key::Six }
			55  => { Key::Seven }
			56  => { Key::Eight }
			57  => { Key::Nine }
			59 => { Key::Semicolon }
			61  => { Key::Equal }
			65  => { Key::A }
			66  => { Key::B }
			67  => { Key::C }
			68  => { Key::D }
			69  => { Key::E }
			70  => { Key::F }
			71  => { Key::G }
			72  => { Key::H }
			73  => { Key::I }
			74  => { Key::J }
			75  => { Key::K }
			76  => { Key::L }
			77  => { Key::M }
			78  => { Key::N }
			79  => { Key::O }
			80  => { Key::P }
			81  => { Key::Q }
			82  => { Key::R }
			83  => { Key::S }
			84  => { Key::T }
			85  => { Key::U }
			86  => { Key::V }
			87  => { Key::W }
			88  => { Key::X }
			89  => { Key::Y }
			90  => { Key::Z }
			91  => { Key::LeftBracket }
			92  => { Key::Backslash }
			93  => { Key::RightBracket }
			96  => { Key::Grave }
			32  => { Key::Space }
			256 => { Key::Escape }
			257 => { Key::Enter }
			258 => { Key::Tab }
			259 => { Key::Backspace }
			260 => { Key::Insert }
			261 => { Key::Delete }
			262 => { Key::Right }
			263 => { Key::Left }
			264 => { Key::Down }
			265 => { Key::Up }
			266 => { Key::PageUp }
			267 => { Key::PageDown }
			268 => { Key::Home }
			269 => { Key::End }
			280 => { Key::CapsLock }
			281 => { Key::ScrollLock }
			282 => { Key::NumLock }
			283 => { Key::PrintScreen }
			284 => { Key::Pause }
			290 => { Key::F1 }
			291 => { Key::F2 }
			292 => { Key::F3 }
			293 => { Key::F4 }
			294 => { Key::F5 }
			295 => { Key::F6 }
			296 => { Key::F7 }
			297 => { Key::F8 }
			298 => { Key::F9 }
			299 => { Key::F10 }
			300 => { Key::F11 }
			301 => { Key::F12 }
			340 => { Key::LeftShift }
			341 => { Key::LeftControl }
			342 => { Key::LeftAlt }
			343 => { Key::LeftSuper }
			344 => { Key::RightShift }
			345 => { Key::RightControl }
			346 => { Key::RightAlt }
			347 => { Key::RightSuper }
			348 => { Key::KbMenu }
			320 => { Key::Kp0 }
			321 => { Key::Kp1 }
			322 => { Key::Kp2 }
			323 => { Key::Kp3 }
			324 => { Key::Kp4 }
			325 => { Key::Kp5 }
			326 => { Key::Kp6 }
			327 => { Key::Kp7 }
			328 => { Key::Kp8 }
			329 => { Key::Kp9 }
			330 => { Key::KpDecimal }
			331 => { Key::KpDivide }
			332 => { Key::KpMultiply }
			333 => { Key::KpSubtract }
			334 => { Key::KpAdd }
			335 => { Key::KpEnter }
			336 => { Key::KpEqual }

			_ => { Key::Null }
		}
	}
}