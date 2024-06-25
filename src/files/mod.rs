

use std::{collections::HashMap, fmt::Display, fs::File, io::Read};


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
impl From<u8> for FileType {
	fn from(value: u8) -> Self {
		match value {
			1 => FileType::Image,
			2 => FileType::Animation,
			3 => FileType::Font,
			4 => FileType::StandardSound,
			5 => FileType::Model,
			_ => FileType::JSON,
		}
	}
}
impl Display for FileType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			FileType::Image			=> write!(f, "Image"),
			FileType::Animation		=> write!(f, "Animation"),
			FileType::Font			=> write!(f, "Font"),
			FileType::StandardSound	=> write!(f, "StandardSound"),
			FileType::Model			=> write!(f, "Model"),
			FileType::JSON			=> write!(f, "JSON"),
		}
	}
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
impl Display for Pointer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Bulk File \"{}\"\n\tType:\t\t{} \n\tPosition:\t{:#08X},\n\tSize:\t\t{},",
			self.name,
			self.file_type,
			self.position,
			self.size,
		)
	}
}

#[derive(Debug)]
pub struct Bulk {
	pub filename: String,
	pub checksum: u64,
	pub table: HashMap<String, Pointer>,
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
		
		let table_mem_count = buf[0] as u32 + ((buf[1] as u32) << 8) + ((buf[2] as u32) << 16) + ((buf[3] as u32) << 24);
		let table_length = table_mem_count * 25;
		for i in 0..table_mem_count {
			let mut ptr = Pointer::default();
			for v in 0..16 {
				let val = buf[(4 + (i * 25) + v) as usize] as char;
				if val != '\0' { ptr.name.push(val) }
			}
			ptr.position += ((buf[(4 + (i * 25) + 16) as usize]) as usize) <<  0;
			ptr.position += ((buf[(4 + (i * 25) + 17) as usize]) as usize) <<  8;
			ptr.position += ((buf[(4 + (i * 25) + 18) as usize]) as usize) << 16;
			ptr.position += ((buf[(4 + (i * 25) + 19) as usize]) as usize) << 24;
			ptr.size	 += ((buf[(4 + (i * 25) + 20) as usize]) as usize) <<  0;
			ptr.size	 += ((buf[(4 + (i * 25) + 21) as usize]) as usize) <<  8;
			ptr.size	 += ((buf[(4 + (i * 25) + 22) as usize]) as usize) << 16;
			ptr.size	 += ((buf[(4 + (i * 25) + 23) as usize]) as usize) << 24;
			ptr.file_type = FileType::from(buf[(4 + (i * 24) + 24) as usize]);
			
			println!("{ptr}");
		}
		
		let mut data = Vec::new();
		for i in buf.bytes() {
			if i.is_ok()  { data.push(i.unwrap()); }
		}
		
		Ok(Self {
			filename: filename.to_string(),
			checksum: 0,
			table: HashMap::new(),
			data,
		})
	}
	/// #### new
	/// Creates a new blank Bulk.
	pub fn new() -> Self {
		Self {
			filename: "".to_string(),
			checksum: 0,
			table: HashMap::new(),
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
	//
	pub fn get(&mut self, name: &str) -> 
	
}

pub fn from_u8(arr: [u8;4]) -> u32 {
	arr[0] as u32 + ((arr[1] as u32) << 8) + ((arr[2] as u32) << 16) + ((arr[3] as u32) << 24)
}