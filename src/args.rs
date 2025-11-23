use std::path::PathBuf;
use clap::{Subcommand, Args};

#[derive(Subcommand, Debug)]
pub enum PngmeArgs{
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs)
}

#[derive(Debug, Args)]
pub struct EncodeArgs{
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<PathBuf>
}

#[derive(Debug, Args)]
pub struct DecodeArgs{
    pub file_path: PathBuf,
    pub chunk_type: Option<String>
}

#[derive(Debug, Args)]
pub struct RemoveArgs{
    pub file_path: PathBuf,
    pub chunk_type: String
}

#[derive(Debug, Args)]
pub struct PrintArgs{
    pub file_path: PathBuf
}