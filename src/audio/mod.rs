

use std::{collections::HashMap, fs::File, io::Read};

use crate::sound::*;
use self::midi::*;

pub mod midi;


/// #### AudioHandler
/// Handles audio system.
#[derive(Debug, Clone)]
pub struct AudioHandler {
	mode: AudioMode,
	
	master_volume: f32,
	music_volume: f32,
	sfx_volume: f32,
	
	//* Normal */
	music: Option<Music>,
	
	//* Midi */
	tracks: [Midi; 5],
	
	//* Sfx */
	sfx: HashMap<String, Sound>,
}
impl Default for AudioHandler {
	fn default() -> Self {
		unsafe{ InitAudioDevice() }
		
		Self {
			mode: Default::default(),
			master_volume:	0.8,
			music_volume:	0.6,
			sfx_volume:		0.8,
			music: None,
			tracks: Default::default(),
			sfx: HashMap::new(),
		}
	}
}

impl AudioHandler {
	
	/// #### close
	/// Turns off raylib audio interface.
	pub fn close(&mut self) {
		unsafe{ CloseAudioDevice() }
	}
	
	/// #### load_midi
	/// Load Midi tracks from file into memory.
	pub fn load_midi(&mut self, track_name: &str, track_number: usize) -> &mut Self {
		let file = File::open(track_name);
		if file.is_err() { return self; }
		
		let dat: &mut String = &mut "".to_string();
		let _ = file.unwrap().read_to_string(dat);
		let dat_value: serde_json::Value = serde_json::from_str(dat).expect("Failed to convert file.");
		
		let track_data = dat_value.as_object().unwrap()["data"].as_array().unwrap();
		let length = track_data.len();
		
		let mut instruments: Vec<Sound> = Vec::new();
		for i in dat_value.as_object().unwrap()["instruments"].as_array().unwrap() {
			let value = i.as_str().unwrap();
			instruments.push(Sound::load(value));
		}
		
		let mut track: Vec<(Note, u32)> = Vec::new();
		for i in track_data {
			let array = i.as_array().unwrap();
			let new: (Note, u32) = (
				Note{
					id:			array[0].as_array().unwrap()[0].as_i64().unwrap() as u32,
					duration:	array[0].as_array().unwrap()[1].as_i64().unwrap() as u32,
					pitch:		array[0].as_array().unwrap()[2].as_f64().unwrap() as f32,
					pan:		array[0].as_array().unwrap()[3].as_f64().unwrap() as f32,
				},
				array[1].as_i64().unwrap() as u32,
			);
			track.push(new);
		}
		let end = track_data[track_data.len()-1].as_array().unwrap()[1].as_i64().unwrap() as u32 + 1;
		
		self.tracks[track_number] = Midi {
			is_on: true,
			looping: dat_value.as_object().unwrap()["looping"].as_bool().unwrap(),
			bpm: dat_value.as_object().unwrap()["bpm"].as_i64().unwrap() as u32,
			instruments,
			track,
			timer: 0,
			end,
			track_position: 0,
			length,
		};
		
		self
	}
	/// #### load_song
	/// Load song from file into memory.
	pub fn load_song(&mut self, filename: &str) -> &mut Self {
		if self.music.is_some() {
			if self.music.unwrap().playing() { self.music.unwrap().stop(); }
			self.music.unwrap().unload();
		}
		
		self.music = Some(Music::load(filename));
		self.music.unwrap()
			.volume(self.master_volume * self.music_volume)
			.play();
		
		self
	}
	/// #### load_sfx
	/// Loads sfx into hashmap for future use.
	pub fn load_sfx(&mut self, filename: &str, name: &str) -> &mut Self {
		let mut sound = Sound::load(filename);
		sound.volume(self.master_volume * self.sfx_volume);
		self.sfx.insert(name.to_string(), sound);
		
		self
	}
	
	/// #### update
	/// Update music process.
	pub fn update(&mut self) -> &mut Self {
		match self.mode {
			AudioMode::Normal => {
				if self.music.is_none() { return self; }
				
				self.music.unwrap().update();
			}
			AudioMode::Midi => {
				for i in self.tracks.as_mut().into_iter() {
					if !i.is_on { continue }
					
					i.timer += 1;
					if i.timer > i.end * 8 {
						i.timer = 0;
						i.track_position = 0;
					}
					
					loop {
						let result = i.track.get(i.track_position);
						if result.is_none() { break }
						if i.track[i.track_position].1 == i.timer / 8  {
							i.instruments[i.track[i.track_position].0.id as usize]
								.pitch(i.track[i.track_position].0.pitch)
								.pan(i.track[i.track_position].0.pan)
								.play();
							i.track_position += 1;
						} else { break }
					}
				}
			}
		}
		
		self
	}
	/// #### set_master_volume
	/// 
	pub fn set_master_volume(&mut self, volume: f32) -> &mut Self {
		self.master_volume = volume;
		
		self.update_volume()
	}
	/// #### set_music_volume
	/// 
	pub fn set_music_volume(&mut self, volume: f32) -> &mut Self {
		self.music_volume = volume;
		
		self.update_volume()
	}
	/// #### set_sfx_volume
	/// 
	pub fn set_sfx_volume(&mut self, volume: f32) -> &mut Self {
		self.sfx_volume = volume;
		
		self.update_volume()
	}
	/// #### update_volume
	/// Applies changes to the volumes to the music/sfx.
	fn update_volume(&mut self) -> &mut Self {
		match self.mode {
			AudioMode::Normal => {
				if self.music.is_none() { return self; }
				
				self.music.unwrap().volume(self.master_volume * self.music_volume);
			}
			AudioMode::Midi => {
				// TODO
			}
		}
		
		// TODO SFX
		
		self
	}
	/// #### play_sfx
	/// Plays the input sfx if it exists in memory.
	pub fn play_sfx(&mut self, name: &str) -> &mut Self {
		let result = self.sfx.get(name);
		if result.is_none() { return self; }
		
		result.unwrap().play();
		
		self
	}
	
}



extern "C" { fn InitAudioDevice(); }
extern "C" { fn CloseAudioDevice(); }
