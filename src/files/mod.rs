

pub mod compression;
use self::compression::{CompressionLevel, CompressionType};
pub mod loading;


/// Bulk
/// 
/// The File itself that stores all of the resources.
pub struct Bulk {
	pub version: [u8;4],

	pub paks: Vec<Pak>,
}

/// Pak
/// 
/// The compressed version of a singular resource.
pub struct Pak {
	pub compression_type: CompressionType,
	pub compression_level: CompressionLevel,

	pub data: Vec<u8>,
}