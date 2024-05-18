

use crate::structures::audio::*;
	

pub struct AudioHandler {
	music: [Music; 5],
}

pub struct Music {
	instruments: Vec<Sound>,
	track: Vec<(Note, u32)>,
	timer: u32,
	length: u32,
	track_position: u32,
	
}

pub struct Note {
	id: u32,
	duration: u32,
	pitch: f32,
	pan: f32,
}