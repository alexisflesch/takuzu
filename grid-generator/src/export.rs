use crate::grid::Puzzle;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub fn save_puzzles(puzzles: &[Puzzle], path: &Path) -> std::io::Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, puzzles)?;
    Ok(())
}