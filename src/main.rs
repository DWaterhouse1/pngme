use std::{fs, str::FromStr};

use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use crate::{
    chunk::Chunk,
    chunk_type::ChunkType,
    commands::{Args, Commands},
    png::Png,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.cmd {
        Commands::Encode {
            path,
            chunk_type,
            message,
            output,
        } => {
            let data: Vec<u8> = fs::read(path.clone()).map_err(|err| {
                format!(
                    "Error reading PNG file at {}: {}",
                    path.to_str().unwrap_or("<Invalid Path>"),
                    err
                )
            })?;

            let mut png = Png::try_from(data.as_slice()).map_err(|err| {
                format!(
                    "Error parsing PNG data for file at {}: {}",
                    path.to_str().unwrap_or("<Invalid Path>"),
                    err
                )
            })?;

            let chunk_type_value = ChunkType::from_str(&chunk_type)
                .map_err(|err| format!("Invalid chunk type {:?}: {}", chunk_type, err))?;

            let chunk = Chunk::new(chunk_type_value, message.into_bytes());

            png.append_chunk(chunk);

            let output_path = output.unwrap_or(path);

            fs::write(output_path, png.as_bytes())?;
        }

        Commands::Decode { path, chunk_type } => {
            let data: Vec<u8> = fs::read(path.clone()).map_err(|err| {
                format!(
                    "Error reading PNG file at {}: {}",
                    path.to_str().unwrap_or("<Invalid Path>"),
                    err
                )
            })?;

            let png = Png::try_from(data.as_slice()).map_err(|err| {
                format!(
                    "Error parsing PNG data for file at {}: {}",
                    path.to_str().unwrap_or("<Invalid Path>"),
                    err
                )
            })?;

            let chunk = png
                .chunk_by_type(&chunk_type)
                .ok_or(format!("Chunk type {:?} not found", chunk_type))?;

            println!(
                "Decoded: {}",
                chunk
                    .data_as_string()
                    .unwrap_or("<Not Representable>".to_string())
            );
        }

        Commands::Remove { path, chunk_type } => {
            let data: Vec<u8> = fs::read(path.clone()).map_err(|err| {
                format!(
                    "Error reading PNG file at {}: {}",
                    path.to_str().unwrap_or("<Invalid Path>"),
                    err
                )
            })?;

            let mut png = Png::try_from(data.as_slice()).map_err(|err| {
                format!(
                    "Error parsing PNG data for file at {}: {}",
                    path.to_str().unwrap_or("<Invalid Path>"),
                    err
                )
            })?;

            png.remove_first_chunk(&chunk_type)
                .map_err(|err| format!("Could not remove chunk type {:?}: {}", chunk_type, err))?;

            fs::write(path, png.as_bytes())?;
        }

        Commands::Print { path } => {
            let data: Vec<u8> = fs::read(path.clone()).map_err(|err| {
                format!(
                    "Error reading PNG file at {}: {}",
                    path.to_str().unwrap_or("<Invalid Path>"),
                    err
                )
            })?;

            let png = Png::try_from(data.as_slice()).map_err(|err| {
                format!(
                    "Error parsing PNG data for file at {}: {}",
                    path.to_str().unwrap_or("<Invalid Path>"),
                    err
                )
            })?;

            println!(
                "{}\n{}",
                path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("<Unknown Filename>"),
                png
            );
        }
    }

    Ok(())
}
