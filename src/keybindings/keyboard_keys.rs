

//= Imports


//= Structures and Enumerations

/// Keyboard Keys stolen from raylib
#[derive(Debug, Clone)]
pub enum KeyboardKey {
	/// Key: NULL, used for no key pressed
	Null = 0,
	/// Key: '
	Apostrophe = 39,
	/// Key: ,
	Comma = 44,
	/// Key: -
	Minus = 45,
	/// Key: .
	Period = 46,
	/// Key: /
	Slash = 47,
	/// Key: 0
	Zero = 48,
	/// Key: 1
	One = 49,
	/// Key: 2
	Two = 50,
	/// Key: 3
	Three = 51,
	/// Key: 4
	Four = 52,
	/// Key: 5
	Five = 53,
	/// Key: 6
	Six = 54,
	/// Key: 7
	Seven = 55,
	/// Key: 8
	Eight = 56,
	/// Key: 9
	Nine = 57,
	/// Key: ;
	Semicolon = 59,
	/// Key: =
	Equal = 61,
	/// Key: A | a
	A = 65,
	/// Key: B | b
	B = 66,
	/// Key: C | c
	C = 67,
	/// Key: D | d
	D = 68,
	/// Key: E | e
	E = 69,
	/// Key: F | f
	F = 70,
	/// Key: G | g
	G = 71,
	/// Key: H | h
	H = 72,
	/// Key: I | i
	I = 73,
	/// Key: J | j
	J = 74,
	/// Key: K | k
	K = 75,
	/// Key: L | l
	L = 76,
	/// Key: M | m
	M = 77,
	/// Key: N | n
	N = 78,
	/// Key: O | o
	O = 79,
	/// Key: P | p
	P = 80,
	/// Key: Q | q
	Q = 81,
	/// Key: R | r
	R = 82,
	/// Key: S | s
	S = 83,
	/// Key: T | t
	T = 84,
	/// Key: U | u
	U = 85,
	/// Key: V | v
	V = 86,
	/// Key: W | w
	W = 87,
	/// Key: X | x
	X = 88,
	/// Key: Y | y
	Y = 89,
	/// Key: Z | z
	Z = 90,
	/// Key: [
	LeftBracket = 91,
	/// Key: '\'
	Backslash = 92,
	/// Key: ]
	RightBracket = 93,
	/// Key: `
	Grave = 96,
	/// Key: Space
	Space = 32,
	/// Key: Esc
	Escape = 256,
	/// Key: Enter
	Enter = 257,
	/// Key: Tab
	Tab = 258,
	/// Key: Backspace
	Backspace = 259,
	/// Key: Ins
	Insert = 260,
	/// Key: Del
	Delete = 261,
	/// Key: Cursor right
	Right = 262,
	/// Key: Cursor left
	Left = 263,
	/// Key: Cursor down
	Down = 264,
	/// Key: Cursor up
	Up = 265,
	/// Key: Page up
	PageUp = 266,
	/// Key: Page down
	PageDown = 267,
	/// Key: Home
	Home = 268,
	/// Key: End
	End = 269,
	/// Key: Caps lock
	CapsLock = 280,
	/// Key: Scroll down
	ScrollLock = 281,
	/// Key: Num lock
	NumLock = 282,
	/// Key: Print screen
	PrintScreen = 283,
	/// Key: Pause
	Pause = 284,
	/// Key: F1
	F1 = 290,
	/// Key: F2
	F2 = 291,
	/// Key: F3
	F3 = 292,
	/// Key: F4
	F4 = 293,
	/// Key: F5
	F5 = 294,
	/// Key: F6
	F6 = 295,
	/// Key: F7
	F7 = 296,
	/// Key: F8
	F8 = 297,
	/// Key: F9
	F9 = 298,
	/// Key: F10
	F10 = 299,
	/// Key: F11
	F11 = 300,
	/// Key: F12
	F12 = 301,
	/// Key: Shift left
	LeftShift = 340,
	/// Key: Control left
	LeftControl = 341,
	/// Key: Alt left
	LeftAlt = 342,
	/// Key: Super left
	LeftSuper = 343,
	/// Key: Shift right
	RightShift = 344,
	/// Key: Control right
	RightControl = 345,
	/// Key: Alt right
	RightAlt = 346,
	/// Key: Super right
	RightSuper = 347,
	/// Key: KB menu
	KbMenu = 348,
	/// Key: Keypad 0
	Kp0 = 320,
	/// Key: Keypad 1
	Kp1 = 321,
	/// Key: Keypad 2
	Kp2 = 322,
	/// Key: Keypad 3
	Kp3 = 323,
	/// Key: Keypad 4
	Kp4 = 324,
	/// Key: Keypad 5
	Kp5 = 325,
	/// Key: Keypad 6
	Kp6 = 326,
	/// Key: Keypad 7
	Kp7 = 327,
	/// Key: Keypad 8
	Kp8 = 328,
	/// Key: Keypad 9
	Kp9 = 329,
	/// Key: Keypad .
	KpDecimal = 330,
	/// Key: Keypad /
	KpDivide = 331,
	/// Key: Keypad *
	KpMultiply = 332,
	/// Key: Keypad -
	KpSubtract = 333,
	/// Key: Keypad +
	KpAdd = 334,
	/// Key: Keypad Enter
	KpEnter = 335,
	/// Key: Keypad =
	KpEqual = 336,
	/// Key: Android back button
	Back = 4,
	/// Key: Android volume up button
	VolumeUp = 24,
	/// Key: Android volume down button
	VolumeDown = 25,
}