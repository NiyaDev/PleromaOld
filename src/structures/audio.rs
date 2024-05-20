

use crate::rl_str;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WaveRl {
	frame_count: u32,
	sample_rate: u32,
	sample_size: u32,
	channels: u32,
	data: *mut std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioStreamRl {
	buffer: *mut AudioBufferRl,
	processor: *mut AudioProcessorRl,
	
	sample_rate: u32,
	sample_size: u32,
	channels: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sound {
	stream: AudioStreamRl,
	frame_count: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Music {
	stream: AudioStreamRl,
	frame_count: u32,
	looping: bool,
	
	ctx_type: i32,
	ctx_data: *mut std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioBufferRl {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioProcessorRl {
    _unused: [u8; 0],
}

impl Sound {
	
	/// #### load
	/// Wrapper for Raylib::LoadSound(filename: *const i8).
	pub fn load(filename: &str) -> Self {
		unsafe{ LoadSound(rl_str!(filename)) }
	}
	
	/// #### play
	/// Wrapper for Raylib::PlaySound(sound: Sound).
	pub fn play(&self) -> &Self {
		unsafe{ PlaySound(*self) }
		
		self 
	}
	/// #### pitch
	/// Wrapper for Raylib::SetSoundPitch(sound: Sound, pitch: f32).
	pub fn pitch(&self, pitch: f32) -> &Self {
		unsafe{ SetSoundPitch(*self, pitch) }
		
		self 
	}
	/// #### pan
	/// Wrapper for Raylib::SetSoundPitch(sound: Sound, pan: f32).
	pub fn pan(&self, pan: f32) -> &Self {
		unsafe{ SetSoundPan(*self, pan) }
		
		self 
	}
	
}

impl Music {
	
	/// #### load
	/// Wrapper for Raylib::LoadMusicStream(filename: *const i8).
	pub fn load(filename: &str) -> Self {
		unsafe{ LoadMusicStream(rl_str!(filename)) }
	}
	/// #### is_ready
	/// Wrapper for Raylib::IsMusicReady(music: Music) -> bool.
	pub fn is_ready(self) -> bool {
		unsafe{ IsMusicReady(self) }
	}
	/// #### is_playing
	/// Wrapper for Raylib::IsMusicStreamPlaying(music: Music) -> bool.
	pub fn is_playing(self) -> bool {
		unsafe{ IsMusicStreamPlaying(self) }
	}
	/// #### unload
	/// Wrapper for Raylib::UnloadMusicStream(music: Music).
	pub fn unload(self) {
		unsafe{ UnloadMusicStream(self) }
	}
	
	/// #### play
	/// Wrapper for Raylib::PlayMusicStream(music: Music).
	pub fn play(&mut self) -> &mut Self {
		unsafe{ PlayMusicStream(*self) }
		
		self
	}
	/// #### stop
	/// Wrapper for Raylib::StopMusicStream(music: Music).
	pub fn stop(&mut self) -> &mut Self {
		unsafe{ StopMusicStream(*self) }
		
		self
	}
	/// #### pause
	/// Wrapper for Raylib::PauseMusicStream(music: Music).
	pub fn pause(&mut self) -> &mut Self {
		unsafe{ PauseMusicStream(*self) }
		
		self
	}
	/// #### resume
	/// Wrapper for Raylib::ResumeMusicStream(music: Music).
	pub fn resume(&mut self) -> &mut Self {
		unsafe{ ResumeMusicStream(*self) }
		
		self
	}
	/// #### update
	/// Wrapper for Raylib::UpdateMusicStream(music: Music).
	pub fn update(&mut self) -> &mut Self {
		unsafe{ UpdateMusicStream(*self) }
		
		self
	}
	/// #### seek
	/// Wrapper for Raylib::SeekMusicStream(music: Music, position: f32).
	pub fn seek(&mut self, position: f32) -> &mut Self {
		unsafe{ SeekMusicStream(*self, position) }
		
		self
	}
	/// #### volume
	/// Wrapper for Raylib::SetMusicVolume(music: Music, volume: f32).
	pub fn volume(&mut self, volume: f32) -> &mut Self {
		unsafe{ SetMusicVolume(*self, volume) }
		
		self
	}
	/// #### pitch
	/// Wrapper for Raylib::SetMusicPitch(music: Music, pitch: f32).
	pub fn pitch(&mut self, pitch: f32) -> &mut Self {
		unsafe{ SetMusicPitch(*self, pitch) }
		
		self
	}
	/// #### pan
	/// Wrapper for Raylib::SetMusicPan(music: Music, pan: f32).
	pub fn pan(&mut self, pan: f32) -> &mut Self {
		unsafe{ SetMusicPan(*self, pan) }
		
		self
	}
	/// #### length
	/// Wrapper for Raylib::SetMusicVolume(music: Music) -> f32.
	pub fn length(&mut self) -> f32 {
		unsafe{ GetMusicTimeLength(*self) }
	}
	/// #### played
	/// Wrapper for Raylib::GetMusicTimePlayed(music: Music) -> f32.
	pub fn played(&mut self) -> f32 {
		unsafe{ GetMusicTimePlayed(*self) }
	}
	
}


extern "C" { fn LoadSound(filename: *const i8) -> Sound; }
extern "C" { fn PlaySound(sound: Sound); }
extern "C" { fn SetSoundPitch(sound: Sound, pitch: f32); }
extern "C" { fn SetSoundPan(sound: Sound, pan: f32); }

extern "C" { fn LoadMusicStream(filename: *const i8) -> Music; }
extern "C" { fn IsMusicReady(music: Music) -> bool; }
extern "C" { fn UnloadMusicStream(music: Music); }
extern "C" { fn PlayMusicStream(music: Music); }
extern "C" { fn IsMusicStreamPlaying(music: Music) -> bool; }
extern "C" { fn UpdateMusicStream(music: Music); }
extern "C" { fn StopMusicStream(music: Music); }
extern "C" { fn PauseMusicStream(music: Music); }
extern "C" { fn ResumeMusicStream(music: Music); }
extern "C" { fn SeekMusicStream(music: Music, position: f32); }
extern "C" { fn SetMusicVolume(music: Music, volume: f32); }
extern "C" { fn SetMusicPitch(music: Music, pitch: f32); }
extern "C" { fn SetMusicPan(music: Music, pan: f32); }
extern "C" { fn GetMusicTimeLength(music: Music) -> f32; }
extern "C" { fn GetMusicTimePlayed(music: Music) -> f32; }
