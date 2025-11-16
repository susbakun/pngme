use super::chunk_type::ChunkType;
use super::{Error, Result};
use anyhow::bail;
use crc::{CRC_32_ISO_HDLC, Crc};
use std::fmt::Display;
use std::io::{BufReader, Read};

#[derive(Debug)]
pub struct Chunk{
    data: Vec<u8>,
    length: u32,
    crc: u32,
    chunk_type: ChunkType
}

impl Chunk{
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self{
        let length = data.len() as u32;

        let bytes = [chunk_type.bytes().to_vec(), data.clone()].concat();
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(&bytes);
        Self { data, length, crc, chunk_type }
    }

    pub fn data(&self) -> &[u8]{
        &self.data
    }

    pub fn as_bytes(&self) -> Vec<u8>{
        let mut result = Vec::new();
        result.extend_from_slice(&self.length.to_be_bytes());
        result.extend_from_slice(&self.chunk_type.bytes());
        result.extend_from_slice(&self.data);
        result.extend_from_slice(&self.crc.to_be_bytes());
        
        result
    }

    pub fn data_as_string(&self) -> Result<String>{
        let result = String::from_utf8(self.data.clone())?;
        Ok(result)
    }

    pub fn length(&self) -> u32{
        self.length
    }

    pub fn crc(&self) -> u32{
        self.crc
    }

    pub fn chunk_type(&self) -> &ChunkType{
        &self.chunk_type
    }
}

impl TryFrom<&[u8]> for Chunk{
    type Error = Error;
    fn try_from(bytes: &[u8]) -> std::result::Result<Self, Self::Error> {
        let mut reader = BufReader::new(bytes);
        let mut buffer: [u8;4] = [0, 0, 0, 0];
        
        reader.read_exact(&mut buffer)?;
        let length = u32::from_be_bytes(buffer);
        
        let mut buffer: [u8;4] = [0, 0, 0, 0];
        reader.read_exact(&mut buffer)?;
        let chunk_type = ChunkType::try_from(buffer)?;
        
        let mut buffer = Vec::new();
        (0..length).for_each(|_| buffer.push(0));
        reader.read_exact(&mut buffer)?;
        let data = buffer;

        let mut buffer: [u8;4] = [0, 0, 0, 0];
        reader.read_exact(&mut buffer)?;
        let crc = u32::from_be_bytes(buffer);

        // validating crc
        let bytes = [chunk_type.bytes().to_vec(), data.clone()].concat();
        let valid_crc = Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(&bytes);

        if valid_crc != crc{
            bail!("crc is not valid");
        }

        Ok(Self {
            length,
            data,
            chunk_type,
            crc
        })
    }
}

impl Display for Chunk{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Data {{")?;
        writeln!(f, "Length: {}", self.length())?;
        writeln!(f, "Type: {}", self.chunk_type)?;
        writeln!(f, "Data {} bytes", self.data().len())?;
        writeln!(f, "Crc: {}", self.crc)?;
        write!(f, "}}")
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
        let data = "This is where your secret message will be!".as_bytes().to_vec();
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
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes: &[u8] = "This is where your secret message will be!".as_bytes();
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

