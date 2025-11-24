#![allow(dead_code)]
#![warn(clippy::all, clippy::pedantic)]

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
mod constants;

use clap::Parser;
use anyhow::Error;
use args::PngmeArgs;
use commands::{decode, encode, 
    print_chunks, remove};


#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args{
    #[command(subcommand)]
    command: PngmeArgs
}

pub type Result<T> = std::result::Result<T, Error>;

/// # Errors
/// run will return Error struct from anyhow crate when it fails
pub fn run() -> Result<()> {
    let args = Args::parse();

    match args.command{
        PngmeArgs::Encode(args) => encode(args)?,
        PngmeArgs::Decode(args) => decode(args)?,
        PngmeArgs::Remove(args) => remove(args)?,
        PngmeArgs::Print(args) => print_chunks(args)?
    }

    Ok(())
}