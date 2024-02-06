

//= Allows


//= Imports
use crate::{*};


//= Constants


//= Structures & Enumerations

#[derive(Debug, Clone)]
pub struct CoreData {
	pub platform: Platform,
	pub rshapes: bool,
	pub rtextures: bool,
	pub rtext: bool,
	pub rmodels: bool,
	pub raudio: bool,

	pub window: Window,
	pub storage: Storage,
	pub input: Input,
	pub time: Time,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Window {
	pub title: &'static str,		// Window text title const pointer
	pub flags: Flag,				// Configuration flags (bit based), keeps window state
	pub ready: bool,				// Check if window has been initialized successfully
	pub fullscreen: bool,			// Check if fullscreen mode is enabled
	pub should_close: bool,			// Check if window set for closing
	pub resized_last_frame: bool,	// Check if window has been resized last frame
	pub event_waiting: bool,		// Wait for events before ending frame
	pub using_fbo: bool,			// Using FBO (RenderTexture) for rendering instead of default framebuffer

	pub position: Point,			// Window position (required on fullscreen toggle)
	pub previous_position: Point,	// Window previous position (required on borderless windowed toggle)
	pub display: Size,				// Display width and height (monitor, device-screen, LCD, ...)
	pub screen: Size,				// Screen width and height (used render area)
	pub previous_screen: Size,		// Screen previous width and height (required on borderless windowed toggle)
	pub current_fbo: Size,			// Current render width and height (depends on active fbo)
	pub render: Size,				// Framebuffer width and height (render area, including black bars if required)
	pub render_offset: Point,		// Offset from render area (must be divided by 2)
	pub screen_min: Size,			// Screen minimum width and height (for resizable window)
	pub scren_max: Size,			// Screen maximum width and height (for resizable window)
	pub screen_scale: Matrix,		// Matrix to scale screen (framebuffer rendering)

	pub dropped_filepaths: [&'static str; 100], // Dropped files strings
}

#[derive(Debug, Clone, PartialEq)]
pub struct Storage {
	// ???
}

#[derive(Debug, Clone, PartialEq)]
pub struct Input {
	pub keyboard: Keyboard,
	pub mouse: Mouse,
	pub touch: Touch,
	pub gamepad: Gamepad,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Keyboard {
	pub exit_key: KeyboardKey,
	pub current_key_state: [i8; MAX_KEYBOARD_KEYS],	// Registers current frame key state
	pub previous_key_state: [i8; MAX_KEYBOARD_KEYS],	// Registers previous frame key state

	// NOTE: Since key press logic involves comparing prev vs cur key state, we need to handle key repeats specially
	pub key_repeat_in_frame: [i8; MAX_KEYBOARD_KEYS],	// Registers key repeats for current frame.

	pub key_press_queue: [i32; MAX_KEY_PRESSED_QUEUE],	// Input keys queue
	pub key_press_queue_count: i32,						// Input keys queue count

	pub char_press_queue: [i32; MAX_CHAR_PRESSED_QUEUE],// Input characters queue
	pub char_press_queue_count: i32,					// Input characters queue count
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mouse {
	pub offset: Vector2,				// Mouse offset
	pub scale: Vector2,					// Mouse scaling
	pub current_position: Vector2,		// Mouse position on screen
	pub previous_position: Vector2,		// Previous mouse position

	pub cursor: MouseCursor,			// Tracks current mouse cursor
	pub cursor_hidden: bool,			// Track if cursor is hidden
	pub cursor_on_screen: bool,			// Tracks if cursor is inside client area

	pub current_button_state: [i8; MAX_MOUSE_BUTTONS],		// Registers current mouse button state
	pub previous_button_state: [i8; MAX_MOUSE_BUTTONS],	// Registers previous mouse button state
	pub current_wheel_move: Vector2,	// Registers current mouse wheel variation
	pub previous_wheel_move: Vector2,	// Registers previous mouse wheel variation
}

#[derive(Debug, Clone, PartialEq)]
pub struct Touch {
	// TODO: Don't really care about this that much
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gamepad {
	pub last_button_pressed: i32,
	pub axis_count: [i32; MAX_GAMEPADS],
	pub ready: [bool; MAX_GAMEPADS],
	pub name: [[u8; 64]; MAX_GAMEPADS],
	pub current_button_state: [[i8; MAX_GAMEPAD_BUTTONS]; MAX_GAMEPADS],
	pub previous_button_states: [[i8; MAX_GAMEPAD_BUTTONS]; MAX_GAMEPADS],
	pub axis_state: [[f32; MAX_GAMEPAD_AXIS]; MAX_GAMEPADS],
}

#[derive(Debug, Clone, PartialEq)]
pub struct Time {
	pub current: f64,
	pub previous: f64,
	pub update: f64,
	pub draw: f64,
	pub frame: f64,
	pub target: f64,
	pub base: u64,
	pub frame_counter: u32,
}