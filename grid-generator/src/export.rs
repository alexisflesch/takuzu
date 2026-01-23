use crate::grid::Puzzle;
use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[derive(Serialize)]
struct AngularPuzzle {
    grid: Vec<Vec<i8>>,
    solution: Vec<Vec<i8>>,
    id: String,
}

pub fn save_puzzles(puzzles: &[Puzzle], path: &Path) -> std::io::Result<()> {
    let angular_puzzles: Vec<AngularPuzzle> = puzzles
        .iter()
        .enumerate()
        .map(|(i, p)| AngularPuzzle {
            grid: p
                .puzzle
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|&cell| match cell {
                            Some(0) => 0,
                            Some(1) => 1,
                            None => -1,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect(),
            solution: p
                .solution
                .iter()
                .map(|row| row.iter().map(|&x| x as i8).collect())
                .collect(),
            id: format!("grid_{}", i + 1),
        })
        .collect();

    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &angular_puzzles)?;
    Ok(())
}