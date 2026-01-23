use crate::digger::Digger;
use crate::grid::{Grid, Puzzle};
// use crate::solver::BacktrackSolver;
use rand::seq::SliceRandom;
use rand::Rng;

pub fn generate_puzzles(size: usize, count: usize, difficulty: u8) -> Vec<Puzzle> {
    let mut puzzles = Vec::with_capacity(count);
    let mut rng = rand::thread_rng();
    let digger = Digger::new(difficulty.into());

    let mut attempts = 0;
    let max_attempts = count * 100;

    while puzzles.len() < count && attempts < max_attempts {
        attempts += 1;

        if let Some(complete_grid) = generate_complete_grid(size, &mut rng) {
            if let Some(puzzle_grid) = digger.dig(&complete_grid) {
                puzzles.push(puzzle_grid.to_puzzle(&complete_grid));

                if puzzles.len() % 10 == 0 {
                    println!("Generated {}/{} puzzles...", puzzles.len(), count);
                }
            }
        }
    }

    puzzles
}

fn generate_complete_grid(size: usize, rng: &mut impl Rng) -> Option<Grid> {
    let valid_lines = precompute_valid_lines(size);

    let mut grid = Grid::new(size);
    if fill_grid_recursive(&mut grid, 0, &valid_lines, rng) {
        Some(grid)
    } else {
        None
    }
}

/// Precompute all valid lines (rows/columns) for a given size
fn precompute_valid_lines(size: usize) -> Vec<Vec<bool>> {
    let max = size / 2;
    let mut valid = Vec::new();

    for mask in 0..(1u32 << size) {
        let line: Vec<bool> = (0..size).map(|i| (mask >> i) & 1 == 1).collect();

        // Check equal count
        let ones = line.iter().filter(|&&b| b).count();
        if ones != max {
            continue;
        }

        // Check no triple
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

fn fill_grid_recursive(
    grid: &mut Grid,
    row: usize,
    valid_lines: &[Vec<bool>],
    rng: &mut impl Rng,
) -> bool {
    let size = grid.size;

    if row == size {
        // Verify column uniqueness
        return verify_column_uniqueness(grid);
    }

    // Get compatible lines for this row
    let mut candidates: Vec<&Vec<bool>> = valid_lines
        .iter()
        .filter(|line| is_line_compatible(grid, row, line))
        .collect();

    candidates.shuffle(rng);

    for line in candidates {
        // Place the line
        for (col, &val) in line.iter().enumerate() {
            grid.set(row, col, Some(val));
        }

        if fill_grid_recursive(grid, row + 1, valid_lines, rng) {
            return true;
        }

        // Backtrack
        for col in 0..size {
            grid.set(row, col, None);
        }
    }

    false
}

fn is_line_compatible(grid: &Grid, row: usize, line: &[bool]) -> bool {
    let size = grid.size;
    let max = size / 2;

    // Check column constraints
    for col in 0..size {
        let val = line[col];

        // Check vertical triples
        if row >= 2 {
            let above1 = grid.get(row - 1, col);
            let above2 = grid.get(row - 2, col);
            if above1 == Some(val) && above2 == Some(val) {
                return false;
            }
        }

        // Check column quota
        let col_cells = grid.col(col);
        let count = Grid::count_in_line(&col_cells, val);
        if count >= max {
            return false;
        }
    }

    // Check row uniqueness against previous rows
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