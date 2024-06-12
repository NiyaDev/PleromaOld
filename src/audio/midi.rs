

use crate::sound::*;


#[derive(Debug, Clone, Default)]
pub enum AudioMode {
	#[default]
	Normal,
	Midi,
}

/// #### Music
/// Handles each individual Midi track.
#[derive(Debug, Clone, Default)]
pub struct Midi {
	pub is_on: bool,
	pub looping: bool,
	pub bpm: u32,
	pub instruments: Vec<Sound>,
	pub track: Vec<(Note, u32)>,
	pub timer: u32,
	pub end: u32,
	pub track_position: usize,
	pub length: usize,
}

/// #### Note
/// Handles each Midi note.
#[derive(Debug, Clone, Default)]
pub struct Note {
	pub id: u32,
	pub duration: u32,
	pub pitch: f32,
	pub pan: f32,
}
