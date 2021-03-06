use std::fmt;
use hex::FromHex;
use ser::{deserialize, serialize};
use crypto::dhash256;
use compact::Compact;
use hash::H256;
use primitives::bytes::Bytes;
use solution::EquihashSolution;
use ser::Stream;

#[derive(PartialEq, Clone, Serializable, Deserializable)]
pub struct BlockHeader {
	pub version: u32,
	pub previous_header_hash: H256,
	pub merkle_root_hash: H256,
	pub reserved_hash: H256,
	pub time: u32,
	pub bits: Compact,
	pub nonce: H256,
	pub solution: EquihashSolution,
}

impl BlockHeader {
	pub fn hash(&self) -> H256 {
		dhash256(&serialize(self))
	}

	pub fn equihash_input(&self) -> Bytes {
		let mut stream = Stream::new();
		stream.append(&self.version)
			.append(&self.previous_header_hash)
			.append(&self.merkle_root_hash)
			.append(&self.reserved_hash)
			.append(&self.time)
			.append(&self.bits)
			.append(&self.nonce);
		stream.out()
	}
}

impl fmt::Debug for BlockHeader {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("BlockHeader")
			.field("version", &self.version)
			.field("previous_header_hash", &self.previous_header_hash.reversed())
			.field("merkle_root_hash", &self.merkle_root_hash.reversed())
			.field("time", &self.time)
			.field("bits", &self.bits)
			.field("nonce", &self.nonce)
			.field("equihash_solution", &self.solution)
			.finish()
	}
}

impl From<&'static str> for BlockHeader {
	fn from(s: &'static str) -> Self {
		deserialize(&s.from_hex::<Vec<u8>>().unwrap() as &[u8]).unwrap()
	}
}

#[cfg(test)]
mod tests {
	use ser::{Reader, Error as ReaderError, Stream};
	use solution::SOLUTION_SIZE;
	use super::BlockHeader;

	fn test_block_buffer() -> Vec<u8> {
		let mut buffer = vec![
			1, 0, 0, 0,
			2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
			3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			4, 0, 0, 0,
			5, 0, 0, 0,
			6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			253, 64, 5,
		];
		buffer.extend_from_slice(&[0u8; SOLUTION_SIZE]);
		buffer
	}

	#[test]
	fn test_block_header_stream() {
		let block_header = BlockHeader {
			version: 1,
			previous_header_hash: [2; 32].into(),
			merkle_root_hash: [3; 32].into(),
			reserved_hash: Default::default(),
			time: 4,
			bits: 5.into(),
			nonce: 6.into(),
			solution: Default::default(),
		};

		let mut stream = Stream::new();
		stream.append(&block_header);

		assert_eq!(stream.out(), test_block_buffer().into());
	}

	#[test]
	fn test_block_header_reader() {
		let buffer = test_block_buffer();
		let mut reader = Reader::new(&buffer);

		let expected = BlockHeader {
			version: 1,
			previous_header_hash: [2; 32].into(),
			merkle_root_hash: [3; 32].into(),
			reserved_hash: Default::default(),
			time: 4,
			bits: 5.into(),
			nonce: 6.into(),
			solution: Default::default(),
		};

		assert_eq!(expected, reader.read().unwrap());
		assert_eq!(ReaderError::UnexpectedEnd, reader.read::<BlockHeader>().unwrap_err());
	}
}
