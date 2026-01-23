use crate::digger::Digger;
use crate::grid::{Grid, Puzzle};
use crate::transform::Transformer;
use rand::seq::SliceRandom;
use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

#[derive(Clone)]
struct ValidLine {
    bits: u32,
}

pub fn generate_puzzles(size: usize, count: usize, difficulty: u8) -> Vec<Puzzle> {
    let valid_lines = precompute_valid_lines(size);
    let start = Instant::now();

    let sources_needed = (count / 5).max(3).min(count);
    let variants_per_source = (count / sources_needed) + 2;

    println!(
        "Strategy: {} source grids × ~{} variants",
        sources_needed, variants_per_source
    );

    let source_grids: Vec<Grid> = generate_source_grids(size, sources_needed, &valid_lines);

    println!(
        "Generated {} source grids in {:?}",
        source_grids.len(),
        start.elapsed()
    );

    let mut puzzles: Vec<Puzzle> = Vec::with_capacity(count);

for source in source_grids.iter() {
        if puzzles.len() >= count {
            break;
        }

        let mut rng = rand::thread_rng();
        let needed = (count - puzzles.len()).min(variants_per_source);

        let mut variants = Transformer::generate_variants(source, needed, &mut rng);
        variants.insert(0, source.clone());
    
        let dug: Vec<Puzzle> = variants
    .par_iter()
    .filter_map(|complete_grid| {
        let digger = Digger::new(difficulty.into(), size);
        let puzzle_grid = digger.dig(complete_grid)?;
        Some(puzzle_grid.to_puzzle(complete_grid))
    })
    .collect();

        for puzzle in dug {
            if puzzles.len() < count {
                puzzles.push(puzzle);
                println!(
                    "Generated {}/{} puzzles ({:?})",
                    puzzles.len(),
                    count,
                    start.elapsed()
                );
            }
        }
    }

    puzzles
}

fn generate_source_grids(size: usize, count: usize, valid_lines: &[ValidLine]) -> Vec<Grid> {
    let mut grids = Vec::new();
    let batch_size = num_cpus::get() * 2;

    while grids.len() < count {
        let batch: Vec<Grid> = (0..batch_size)
            .into_par_iter()
            .filter_map(|_| {
                let mut rng = rand::thread_rng();
                generate_complete_grid_with_constraints(size, valid_lines, &mut rng, 100_000)
            })
            .collect();

        for grid in batch {
            if grids.len() < count {
                grids.push(grid);
            }
        }
    }

    grids
}

fn generate_complete_grid_with_constraints(
    size: usize,
    valid_lines: &[ValidLine],
    rng: &mut impl Rng,
    max_iterations: usize,
) -> Option<Grid> {
    let mut grid = Grid::new(size);
    let mut iterations = 0;

    let max_per_line = (size / 2) as u32;
    let mut col_ones_remaining = vec![max_per_line; size];
    let mut col_zeros_remaining = vec![max_per_line; size];
    let mut used_row_sigs: Vec<u32> = Vec::with_capacity(size);

    if fill_with_constraints(
        &mut grid,
        0,
        size,
        valid_lines,
        &mut col_ones_remaining,
        &mut col_zeros_remaining,
        &mut used_row_sigs,
        rng,
        &mut iterations,
        max_iterations,
    ) {
        if grid.cols_unique() {
            return Some(grid);
        }
    }

    None
}

fn fill_with_constraints(
    grid: &mut Grid,
    row: usize,
    size: usize,
    valid_lines: &[ValidLine],
    col_ones_remaining: &mut [u32],
    col_zeros_remaining: &mut [u32],
    used_row_sigs: &mut Vec<u32>,
    rng: &mut impl Rng,
    iterations: &mut usize,
    max_iterations: usize,
) -> bool {
    *iterations += 1;
    if *iterations >= max_iterations {
        return false;
    }

    if row == size {
        return true;
    }

    let mut candidates: Vec<&ValidLine> = valid_lines
        .iter()
        .filter(|line| {
            for col in 0..size {
                let is_one = line.bits & (1 << col) != 0;
                if is_one {
                    if col_ones_remaining[col] == 0 {
                        return false;
                    }
                } else if col_zeros_remaining[col] == 0 {
                    return false;
                }
            }

            if row >= 2 {
                for col in 0..size {
                    let val = line.bits & (1 << col) != 0;
                    let prev1 = grid.get(row - 1, col).unwrap();
                    let prev2 = grid.get(row - 2, col).unwrap();
                    if val == prev1 && prev1 == prev2 {
                        return false;
                    }
                }
            }

            !used_row_sigs.contains(&line.bits)
        })
        .collect();

    candidates.shuffle(rng);

    for line in candidates {
        grid.set_row(row, line.bits);
        used_row_sigs.push(line.bits);

        for col in 0..size {
            if line.bits & (1 << col) != 0 {
                col_ones_remaining[col] -= 1;
            } else {
                col_zeros_remaining[col] -= 1;
            }
        }

        if fill_with_constraints(
            grid,
            row + 1,
            size,
            valid_lines,
            col_ones_remaining,
            col_zeros_remaining,
            used_row_sigs,
            rng,
            iterations,
            max_iterations,
        ) {
            return true;
        }

        for col in 0..size {
            if line.bits & (1 << col) != 0 {
                col_ones_remaining[col] += 1;
            } else {
                col_zeros_remaining[col] += 1;
            }
        }
        used_row_sigs.pop();

        if *iterations >= max_iterations {
            return false;
        }
    }

    false
}

fn precompute_valid_lines(size: usize) -> Vec<ValidLine> {
    let max = size / 2;
    let mut valid = Vec::new();

    for bits in 0..(1u32 << size) {
        let ones = bits.count_ones() as usize;
        if ones != max {
            continue;
        }

        let mut has_triple = false;
        for i in 0..size.saturating_sub(2) {
            let window = (bits >> i) & 0b111;
            if window == 0b111 || window == 0b000 {
                has_triple = true;
                break;
            }
        }

        if !has_triple {
            valid.push(ValidLine { bits });
        }
    }

    valid
}