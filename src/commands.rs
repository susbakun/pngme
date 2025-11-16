use std::fs;
use std::str::FromStr;

use anyhow::{Ok, bail};

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

use super::png::Png;
use super::Result;
use super::args::{EncodeArgs, DecodeArgs, RemoveArgs, PrintArgs};

pub fn encode(args: EncodeArgs) -> Result<()>{
    let mut png = Png::from_file(&args.file_path)?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let message = args.message.as_bytes().to_vec();

    let chunk = Chunk::new(chunk_type, message);
    png.append_chunk(chunk);
    let output_path = args.output_file.unwrap_or(args.file_path);
    fs::write(output_path, png.as_bytes())?;
    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()>{
    let png = Png::from_file(args.file_path)?;
    let chunk = png.chunk_by_type(&args.chunk_type);
    if let Some(chunk) = chunk {
        let message = chunk.data_as_string().unwrap_or("".to_string());
        println!("{message}");
        return Ok(())
    }

    bail!("Failed decoding the message");
}

pub fn remove(args: RemoveArgs) -> Result<()>{
    let mut png = Png::from_file(args.file_path)?;
    png.remove_first_chunk(&args.chunk_type)?;
    Ok(())
}

pub fn print_chunks(args: PrintArgs) -> Result<()>{
    let png = Png::from_file(args.file_path)?;
    println!("{png}");
    Ok(())
}