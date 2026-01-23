use crate::digger::Digger;
use crate::grid::{Grid, Puzzle};
use rand::seq::SliceRandom;
use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

pub fn generate_puzzles(size: usize, count: usize, difficulty: u8) -> Vec<Puzzle> {
    let valid_lines = precompute_valid_lines(size);
    let start = Instant::now();

    let mut puzzles: Vec<Puzzle> = Vec::with_capacity(count);
    
    while puzzles.len() < count {
        let needed = count - puzzles.len();
        let batch_size = (needed * 2).max(num_cpus::get());
        
        let batch: Vec<Puzzle> = (0..batch_size)
            .into_par_iter()
            .filter_map(|_| {
                let mut rng = rand::thread_rng();
                let digger = Digger::new(difficulty.into());

                let complete_grid = generate_complete_grid_with_limit(size, &valid_lines, &mut rng, 50_000)?;
                let puzzle_grid = digger.dig(&complete_grid)?;

                Some(puzzle_grid.to_puzzle(&complete_grid))
            })
            .collect();

        for puzzle in batch {
            if puzzles.len() < count {
                puzzles.push(puzzle);
                println!("Generated {}/{} puzzles ({:.1?})", puzzles.len(), count, start.elapsed());
            }
        }
    }

    puzzles
}

fn generate_complete_grid_with_limit(
    size: usize, 
    valid_lines: &[Vec<bool>], 
    rng: &mut impl Rng,
    max_iterations: usize,
) -> Option<Grid> {
    let mut grid = Grid::new(size);
    let mut iterations = 0;
    
    if fill_grid_recursive_limited(&mut grid, 0, valid_lines, rng, &mut iterations, max_iterations) {
        Some(grid)
    } else {
        None
    }
}

fn fill_grid_recursive_limited(
    grid: &mut Grid,
    row: usize,
    valid_lines: &[Vec<bool>],
    rng: &mut impl Rng,
    iterations: &mut usize,
    max_iterations: usize,
) -> bool {
    *iterations += 1;
    
    if *iterations >= max_iterations {
        return false;
    }

    let size = grid.size;

    if row == size {
        return verify_column_uniqueness(grid);
    }

    let mut candidates: Vec<&Vec<bool>> = valid_lines
        .iter()
        .filter(|line| is_line_compatible(grid, row, line))
        .collect();

    candidates.shuffle(rng);

    for line in candidates {
        for (col, &val) in line.iter().enumerate() {
            grid.set(row, col, Some(val));
        }

        if fill_grid_recursive_limited(grid, row + 1, valid_lines, rng, iterations, max_iterations) {
            return true;
        }

        if *iterations >= max_iterations {
            return false;
        }

        for col in 0..size {
            grid.set(row, col, None);
        }
    }

    false
}

fn precompute_valid_lines(size: usize) -> Vec<Vec<bool>> {
    let max = size / 2;
    let mut valid = Vec::new();

    for mask in 0..(1u32 << size) {
        let line: Vec<bool> = (0..size).map(|i| (mask >> i) & 1 == 1).collect();

        let ones = line.iter().filter(|&&b| b).count();
        if ones != max {
            continue;
        }

        let mut has_triple = false;
        for i in 0..size.saturating_sub(2) {
            if line[i] == line[i + 1] && line[i + 1] == line[i + 2] {
                has_triple = true;
                break;
            }
        }

        if !has_triple {
            valid.push(line);
        }
    }

    valid
}

fn is_line_compatible(grid: &Grid, row: usize, line: &[bool]) -> bool {
    let size = grid.size;
    let max = size / 2;

    for col in 0..size {
        let val = line[col];

        if row >= 2 {
            let above1 = grid.get(row - 1, col);
            let above2 = grid.get(row - 2, col);
            if above1 == Some(val) && above2 == Some(val) {
                return false;
            }
        }

        let col_cells = grid.col(col);
        let count = Grid::count_in_line(&col_cells, val);
        if count >= max {
            return false;
        }
    }

    let existing_sigs = grid.complete_row_signatures();
    let new_sig = line
        .iter()
        .enumerate()
        .fold(0u32, |acc, (i, &b)| if b { acc | (1 << i) } else { acc });

    for (_, sig) in existing_sigs {
        if sig == new_sig {
            return false;
        }
    }

    true
}

fn verify_column_uniqueness(grid: &Grid) -> bool {
    let sigs = grid.complete_col_signatures();

    for i in 0..sigs.len() {
        for j in (i + 1)..sigs.len() {
            if sigs[i].1 == sigs[j].1 {
                return false;
            }
        }
    }

    true
}