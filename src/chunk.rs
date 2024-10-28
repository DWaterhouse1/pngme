use core::fmt;
use std::string::FromUtf8Error;
use thiserror::Error;

use crc::Crc;

use crate::chunk_type::{ChunkType, ChunkTypeError, CHUNK_TYPE_NUM_BYTES};

pub const CHUNK_LENGTH_NUM_BYTES: usize = 4;
pub const CHUNK_CHECK_NUM_BYTES: usize = 4;
pub const CHUNK_METADATA_NUM_BYTES: usize =
    CHUNK_LENGTH_NUM_BYTES + CHUNK_CHECK_NUM_BYTES + CHUNK_TYPE_NUM_BYTES;

#[derive(Error, Debug)]
pub enum ChunkError {
    #[error(transparent)]
    BadType(#[from] ChunkTypeError),
    #[error("Given {0} bytes are insufficient to form chunk.")]
    InsufficientBytes(usize),
    #[error("Chunk failed checksum, expected {expected} but was given {actual}.")]
    BadChecksum { expected: u32, actual: u32 },
}

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    checksum: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

        let checksum = crc.checksum(&[chunk_type.bytes().as_slice(), data.as_slice()].concat());

        Self {
            length: data.len() as u32,
            chunk_type,
            data,
            checksum,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn crc(&self) -> u32 {
        self.checksum
    }

    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        u32::to_be_bytes(self.length)
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data().iter())
            .chain(u32::to_be_bytes(self.checksum).iter())
            .cloned()
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let value_len = value.len();
        if value_len < CHUNK_METADATA_NUM_BYTES {
            return Err(ChunkError::InsufficientBytes(value_len));
        }

        let (length_slice, remaining_bytes) = value.split_at(CHUNK_LENGTH_NUM_BYTES);
        let (type_slice, remaining_bytes) = remaining_bytes.split_at(CHUNK_TYPE_NUM_BYTES);

        let length = u32::from_be_bytes(
            length_slice
                .try_into()
                .map_err(|_| ChunkError::InsufficientBytes(value_len))?,
        );

        let chunk_type = ChunkType::try_from(
            TryInto::<[u8; 4]>::try_into(type_slice)
                .map_err(|_| ChunkError::InsufficientBytes(value_len))?,
        )?;

        if remaining_bytes.len() < length as usize {
            return Err(ChunkError::InsufficientBytes(value_len));
        }

        let (data, remaining_bytes) = remaining_bytes.split_at(length as usize);
        let (crc_slice, _) = remaining_bytes.split_at(CHUNK_CHECK_NUM_BYTES);

        let chunk = Chunk::new(chunk_type, Vec::from(data));

        let crc = u32::from_be_bytes(
            crc_slice
                .try_into()
                .map_err(|_| ChunkError::InsufficientBytes(value_len))?,
        );

        if chunk.crc() == crc {
            Ok(chunk)
        } else {
            Err(ChunkError::BadChecksum {
                expected: chunk.crc(),
                actual: crc,
            })
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data_repr = match self.data_as_string() {
            Ok(data_str) => data_str,
            Err(_) => "Not String Representable".to_string(),
        };
        write!(
            f,
            "Length: {}\nType: {}\nData: {}\nCRC: {}",
            self.length(),
            self.chunk_type(),
            data_repr,
            self.crc()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    fn test_valid_chunk_as_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_bytes = Vec::from(chunk.as_bytes());

        assert_eq!(chunk_bytes, chunk_data);
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
