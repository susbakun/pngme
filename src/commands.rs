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
    let mut potential_message = false;

    let chunk = if let Some(chunk_type) = args.chunk_type{
        png.chunk_by_type(&chunk_type)
    }else {
        potential_message = true;
        png.auto_chunk_detect()
    };
    
    if let Some(chunk) = chunk {
        let message = chunk.data_as_string().unwrap_or(String::new());

        if potential_message{
            print!("potential hidden message: ");
        }

        println!("{message}");
        return Ok(())
    }

    bail!("couldn't find any hidden message");
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