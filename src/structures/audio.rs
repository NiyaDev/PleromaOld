

pub struct WaveRl {
	frame_count: u32,
	sample_rate: u32,
	sample_size: u32,
	channels: u32,
	data: *mut std::os::raw::c_void,
}

pub struct AudioStreamRl {
	buffer: *mut AudioBufferRl,
	buffer: *mut AudioProcessorRl,
	
	sample_rate: u32,
	sample_size: u32,
	channels: u32,
}

pub struct SoundRl {
	stream: AudioStreamRl,
	frame_count: u32,
}

pub struct MusicRl {
	stream: AudioStreamRl,
	frame_count: u32,
	looping: bool,
	
	ctx_type: i32,
	ctx_data: *mut std::os::raw::c_void,
}

pub struct AudioBufferRl {
	converter: MaDataConverter,
	callback: AudioCallback,
	
	volume: f32,
	pitch: f32,
	pan: f32,
	
	playing: bool,
	paused: bool,
	looping: bool,
	usage: i32,
	
	is_sub_buffer_processed: [bool; 2],
	size_in_frames: u32,
	frame_cursor_position: u32,
	frames_processed: u32,
	
	data: *const i8,
	
	next: *mut AudioBufferRl,
	prev: *mut AudioBufferRl,
}

pub struct AudioProcessorRl {
	process: AudioCallback,
	next: *mut AudioBufferRl,
	prev: *mut AudioBufferRl,
}