

use crate::rl_str;


/// #### Sound
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sound {
	stream: AudioStream,
	frame_count: u32,
}

impl Sound {
	
	/// #### load
	/// Wrapper for Raylib::LoadSound(filename: *const i8).
	pub fn load(filename: &str) -> Self {
		unsafe{ LoadSound(rl_str!(filename)) }
	}
	/// #### unload
	/// Wrapper for Raylib::UnloadSound(sound: Sound).
	pub fn unload(&mut self) {
		unsafe{ UnloadSound(*self) }
	}
	/// #### ready
	/// Wrapper for Raylib::IsSoundReady(sound: Sound) -> bool.
	pub fn ready(&self) -> bool {
		unsafe{ IsSoundReady(*self) }
	}
	/// #### playing
	/// Wrapper for Raylib::IsSoundPlaying(sound: Sound) -> bool.
	pub fn playing(&self) -> bool {
		unsafe{ IsSoundPlaying(*self) }
	}
	
	/// #### play
	/// Wrapper for Raylib::PlaySound(sound: Sound).
	pub fn play(&self) -> &Self {
		unsafe{ PlaySound(*self) }
		
		self 
	}
	/// #### stop
	/// Wrapper for Raylib::StopSound(sound: Sound).
	pub fn stop(&mut self) -> &mut Self {
		unsafe{ StopSound(*self) }
		
		self
	}
	/// #### pause
	/// Wrapper for Raylib::StopSound(sound: Sound).
	pub fn pause(&mut self) -> &mut Self {
		unsafe{ PauseSound(*self) }
		
		self
	}
	/// #### resume
	/// Wrapper for Raylib::ResumeSound(sound: Sound).
	pub fn resume(&mut self) -> &mut Self {
		unsafe{ ResumeSound(*self) }
		
		self
	}
	/// #### volume
	/// Wrapper for Raylib::SetSoundVolume(sound: Sound, volume: f32).
	pub fn volume(&mut self, volume: f32) -> &mut Self {
		unsafe{ SetSoundVolume(*self, volume) }
		
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


/// #### Music
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Music {
	stream: AudioStream,
	frame_count: u32,
	looping: bool,
	
	ctx_type: i32,
	ctx_data: *mut std::os::raw::c_void,
}

impl Music {
	
	/// #### load
	/// Wrapper for Raylib::LoadMusicStream(filename: *const i8).
	pub fn load(filename: &str) -> Self {
		unsafe{ LoadMusicStream(rl_str!(filename)) }
	}
	/// #### is_ready
	/// Wrapper for Raylib::IsMusicReady(music: Music) -> bool.
	pub fn ready(&self) -> bool {
		unsafe{ IsMusicReady(*self) }
	}
	/// #### is_playing
	/// Wrapper for Raylib::IsMusicStreamPlaying(music: Music) -> bool.
	pub fn playing(&self) -> bool {
		unsafe{ IsMusicStreamPlaying(*self) }
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


/// #### Wave
/// 
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Wave {
	frame_count: u32,
	sample_rate: u32,
	sample_size: u32,
	channels: u32,
	data: *mut std::os::raw::c_void,
}

impl Wave {
	
	/// #### load
	/// Wrapper for Raylib::LoadWave(filename: *const i8) -> Wave.
	pub fn load(filename: &str) -> Self {
		unsafe{ LoadWave(rl_str!(filename)) }
	}
	/// #### unload
	/// Wrapper for Raylib::UnloadWave(wave: Wave).
	pub fn unload(&mut self) {
		unsafe{ UnloadWave(*self) }
	}
	/// #### ready
	/// Wrapper for Raylib::IsWaveReady(filename: *const i8) -> Wave.
	pub fn ready(&self) -> bool {
		unsafe{ IsWaveReady(*self) }
	}
	/// #### ready
	/// Wrapper for Raylib::LoadSoundFromWave(wave: Wave) -> Sound.
	pub fn to_sound(&mut self) -> Sound {
		unsafe{ LoadSoundFromWave(*self) }
	}
	/// #### export
	/// Wrapper for Raylib::ExportWave(wave: Wave, filename: *const i8).
	pub fn export(&mut self, filename: &str) -> &mut Self {
		unsafe{ ExportWave(*self, rl_str!(filename)) }
		
		self
	}
	
	/// #### copy
	/// Wrapper for Raylib::WaveCopy(wave: Wave) -> Wave.
	pub fn copy(&mut self) -> Wave {
		unsafe{ WaveCopy(*self) }
	}
	/// #### crop
	/// Wrapper for Raylib::WaveCrop(wave: *mut Wave, init_sample: i32, final_sample: i32).
	pub fn crop(&mut self, init_sample: i32, final_sample: i32) {
		unsafe{ WaveCrop(self, init_sample, final_sample) }
	}
	/// #### format
	/// Wrapper for Raylib::ExportWave(wave: Wave,filename: *const i8).
	pub fn format(&mut self, sample_rate: i32, sample_size: i32, channels: i32) {
		unsafe{ WaveFormat(self, sample_rate, sample_size, channels) }
	}
	
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AudioStream {
	buffer: *mut AudioBufferRl,
	processor: *mut AudioProcessorRl,
	
	sample_rate: u32,
	sample_size: u32,
	channels: u32,
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


extern "C" { fn LoadWave(filename: *const i8) -> Wave; }
extern "C" { fn IsWaveReady(wave: Wave) -> bool; }
extern "C" { fn LoadSound(filename: *const i8) -> Sound; }
extern "C" { fn LoadSoundFromWave(wave: Wave) -> Sound; }
extern "C" { fn IsSoundReady(sound: Sound) -> bool; }
extern "C" { fn UnloadWave(wave: Wave); }
extern "C" { fn UnloadSound(sound: Sound); }
extern "C" { fn ExportWave(wave: Wave, filename: *const i8); }

extern "C" { fn PlaySound(sound: Sound); }
extern "C" { fn StopSound(sound: Sound); }
extern "C" { fn PauseSound(sound: Sound); }
extern "C" { fn ResumeSound(sound: Sound); }
extern "C" { fn IsSoundPlaying(sound: Sound) -> bool; }
extern "C" { fn SetSoundVolume(sound: Sound, volume: f32); }
extern "C" { fn SetSoundPitch(sound: Sound, pitch: f32); }
extern "C" { fn SetSoundPan(sound: Sound, pan: f32); }
extern "C" { fn WaveCopy(wave: Wave) -> Wave; }
extern "C" { fn WaveCrop(wave: *mut Wave, init_sample: i32, final_sample: i32); }
extern "C" { fn WaveFormat(wave: *mut Wave, sample_rate: i32, sample_size: i32, channels: i32); }

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
