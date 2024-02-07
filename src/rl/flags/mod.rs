

//= Allows



//= Imports


//= Constants


//= Structures & Enumerations

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Flag(u32);


pub enum WindowFlags {
	FullscreenMode		= 0x00000002,	// Set to run program in fullscreen
	Resizable			= 0x00000004,	// Set to allow resizable window
	Undecorated			= 0x00000008,	// Set to disable window decoration (frame and buttons)
	Transparent			= 0x00000010,	// Set to allow transparent framebuffer
	MSAA4xHint			= 0x00000020,	// Set to try enabling MSAA 4X
	VSyncHint			= 0x00000040,	// Set to try enabling V-Sync on GPU
	Hidden				= 0x00000080,	// Set to hide window
	AlwaysRun			= 0x00000100,	// Set to allow windows running while minimized
	Minimized			= 0x00000200,	// Set to minimize window (iconify)
	Maximized			= 0x00000400,	// Set to maximize window (expanded to monitor)
	Unfocused			= 0x00000800,	// Set to window non focused
	TopMost				= 0x00001000,	// Set to window always on top
	HighDPI				= 0x00002000,	// Set to support HighDPI
	MousePassthrough	= 0x00004000,	// Set to support mouse passthrough, only supported when FLAG_WINDOW_UNDECORATED
	BorderlessWindow	= 0x00008000,	// Set to run program in borderless windowed mode
	Interlaced			= 0x00010000,	// Set to try enabling interlaced video format (for V3D)
}
impl Into<u32> for WindowFlags {
	fn into(self) -> u32 {
		self as u32
	}
}


//= Procedures

impl Flag {

	/// Creates new empty container
	pub const fn new() -> Self {
		Self(0)
	}

	/// Checks if the container contains input flag
	pub fn contains(&self, flag: u32) -> bool {
		(self.0 & flag) > 0
	}

	/// Sets the inputed flag
	pub fn set(&mut self, value: u32) {
		self.0 &= value;
	}

	/// Clears the inputed flag
	pub fn clear(&mut self, value: u32) {
		self.0 &= !value;
	}

}