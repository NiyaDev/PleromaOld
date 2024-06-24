

use std::{fs::File, io::Read};

use chrono::offset;


#[derive(Debug, Default, Clone)]
pub enum FileType {
	Image,
	Animation,
	Font,
	StandardSound,
	Model,
	#[default]
	JSON,
}

#[derive(Debug, Default, Clone)]
pub enum FileCompression {
	#[default]
	None,
	Base64,
}

#[derive(Debug, Default, Clone)]
pub struct Pointer {
	pub name: String,
	pub position: usize,
	pub size: usize,
	pub file_type: FileType,
}

#[derive(Debug)]
pub struct Bulk {
	pub filename: String,
	pub checksum: u64,
	pub table: Vec<Pointer>,
	pub data: Vec<u8>,
}
impl Bulk {
	
	/// #### load
	/// Loads Bulk from file.
	pub fn load(filename: &str) -> Result<Self, ()> {
		let file = File::open(filename);
		if file.is_err() {
			//* ERROR
			return Err(());
		}
		
		let mut buf = Vec::new();
		let result = file.unwrap().read_to_end(&mut buf);
		if result.is_err() {
			//* ERROR
			return Err(());
		}
		
		let table_length = buf[0] as u32 + ((buf[1] as u32) << 8) + ((buf[2] as u32) << 16) + ((buf[3] as u32) << 24);
		for i in 0..table_length {
			let ptr = Pointer::default();
			let mut ctr = 4;
			while  {
				
			}
			
			println!("{ptr:?}");
		}
		
		let mut data = Vec::new();
		for i in buf.bytes() {
			if i.is_ok() { data.push(i.unwrap()); }
		}
		
		Ok(Self {
			filename: filename.to_string(),
			checksum: 0,
			table: Vec::new(),
			data,
		})
	}
	/// #### new
	/// Creates a new blank Bulk.
	pub fn new() -> Self {
		Self {
			filename: "".to_string(),
			checksum: 0,
			table: Vec::new(),
			data: Vec::new(),
		}
	}
	/// #### save
	/// Saves bulk to file.
	pub fn save(&mut self) {}
	///// #### set_filename
	///// Sets the filename to be saved as.
	//pub fn set_filename(&mut self, filename: &str) -> &mut Self {}
	///// #### calc_checksum
	///// Calculates the checksum for the data.
	//pub fn calc_checksum(&mut self) -> &mut Self {}
	
}