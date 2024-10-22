use std::str::FromStr;

use chunk_type::ChunkType;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let chunk = ChunkType::from_str("RUSt").unwrap();
    print!("chunk = {}", chunk);
    Ok(())
}
