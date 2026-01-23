use crate::grid::{Cell, Grid};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Difficulty {
    Level1 = 1, // Triplets only
    Level2 = 2, // + Simple counting
    Level3 = 3, // + Anticipation
    Level4 = 4, // + Line uniqueness
    Level5 = 5, // + Single-level contradiction
}

impl From<u8> for Difficulty {
    fn from(v: u8) -> Self {
        match v {
            1 => Difficulty::Level1,
            2 => Difficulty::Level2,
            3 => Difficulty::Level3,
            4 => Difficulty::Level4,
            _ => Difficulty::Level5,
        }
    }
}

/// Result of applying a technique: list of (row, col, value) deductions
pub type Deductions = Vec<(usize, usize, bool)>;

/// Technique 1: Avoid triplets (local patterns)
pub fn avoid_triplets(grid: &Grid) -> Deductions {
    let mut deductions = Vec::new();
    let n = grid.size;

    for r in 0..n {
        for c in 0..n {
            if grid.get(r, c).is_some() {
                continue;
            }

            // Try each value and see if one is forbidden
            let mut forbidden = [false, false];
            for (idx, val) in [false, true].iter().enumerate() {
                if grid.would_create_triple(r, c, *val) {
                    forbidden[idx] = true;
                }
            }

            if forbidden[0] && !forbidden[1] {
                deductions.push((r, c, true));
            } else if forbidden[1] && !forbidden[0] {
                deductions.push((r, c, false));
            }
        }
    }

    deductions
}

/// Technique 2: Simple counting (quota reached)
pub fn simple_counting(grid: &Grid) -> Deductions {
    let mut deductions = Vec::new();
    let n = grid.size;
    let max = n / 2;

    // Check rows
    for r in 0..n {
        let row = grid.row(r);
        let zeros = Grid::count_in_line(row, false);
        let ones = Grid::count_in_line(row, true);

        if zeros == max {
            // All empty cells must be 1
            for c in 0..n {
                if grid.get(r, c).is_none() {
                    deductions.push((r, c, true));
                }
            }
        } else if ones == max {
            // All empty cells must be 0
            for c in 0..n {
                if grid.get(r, c).is_none() {
                    deductions.push((r, c, false));
                }
            }
        }
    }

    // Check columns
    for c in 0..n {
        let col = grid.col(c);
        let zeros = Grid::count_in_line(&col, false);
        let ones = Grid::count_in_line(&col, true);

        if zeros == max {
            for r in 0..n {
                if grid.get(r, c).is_none() {
                    deductions.push((r, c, true));
                }
            }
        } else if ones == max {
            for r in 0..n {
                if grid.get(r, c).is_none() {
                    deductions.push((r, c, false));
                }
            }
        }
    }

    deductions
}

/// Technique 3: Anticipation (placing a value forces a triple that breaks quota)
pub fn anticipation(grid: &Grid) -> Deductions {
    let mut deductions = Vec::new();
    let n = grid.size;
    let max = n / 2;

    for r in 0..n {
        for c in 0..n {
            if grid.get(r, c).is_some() {
                continue;
            }

            let mut forbidden = [false, false];

            for (idx, &val) in [false, true].iter().enumerate() {
                // Simulate placing val
                let mut test_grid = grid.clone();
                test_grid.set(r, c, Some(val));

                // Propagate forced moves from triplet rule
                if let Some(result) = propagate_triplets(&mut test_grid) {
                    if !result {
                        forbidden[idx] = true;
                        continue;
                    }
                }

                // Check if quotas are broken
                if check_quota_violation(&test_grid) {
                    forbidden[idx] = true;
                }
            }

            if forbidden[0] && !forbidden[1] {
                deductions.push((r, c, true));
            } else if forbidden[1] && !forbidden[0] {
                deductions.push((r, c, false));
            }
        }
    }

    deductions
}

/// Helper: propagate triplet rule, returns None if no progress, Some(false) if contradiction
fn propagate_triplets(grid: &mut Grid) -> Option<bool> {
    let mut made_progress = false;
    let mut changed = true;

    while changed {
        changed = false;
        let deductions = avoid_triplets(grid);

        for (r, c, val) in deductions {
            if grid.get(r, c).is_none() {
                // Check for contradiction
                if grid.would_create_triple(r, c, val) {
                    return Some(false);
                }
                if grid.would_exceed_quota(r, c, val) {
                    return Some(false);
                }
                grid.set(r, c, Some(val));
                changed = true;
                made_progress = true;
            }
        }
    }

    if made_progress {
        Some(true)
    } else {
        None
    }
}

/// Helper: check if any row/column has exceeded its quota
fn check_quota_violation(grid: &Grid) -> bool {
    let n = grid.size;
    let max = n / 2;

    for r in 0..n {
        let row = grid.row(r);
        if Grid::count_in_line(row, false) > max || Grid::count_in_line(row, true) > max {
            return true;
        }
    }

    for c in 0..n {
        let col = grid.col(c);
        if Grid::count_in_line(&col, false) > max || Grid::count_in_line(&col, true) > max {
            return true;
        }
    }

    false
}

/// Technique 4: Line uniqueness
pub fn line_uniqueness(grid: &Grid) -> Deductions {
    let mut deductions = Vec::new();
    let n = grid.size;

    // Get existing complete signatures
    let complete_rows = grid.complete_row_signatures();
    let complete_cols = grid.complete_col_signatures();

    // For each almost-complete row (1 or 2 empty cells)
    for r in 0..n {
        let row = grid.row(r);
        let empty_count = Grid::empty_in_line(row);

        if empty_count == 0 || empty_count > 2 {
            continue;
        }

        // Find possible completions
        let empty_positions: Vec<usize> = (0..n).filter(|&c| grid.get(r, c).is_none()).collect();

        let valid_completions = find_valid_row_completions(grid, r, &empty_positions, &complete_rows);

        if let Some(forced) = find_forced_values(&empty_positions, &valid_completions) {
            for (c, val) in forced {
                deductions.push((r, c, val));
            }
        }
    }

    // For each almost-complete column
    for c in 0..n {
        let col = grid.col(c);
        let empty_count = Grid::empty_in_line(&col);

        if empty_count == 0 || empty_count > 2 {
            continue;
        }

        let empty_positions: Vec<usize> = (0..n).filter(|&r| grid.get(r, c).is_none()).collect();

        let valid_completions = find_valid_col_completions(grid, c, &empty_positions, &complete_cols);

        if let Some(forced) = find_forced_values(&empty_positions, &valid_completions) {
            for (r, val) in forced {
                deductions.push((r, c, val));
            }
        }
    }

    deductions
}

fn find_valid_row_completions(
    grid: &Grid,
    row: usize,
    empty_positions: &[usize],
    existing_sigs: &[(usize, u32)],
) -> Vec<Vec<bool>> {
    let n = grid.size;
    let max = n / 2;
    let current_zeros = Grid::count_in_line(grid.row(row), false);
    let current_ones = Grid::count_in_line(grid.row(row), true);

    let num_empty = empty_positions.len();
    let combinations = 1 << num_empty;

    let mut valid = Vec::new();

    'outer: for mask in 0..combinations {
        let values: Vec<bool> = (0..num_empty).map(|i| (mask >> i) & 1 == 1).collect();

        // Check quota
        let added_ones = values.iter().filter(|&&v| v).count();
        let added_zeros = num_empty - added_ones;

        if current_zeros + added_zeros > max || current_ones + added_ones > max {
            continue;
        }

        // Check triplet rule
        let mut test_grid = grid.clone();
        for (i, &col) in empty_positions.iter().enumerate() {
            if test_grid.would_create_triple(row, col, values[i]) {
                continue 'outer;
            }
            test_grid.set(row, col, Some(values[i]));
        }

        // Check uniqueness against existing complete rows
        if let Some(sig) = Grid::line_as_bits(test_grid.row(row)) {
            for &(existing_row, existing_sig) in existing_sigs {
                if existing_row != row && sig == existing_sig {
                    continue 'outer;
                }
            }
        }

        valid.push(values);
    }

    valid
}

fn find_valid_col_completions(
    grid: &Grid,
    col: usize,
    empty_positions: &[usize],
    existing_sigs: &[(usize, u32)],
) -> Vec<Vec<bool>> {
    let n = grid.size;
    let max = n / 2;
    let col_cells = grid.col(col);
    let current_zeros = Grid::count_in_line(&col_cells, false);
    let current_ones = Grid::count_in_line(&col_cells, true);

    let num_empty = empty_positions.len();
    let combinations = 1 << num_empty;

    let mut valid = Vec::new();

    'outer: for mask in 0..combinations {
        let values: Vec<bool> = (0..num_empty).map(|i| (mask >> i) & 1 == 1).collect();

        // Check quota
        let added_ones = values.iter().filter(|&&v| v).count();
        let added_zeros = num_empty - added_ones;

        if current_zeros + added_zeros > max || current_ones + added_ones > max {
            continue;
        }

        // Check triplet rule
        let mut test_grid = grid.clone();
        for (i, &row) in empty_positions.iter().enumerate() {
            if test_grid.would_create_triple(row, col, values[i]) {
                continue 'outer;
            }
            test_grid.set(row, col, Some(values[i]));
        }

        // Check uniqueness against existing complete cols
        let test_col = test_grid.col(col);
        if let Some(sig) = Grid::line_as_bits(&test_col) {
            for &(existing_col, existing_sig) in existing_sigs {
                if existing_col != col && sig == existing_sig {
                    continue 'outer;
                }
            }
        }

        valid.push(values);
    }

    valid
}

fn find_forced_values(
    empty_positions: &[usize],
    valid_completions: &[Vec<bool>],
) -> Option<Vec<(usize, bool)>> {
    if valid_completions.is_empty() {
        return None; // Contradiction
    }

    if valid_completions.len() == 1 {
        // All values are forced
        return Some(
            empty_positions
                .iter()
                .zip(valid_completions[0].iter())
                .map(|(&pos, &val)| (pos, val))
                .collect(),
        );
    }

    // Check if any position has the same value in all valid completions
    let mut forced = Vec::new();
    for (i, &pos) in empty_positions.iter().enumerate() {
        let first_val = valid_completions[0][i];
        if valid_completions.iter().all(|comp| comp[i] == first_val) {
            forced.push((pos, first_val));
        }
    }

    if forced.is_empty() {
        None
    } else {
        Some(forced)
    }
}

/// Technique 5: Single-level contradiction
pub fn single_contradiction(grid: &Grid) -> Deductions {
    let mut deductions = Vec::new();
    let n = grid.size;

    for r in 0..n {
        for c in 0..n {
            if grid.get(r, c).is_some() {
                continue;
            }

            let mut forbidden = [false, false];

            for (idx, &val) in [false, true].iter().enumerate() {
                let mut test_grid = grid.clone();
                test_grid.set(r, c, Some(val));

                // Try to solve with techniques 1-4
                if leads_to_contradiction(&mut test_grid) {
                    forbidden[idx] = true;
                }
            }

            if forbidden[0] && !forbidden[1] {
                deductions.push((r, c, true));
            } else if forbidden[1] && !forbidden[0] {
                deductions.push((r, c, false));
            }
        }
    }

    deductions
}

fn leads_to_contradiction(grid: &mut Grid) -> bool {
    // Propagate with techniques 1-4 and detect contradictions
    let mut changed = true;

    while changed {
        changed = false;

        // Check for quota violations
        if check_quota_violation(grid) {
            return true;
        }

        // Check for duplicate complete lines
        let row_sigs = grid.complete_row_signatures();
        let col_sigs = grid.complete_col_signatures();

        for i in 0..row_sigs.len() {
            for j in (i + 1)..row_sigs.len() {
                if row_sigs[i].1 == row_sigs[j].1 {
                    return true;
                }
            }
        }

        for i in 0..col_sigs.len() {
            for j in (i + 1)..col_sigs.len() {
                if col_sigs[i].1 == col_sigs[j].1 {
                    return true;
                }
            }
        }

        // Apply techniques 1-2
        for technique in [avoid_triplets, simple_counting] {
            let deductions = technique(grid);
            for (r, c, val) in deductions {
                if grid.get(r, c).is_none() {
                    // Verify it doesn't immediately violate rules
                    if grid.would_create_triple(r, c, val) || grid.would_exceed_quota(r, c, val) {
                        return true;
                    }
                    grid.set(r, c, Some(val));
                    changed = true;
                }
            }
        }
    }

    false
}