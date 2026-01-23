use crate::grid::Grid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Difficulty {
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
    Level4 = 4,
    Level5 = 5,
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

pub type Deductions = Vec<(usize, usize, bool)>;

pub fn avoid_triplets(grid: &Grid) -> Deductions {
    let mut deductions = Vec::new();
    let n = grid.size;

    for r in 0..n {
        for c in 0..n {
            if grid.get(r, c).is_some() {
                continue;
            }

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

pub fn simple_counting(grid: &Grid) -> Deductions {
    let mut deductions = Vec::new();
    let n = grid.size;
    let max = (n / 2) as u32;

    for r in 0..n {
        let zeros = grid.count_zeros_in_row(r);
        let ones = grid.count_ones_in_row(r);

        if zeros == max {
            for c in 0..n {
                if grid.get(r, c).is_none() {
                    deductions.push((r, c, true));
                }
            }
        } else if ones == max {
            for c in 0..n {
                if grid.get(r, c).is_none() {
                    deductions.push((r, c, false));
                }
            }
        }
    }

    for c in 0..n {
        let zeros = grid.count_zeros_in_col(c);
        let ones = grid.count_ones_in_col(c);

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

pub fn anticipation(grid: &Grid) -> Deductions {
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

                if let Some(result) = propagate_triplets(&mut test_grid) {
                    if !result {
                        forbidden[idx] = true;
                        continue;
                    }
                }

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

fn propagate_triplets(grid: &mut Grid) -> Option<bool> {
    let mut made_progress = false;
    let mut changed = true;

    while changed {
        changed = false;
        let deductions = avoid_triplets(grid);

        for (r, c, val) in deductions {
            if grid.get(r, c).is_none() {
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

fn check_quota_violation(grid: &Grid) -> bool {
    let n = grid.size;
    let max = (n / 2) as u32;

    for r in 0..n {
        if grid.count_zeros_in_row(r) > max || grid.count_ones_in_row(r) > max {
            return true;
        }
    }

    for c in 0..n {
        if grid.count_zeros_in_col(c) > max || grid.count_ones_in_col(c) > max {
            return true;
        }
    }

    false
}

pub fn line_uniqueness(grid: &Grid) -> Deductions {
    let mut deductions = Vec::new();
    let n = grid.size;

    let complete_rows = grid.complete_row_signatures();
    let complete_cols = grid.complete_col_signatures();

    // Check rows with 1-2 empty cells
    for r in 0..n {
        let filled = grid.get_row_filled(r);
        let full_mask = (1u32 << n) - 1;
        let empty_count = (full_mask ^ filled).count_ones();

        if empty_count == 0 || empty_count > 2 {
            continue;
        }

        let empty_positions: Vec<usize> = (0..n)
            .filter(|&c| grid.get(r, c).is_none())
            .collect();

        let valid = find_valid_row_completions(grid, r, &empty_positions, &complete_rows);

        if let Some(forced) = find_forced_values(&empty_positions, &valid) {
            for (c, val) in forced {
                deductions.push((r, c, val));
            }
        }
    }

    // Check cols with 1-2 empty cells
    for c in 0..n {
        let filled = grid.get_col_filled(c);
        let full_mask = (1u32 << n) - 1;
        let empty_count = (full_mask ^ filled).count_ones();

        if empty_count == 0 || empty_count > 2 {
            continue;
        }

        let empty_positions: Vec<usize> = (0..n)
            .filter(|&r| grid.get(r, c).is_none())
            .collect();

        let valid = find_valid_col_completions(grid, c, &empty_positions, &complete_cols);

        if let Some(forced) = find_forced_values(&empty_positions, &valid) {
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
    existing_sigs: &[u32],
) -> Vec<Vec<bool>> {
    let n = grid.size;
    let max = (n / 2) as u32;
    let current_zeros = grid.count_zeros_in_row(row);
    let current_ones = grid.count_ones_in_row(row);

    let num_empty = empty_positions.len();
    let combinations = 1 << num_empty;

    let mut valid = Vec::new();

    'outer: for mask in 0..combinations {
        let values: Vec<bool> = (0..num_empty).map(|i| (mask >> i) & 1 == 1).collect();

        let added_ones = values.iter().filter(|&&v| v).count() as u32;
        let added_zeros = (num_empty as u32) - added_ones;

        if current_zeros + added_zeros > max || current_ones + added_ones > max {
            continue;
        }

        let mut test_grid = grid.clone();
        for (i, &col) in empty_positions.iter().enumerate() {
            if test_grid.would_create_triple(row, col, values[i]) {
                continue 'outer;
            }
            test_grid.set(row, col, Some(values[i]));
        }

        let sig = test_grid.get_row_values(row);
        if existing_sigs.contains(&sig) {
            continue 'outer;
        }

        valid.push(values);
    }

    valid
}

fn find_valid_col_completions(
    grid: &Grid,
    col: usize,
    empty_positions: &[usize],
    existing_sigs: &[u32],
) -> Vec<Vec<bool>> {
    let n = grid.size;
    let max = (n / 2) as u32;
    let current_zeros = grid.count_zeros_in_col(col);
    let current_ones = grid.count_ones_in_col(col);

    let num_empty = empty_positions.len();
    let combinations = 1 << num_empty;

    let mut valid = Vec::new();

    'outer: for mask in 0..combinations {
        let values: Vec<bool> = (0..num_empty).map(|i| (mask >> i) & 1 == 1).collect();

        let added_ones = values.iter().filter(|&&v| v).count() as u32;
        let added_zeros = (num_empty as u32) - added_ones;

        if current_zeros + added_zeros > max || current_ones + added_ones > max {
            continue;
        }

        let mut test_grid = grid.clone();
        for (i, &row) in empty_positions.iter().enumerate() {
            if test_grid.would_create_triple(row, col, values[i]) {
                continue 'outer;
            }
            test_grid.set(row, col, Some(values[i]));
        }

        let sig = test_grid.get_col_values(col);
        if existing_sigs.contains(&sig) {
            continue 'outer;
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
        return None;
    }

    if valid_completions.len() == 1 {
        return Some(
            empty_positions
                .iter()
                .zip(valid_completions[0].iter())
                .map(|(&pos, &val)| (pos, val))
                .collect(),
        );
    }

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

// src/solver/techniques.rs - version optimisée de single_contradiction

pub fn single_contradiction(grid: &Grid) -> Deductions {
    let mut deductions = Vec::new();
    let n = grid.size;

    // Pour les grandes grilles, ne tester que les cellules "prometteuses"
    // (celles avec beaucoup de voisins remplis)
    let mut candidates: Vec<(usize, usize, usize)> = Vec::new();
    
    for r in 0..n {
        for c in 0..n {
            if grid.get(r, c).is_some() {
                continue;
            }
            
            // Score = nombre de voisins remplis
            let score = count_filled_neighbors(grid, r, c);
            candidates.push((r, c, score));
        }
    }
    
    // Trier par score décroissant et ne garder que les meilleurs
    candidates.sort_by(|a, b| b.2.cmp(&a.2));
    let max_candidates = if n <= 12 { candidates.len() } else { candidates.len().min(n * 2) };

    for (r, c, _) in candidates.into_iter().take(max_candidates) {
        let mut forbidden = [false, false];

        for (idx, &val) in [false, true].iter().enumerate() {
            let mut test_grid = grid.clone();
            test_grid.set(r, c, Some(val));

            if leads_to_contradiction_limited(&mut test_grid, 20) {
                forbidden[idx] = true;
            }
        }

        if forbidden[0] && !forbidden[1] {
            deductions.push((r, c, true));
        } else if forbidden[1] && !forbidden[0] {
            deductions.push((r, c, false));
        }
    }

    deductions
}

fn count_filled_neighbors(grid: &Grid, row: usize, col: usize) -> usize {
    let n = grid.size;
    let mut count = 0;
    
    for dr in -1i32..=1 {
        for dc in -1i32..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                if grid.get(nr as usize, nc as usize).is_some() {
                    count += 1;
                }
            }
        }
    }
    count
}

fn leads_to_contradiction_limited(grid: &mut Grid, max_iterations: usize) -> bool {
    let mut iterations = 0;
    let mut changed = true;

    while changed && iterations < max_iterations {
        changed = false;
        iterations += 1;

        if check_quota_violation(grid) {
            return true;
        }

        let row_sigs = grid.complete_row_signatures();
        for i in 0..row_sigs.len() {
            for j in (i + 1)..row_sigs.len() {
                if row_sigs[i] == row_sigs[j] {
                    return true;
                }
            }
        }

        let col_sigs = grid.complete_col_signatures();
        for i in 0..col_sigs.len() {
            for j in (i + 1)..col_sigs.len() {
                if col_sigs[i] == col_sigs[j] {
                    return true;
                }
            }
        }

        for technique in [avoid_triplets, simple_counting] {
            let deductions = technique(grid);
            for (r, c, val) in deductions {
                if grid.get(r, c).is_none() {
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

