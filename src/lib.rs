

//= Allows


//= Imports
pub mod window;
pub mod logging;
pub mod vectors;
pub mod matrix;
pub mod platform;

use {window::{*, data_structures::*}, vectors::*, matrix::*};


//= Constants
pub const RAYLIB_VERSION_MAJOR: i32 = 5;
pub const RAYLIB_VERSION_MINOR: i32 = 1;
pub const RAYLIB_VERSION_PATCH: i32 = 0;
pub const RAYLIB_VERSION: &str = "5.1.0";


//= Static
pub static mut CORE: CoreData = CoreData {
	platform: Platform::Desktop,
	rshapes:	false,
	rtextures:	false,
	rtext:		false,
	rmodels:	false,
	raudio:		false,

	window: Window {
		title: "",
		flags: 0,
		ready: false,
		fullscreen: false,
		should_close: false,
		resized_last_frame: false,
		event_waiting: false,
		using_fbo: false,
		position: Point{x: 0, y: 0},
		previous_position: Point{x: 0, y: 0},
		display: Size{width: 0, height: 0},
		screen: Size{width: 0, height: 0},
		previous_screen: Size{width: 0, height: 0},
		current_fbo: Size{width: 0, height: 0},
		render: Size{width: 0, height: 0},
		render_offset: Point{x: 0, y: 0},
		screen_min: Size{width: 0, height: 0},
		scren_max: Size{width: 0, height: 0},
		screen_scale: Matrix{
			m0:  0.0,
			m4:  0.0,
			m8:  0.0,
			m12: 0.0,
			m1:  0.0,
			m5:  0.0,
			m9:  0.0,
			m13: 0.0,
			m2:  0.0,
			m6:  0.0,
			m10: 0.0,
			m14: 0.0,
			m3:  0.0,
			m7:  0.0,
			m11: 0.0,
			m15: 0.0,
		},
		dropped_filepaths: [""; 100],
	},
	storage: Storage{},
	input: Input{
		keyboard: Keyboard{
			exit_key: KeyboardKey::KeyEscape,
			current_key_state: [0; MAX_KEYBOARD_KEYS],
			previous_key_state: [0; MAX_KEYBOARD_KEYS],
			key_repeat_in_frame: [0; MAX_KEYBOARD_KEYS],
			key_press_queue: [0; MAX_KEY_PRESSED_QUEUE],
			key_press_queue_count: 0,
			char_press_queue: [0; MAX_CHAR_PRESSED_QUEUE],
			char_press_queue_count: 0,
		},
		mouse: Mouse{
			offset: Vector2{x: 0.0, y: 0.0},
			scale: Vector2{x: 0.0, y: 0.0},
			current_position: Vector2{x: 0.0, y: 0.0},
			previous_position: Vector2{x: 0.0, y: 0.0},
			cursor: MouseCursor::MouseCursorArrow,
			cursor_hidden: false,
			cursor_on_screen: false,
			current_button_state: [0; MAX_MOUSE_BUTTONS],
			previous_button_state: [0; MAX_MOUSE_BUTTONS],
			current_wheel_move: Vector2{x: 0.0, y: 0.0},
			previous_wheel_move: Vector2{x: 0.0, y: 0.0},
		},
		touch: Touch{},
		gamepad: Gamepad{},
	},
	time: Time{
		current: 0.0,
		previous: 0.0,
		update: 0.0,
		draw: 0.0,
		frame: 0.0,
		target: 0.0,
		base: 0,
		frame_counter: 0,
	},
};


//= Structures & Enumerations


//= Procedures

