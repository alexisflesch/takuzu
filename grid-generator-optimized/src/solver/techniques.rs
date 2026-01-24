use crate::grid::Grid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Difficulty {
    Level1 = 1, // Depth 0 (Basic deductions)
    Level2 = 2, // Depth 1 (Line-level reasoning)
    Level3 = 3, // Depth 1 (Single hypothesis)
    Level4 = 4, // Depth 2 (Multi-step hypothesis)
}

impl From<u8> for Difficulty {
    fn from(v: u8) -> Self {
        match v {
            1 => Difficulty::Level1,
            2 => Difficulty::Level2,
            3 => Difficulty::Level3,
            _ => Difficulty::Level4,
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

// Depth 0: Basic sandwich patterns (X . X)
pub fn sandwich_patterns(grid: &Grid) -> Deductions {
    let mut deductions = Vec::new();
    let n = grid.size;

    // Check rows for X . X pattern
    for r in 0..n {
        for c in 0..n.saturating_sub(2) {
            // Look for pattern: filled, empty, filled
            if let (Some(left), Some(right)) = (grid.get(r, c), grid.get(r, c + 2)) {
                if left == right && grid.get(r, c + 1).is_none() {
                    // Middle must be opposite of the sides
                    deductions.push((r, c + 1, !left));
                }
            }
        }
    }

    // Check columns for X . X pattern
    for c in 0..n {
        for r in 0..n.saturating_sub(2) {
            if let (Some(top), Some(bottom)) = (grid.get(r, c), grid.get(r + 2, c)) {
                if top == bottom && grid.get(r + 1, c).is_none() {
                    deductions.push((r + 1, c, !top));
                }
            }
        }
    }

    deductions
}

// Depth 0: Extended sandwich patterns (X . . X)
pub fn extended_sandwich_patterns(grid: &Grid) -> Deductions {
    let mut deductions = Vec::new();
    let n = grid.size;

    // Check rows for X . . X pattern
    for r in 0..n {
        for c in 0..n.saturating_sub(3) {
            if let (Some(left), Some(right)) = (grid.get(r, c), grid.get(r, c + 3)) {
                if left == right {
                    // Check middle cells
                    let mid1 = grid.get(r, c + 1);
                    let mid2 = grid.get(r, c + 2);

                    match (mid1, mid2) {
                        (None, None) => {
                            // Both middle empty - cannot have two of the same as ends
                            // This is handled by other rules, but we can note the constraint
                        }
                        (Some(m1), None) => {
                            // First middle filled, second empty
                            if m1 == left {
                                // Would create triplet, so second must be opposite
                                deductions.push((r, c + 2, !left));
                            }
                        }
                        (None, Some(m2)) => {
                            // Second middle filled, first empty
                            if m2 == left {
                                deductions.push((r, c + 1, !left));
                            }
                        }
                        (Some(m1), Some(m2)) => {
                            // Both filled - check for invalid patterns
                            if m1 == left && m2 == left {
                                // This would violate triplet rule, but should be caught elsewhere
                            }
                        }
                    }
                }
            }
        }
    }

    // Similar logic for columns
    for c in 0..n {
        for r in 0..n.saturating_sub(3) {
            if let (Some(top), Some(bottom)) = (grid.get(r, c), grid.get(r + 3, c)) {
                if top == bottom {
                    let mid1 = grid.get(r + 1, c);
                    let mid2 = grid.get(r + 2, c);

                    match (mid1, mid2) {
                        (Some(m1), None) => {
                            if m1 == top {
                                deductions.push((r + 2, c, !top));
                            }
                        }
                        (None, Some(m2)) => {
                            if m2 == top {
                                deductions.push((r + 1, c, !top));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    deductions
}

// Depth 0: Extended triplet avoidance with larger windows
pub fn extended_triplet_avoidance(grid: &Grid) -> Deductions {
    let mut deductions = Vec::new();
    let n = grid.size;

    // Check for potential triplets in windows of size 4-5
    for r in 0..n {
        for start_c in 0..n.saturating_sub(3) {
            let window_size = if n >= 5 && start_c <= n - 5 { 5 } else { 4 };
            // Ensure window_size doesn't exceed available cells
            let window_size = window_size.min(n - start_c); 
            let end_c = start_c + window_size;

            let window: Vec<Option<bool>> = (start_c..end_c)
                .map(|c| grid.get(r, c))
                .collect();

            // Count filled cells
            let filled_count = window.iter().filter(|&&v| v.is_some()).count();

            if filled_count >= 3 {
                // Look for patterns that would force triplets
                for i in 0..window_size.saturating_sub(2) {
                    if let (Some(a), Some(b)) = (window[i], window[i + 1]) {
                        if a == b {
                            // Two same in a row
                            if i + 2 < window_size && window[i + 2].is_none() {
                                // Next cell empty - would create triplet
                                deductions.push((r, start_c + i + 2, !a));
                            }
                        }
                    }
                }
            }
        }
    }

    // Similar for columns
    for c in 0..n {
        for start_r in 0..n.saturating_sub(3) {
            let window_size = if n >= 5 && start_r <= n - 5 { 5 } else { 4 };
            let window_size = window_size.min(n - start_r);
            let end_r = start_r + window_size;

            let window: Vec<Option<bool>> = (start_r..end_r)
                .map(|r| grid.get(r, c))
                .collect();

            let filled_count = window.iter().filter(|&&v| v.is_some()).count();

            if filled_count >= 3 {
                for i in 0..window_size.saturating_sub(2) {
                    if let (Some(a), Some(b)) = (window[i], window[i + 1]) {
                        if a == b && i + 2 < window_size && window[i + 2].is_none() {
                            deductions.push((start_r + i + 2, c, !a));
                        }
                    }
                }
            }
        }
    }

    deductions
}

// Depth 1: Near-duplicate rows or columns (differing in only a few cells)
pub fn near_duplicate_detection(grid: &Grid) -> Deductions {
    let mut deductions = Vec::new();
    let n = grid.size;

    // Check rows that differ in only 1-2 positions
    let row_sigs: Vec<u32> = (0..n).map(|r| grid.get_row_values(r)).collect();
    let row_filled: Vec<u32> = (0..n).map(|r| grid.get_row_filled(r)).collect();

    for i in 0..n {
        for j in (i + 1)..n {
            let filled_i = row_filled[i];
            let filled_j = row_filled[j];

            // Only compare if they have similar filled patterns
            let common_filled = filled_i & filled_j;
            let differing_positions = (filled_i ^ filled_j) | ((row_sigs[i] ^ row_sigs[j]) & common_filled);

            if differing_positions.count_ones() <= 2 {
                // Find positions where they differ
                let diff_positions: Vec<usize> = (0..n)
                    .filter(|&c| (differing_positions & (1 << c)) != 0)
                    .collect();

                // For each differing position, if one is filled and the other empty,
                // we can potentially deduce the empty one
                for &c in &diff_positions {
                    let val_i = grid.get(i, c);
                    let val_j = grid.get(j, c);

                    match (val_i, val_j) {
                        (Some(v), None) => {
                            // Row i has value, row j is empty at this position
                            // If they would be identical otherwise, this must be different
                            deductions.push((j, c, !v));
                        }
                        (None, Some(v)) => {
                            deductions.push((i, c, !v));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Similar logic for columns
    let col_sigs: Vec<u32> = (0..n).map(|c| grid.get_col_values(c)).collect();
    let col_filled: Vec<u32> = (0..n).map(|c| grid.get_col_filled(c)).collect();

    for i in 0..n {
        for j in (i + 1)..n {
            let filled_i = col_filled[i];
            let filled_j = col_filled[j];

            let common_filled = filled_i & filled_j;
            let differing_positions = (filled_i ^ filled_j) | ((col_sigs[i] ^ col_sigs[j]) & common_filled);

            if differing_positions.count_ones() <= 2 {
                let diff_positions: Vec<usize> = (0..n)
                    .filter(|&r| (differing_positions & (1 << r)) != 0)
                    .collect();

                for &r in &diff_positions {
                    let val_i = grid.get(r, i);
                    let val_j = grid.get(r, j);

                    match (val_i, val_j) {
                        (Some(v), None) => {
                            deductions.push((r, j, !v));
                        }
                        (None, Some(v)) => {
                            deductions.push((r, i, !v));
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    deductions
}

// Depth 1: Forced line or column completion
pub fn forced_completion(grid: &Grid) -> Deductions {
    let mut deductions = Vec::new();
    let n = grid.size;
    let max_per_line = (n / 2) as u32;

    // Check rows that are almost complete
    for r in 0..n {
        let ones = grid.count_ones_in_row(r);
        let zeros = grid.count_zeros_in_row(r);
        let filled = ones + zeros;

        if filled == n as u32 - 1 {
            // One cell empty
            if ones == max_per_line {
                // Must fill with 0
                for c in 0..n {
                    if grid.get(r, c).is_none() {
                        deductions.push((r, c, false));
                        break;
                    }
                }
            } else if zeros == max_per_line {
                // Must fill with 1
                for c in 0..n {
                    if grid.get(r, c).is_none() {
                        deductions.push((r, c, true));
                        break;
                    }
                }
            }
        }
    }

    // Check columns that are almost complete
    for c in 0..n {
        let ones = grid.count_ones_in_col(c);
        let zeros = grid.count_zeros_in_col(c);
        let filled = ones + zeros;

        if filled == n as u32 - 1 {
            if ones == max_per_line {
                for r in 0..n {
                    if grid.get(r, c).is_none() {
                        deductions.push((r, c, false));
                        break;
                    }
                }
            } else if zeros == max_per_line {
                for r in 0..n {
                    if grid.get(r, c).is_none() {
                        deductions.push((r, c, true));
                        break;
                    }
                }
            }
        }
    }

    deductions
}
// Depth 2: Controlled multi-step hypothesis
// Tests a hypothesis and applies full logical propagation (depth-0 and depth-1 rules)
pub fn multi_step_hypothesis(grid: &Grid) -> Deductions {
    let mut deductions = Vec::new();
    let n = grid.size;

    // For performance, only test cells with high neighbor density
    let mut candidates: Vec<(usize, usize, usize)> = Vec::new();

    for r in 0..n {
        for c in 0..n {
            if grid.get(r, c).is_some() {
                continue;
            }
            let density = count_filled_neighbors(grid, r, c);
            candidates.push((r, c, density));
        }
    }

    candidates.sort_by(|a, b| b.2.cmp(&a.2));
    let max_candidates = if n <= 12 { 20 } else { 15 };

    for (r, c, _) in candidates.into_iter().take(max_candidates) {
        let mut forbidden = [false, false];

        for (idx, &val) in [false, true].iter().enumerate() {
            let mut test_grid = grid.clone();
            test_grid.set(r, c, Some(val));

            if leads_to_contradiction_full(&mut test_grid) {
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

fn leads_to_contradiction_full(grid: &mut Grid) -> bool {
    let mut changed = true;
    let mut iterations = 0;
    let max_iterations = 50; // Allow more iterations for depth-2

    while changed && iterations < max_iterations {
        changed = false;
        iterations += 1;

        // Check for immediate contradictions
        if check_quota_violation(grid) {
            return true;
        }

        // Check for duplicate rows/columns
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

        // Apply all depth-0 and depth-1 techniques
        let depth_0_techniques = vec![
            avoid_triplets,
            sandwich_patterns,
            extended_sandwich_patterns,
            extended_triplet_avoidance,
            simple_counting,
        ];

        let depth_1_techniques = vec![
            anticipation,
            line_uniqueness,
            near_duplicate_detection,
            forced_completion,
        ];

        for technique in depth_0_techniques.iter().chain(depth_1_techniques.iter()) {
            let technique_deductions = technique(grid);
            for (r, c, val) in technique_deductions {
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

