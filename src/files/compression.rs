

use std::io::prelude::*;
use flate2::write::{DeflateEncoder, GzEncoder, ZlibEncoder};
use flate2::read::{DeflateDecoder, GzDecoder, ZlibDecoder};
use flate2::Compression;


/// Compression types
///
/// Decides what to do with the raw data.
pub enum CompressionType {
	None,
	ZLib,
	Deflate,
	GZ,
}

/// Compression level
/// 
/// How long you're willing to wait.
pub enum CompressionLevel {
	Best,
	Fast,
	None,
	Custom(u32),
}
impl Into<Compression> for CompressionLevel {
	fn into(self) -> Compression {
		match self {
			CompressionLevel::Best => Compression::best(),
			CompressionLevel::Fast => Compression::fast(),
			CompressionLevel::None => Compression::none(),
			CompressionLevel::Custom(level) => Compression::new(level),
		}
	}
}

impl CompressionType {

	/// Decompress the data
	pub fn decompress(&self, data: Vec<u8>) -> Vec<u8> {
		match self {
			Self::ZLib => {
				let mut decoder = ZlibDecoder::new(data.as_slice());
				let mut result: Vec<u8> = Vec::new();
				let _ = decoder.read_to_end(&mut result);

				result
			}
			Self::GZ => {
				let mut decoder = GzDecoder::new(data.as_slice());
				let mut result = Vec::new();
				let _ = decoder.read_to_end(&mut result);

				result
			}
			Self::Deflate => {
				let mut decoder = DeflateDecoder::new(data.as_slice());
				let mut result = Vec::new();
				let _ = decoder.read_to_end(&mut result);

				result
			}
			Self::None => { data }
		}
	}
	/// Compress the data
	pub fn compress(&self, data: Vec<u8>, compression: CompressionLevel) -> Vec<u8> {
		match self {
			Self::ZLib => {
				let mut encoder = ZlibEncoder::new(Vec::new(), compression.into());
				encoder.write_all(&data).unwrap();
				encoder.finish().unwrap()
			}
			Self::GZ => {
				let mut encoder = GzEncoder::new(Vec::new(), compression.into());
				encoder.write_all(&data).unwrap();
				encoder.finish().unwrap()
			}
			Self::Deflate => {
				let mut encoder = DeflateEncoder::new(Vec::new(), compression.into());
				encoder.write_all(&data).unwrap();
				encoder.finish().unwrap()
			}
			Self::None => { data }
		}
	}
}