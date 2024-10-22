use snafu::prelude::*;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Snafu)]
pub enum ChunkTypeError {
    #[snafu(display("Chunk types must be ASCII alphabetic bytes."))]
    NonAlphabetic,
    #[snafu(display("Chunk types are four bytes exactly."))]
    InvalidLength,
}

pub const CHUNK_TYPE_NUM_BYTES: usize = 4;
type ChunkBytes = [u8; CHUNK_TYPE_NUM_BYTES];

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    data: ChunkBytes,
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.data
    }

    fn are_valid_bytes(bytes: [u8; 4]) -> bool {
        bytes
            .iter()
            .filter(|byte| byte.is_ascii_alphabetic())
            .count()
            == bytes.len()
    }

    fn is_bit_five_high(byte: u8) -> bool {
        (byte & 0x20) != 0
    }

    pub fn is_valid(&self) -> bool {
        ChunkType::are_valid_bytes(self.data) && Self::is_reserved_bit_valid(&self)
    }

    pub fn is_critical(&self) -> bool {
        !Self::is_bit_five_high(self.data[0])
    }

    pub fn is_public(&self) -> bool {
        !Self::is_bit_five_high(self.data[1])
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        !Self::is_bit_five_high(self.data[2])
    }

    pub fn is_safe_to_copy(&self) -> bool {
        Self::is_bit_five_high(self.data[3])
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if ChunkType::are_valid_bytes(value) {
            Ok(Self { data: value })
        } else {
            Err(ChunkTypeError::NonAlphabetic)
        }
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: ChunkBytes = match Vec::from(s).try_into() {
            Ok(value) => value,
            Err(_) => return Err(ChunkTypeError::InvalidLength),
        };

        ChunkType::try_from(bytes)
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.data).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
