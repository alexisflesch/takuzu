use crate::grid::Puzzle;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs::{read_dir, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Serialize, serde::Deserialize)]
struct AngularPuzzle {
    grid: Vec<Vec<i8>>,
    solution: Vec<Vec<i8>>,
    id: String,
}

pub fn save_puzzles(puzzles: &[Puzzle], path: &Path) -> std::io::Result<()> {
    // Read existing puzzles if present to continue IDs
    let mut existing: Vec<AngularPuzzle> = if path.exists() {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        match serde_json::from_reader(reader) {
            Ok(v) => v,
            Err(_) => Vec::new(),
        }
    } else {
        Vec::new()
    };

    let start_idx = existing.len();

    let mut new_angular: Vec<AngularPuzzle> = puzzles
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
            id: format!("grid_{}", start_idx + i + 1),
        })
        .collect();

    existing.append(&mut new_angular);

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &existing)?;
    Ok(())
}

#[derive(Serialize)]
struct DifficultyCounts {
    d1: usize,
    d2: usize,
    d3: usize,
    d4: usize,
    d5: usize,
}

#[derive(Serialize)]
struct Summary {
    sizes: BTreeMap<String, DifficultyCounts>,
}

pub fn update_summary(output_dir: &Path) -> std::io::Result<()> {
    let mut sizes: BTreeMap<String, DifficultyCounts> = BTreeMap::new();

    for entry in read_dir(output_dir)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            // Expecting names like takuzu_8x8_d2.json
            if !name.starts_with("takuzu_") || !name.ends_with(".json") {
                continue;
            }
            let parts: Vec<&str> = name.trim_end_matches(".json").split('_').collect();
            if parts.len() != 3 {
                continue;
            }
            let size_part = parts[1]; // e.g. 8x8
            let diff_part = parts[2]; // e.g. d2

            let difficulty = diff_part.trim_start_matches('d');
            let key = size_part.to_string();

            // Read the file and count entries
            let f = File::open(&path)?;
            let reader = BufReader::new(f);
            let value: Value = match serde_json::from_reader(reader) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let count = match value {
                Value::Array(arr) => arr.len(),
                _ => 0,
            };

            let entry = sizes.entry(key).or_insert(DifficultyCounts {
                d1: 0,
                d2: 0,
                d3: 0,
                d4: 0,
                d5: 0,
            });

            match difficulty {
                "1" => entry.d1 = count,
                "2" => entry.d2 = count,
                "3" => entry.d3 = count,
                "4" => entry.d4 = count,
                "5" => entry.d5 = count,
                _ => (),
            }
        }
    }

    let summary = Summary { sizes };
    let out = output_dir.join("grids_summary.json");
    let f = File::create(out)?;
    let writer = BufWriter::new(f);
    serde_json::to_writer_pretty(writer, &summary)?;
    Ok(())
}