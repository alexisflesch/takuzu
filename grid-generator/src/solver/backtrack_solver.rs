use crate::grid::Grid;

pub struct BacktrackSolver;

impl BacktrackSolver {
    /// Count the number of solutions (stops at 2 for uniqueness check)
    pub fn count_solutions(grid: &Grid, max_count: usize) -> usize {
        let mut grid = grid.clone();
        let mut count = 0;
        Self::solve_recursive(&mut grid, &mut count, max_count);
        count
    }

    /// Check if the grid has exactly one solution
    pub fn has_unique_solution(grid: &Grid) -> bool {
        Self::count_solutions(grid, 2) == 1
    }

    fn solve_recursive(grid: &mut Grid, count: &mut usize, max_count: usize) {
        if *count >= max_count {
            return;
        }

        // Find first empty cell
        let empty_cell = Self::find_best_empty_cell(grid);

        match empty_cell {
            None => {
                // Grid is complete, verify it's valid
                if Self::is_valid_complete(grid) {
                    *count += 1;
                }
            }
            Some((row, col)) => {
                for &val in &[false, true] {
                    if Self::is_valid_placement(grid, row, col, val) {
                        grid.set(row, col, Some(val));
                        Self::solve_recursive(grid, count, max_count);
                        grid.set(row, col, None);

                        if *count >= max_count {
                            return;
                        }
                    }
                }
            }
        }
    }

    fn find_best_empty_cell(grid: &Grid) -> Option<(usize, usize)> {
        // Use MRV heuristic: choose cell with fewest valid options
        let mut best: Option<(usize, usize, usize)> = None;

        for r in 0..grid.size {
            for c in 0..grid.size {
                if grid.get(r, c).is_none() {
                    let mut options = 0;
                    for &val in &[false, true] {
                        if Self::is_valid_placement(grid, r, c, val) {
                            options += 1;
                        }
                    }

                    if options == 0 {
                        return Some((r, c)); // Will fail immediately, prune fast
                    }

                    match best {
                        None => best = Some((r, c, options)),
                        Some((_, _, best_options)) if options < best_options => {
                            best = Some((r, c, options));
                        }
                        _ => {}
                    }
                }
            }
        }

        best.map(|(r, c, _)| (r, c))
    }

    fn is_valid_placement(grid: &Grid, row: usize, col: usize, value: bool) -> bool {
        // Check triple rule
        if grid.would_create_triple(row, col, value) {
            return false;
        }

        // Check quota
        if grid.would_exceed_quota(row, col, value) {
            return false;
        }

        true
    }

    fn is_valid_complete(grid: &Grid) -> bool {
        let n = grid.size;

        // Check uniqueness of rows
        let row_sigs = grid.complete_row_signatures();
        for i in 0..row_sigs.len() {
            for j in (i + 1)..row_sigs.len() {
                if row_sigs[i].1 == row_sigs[j].1 {
                    return false;
                }
            }
        }

        // Check uniqueness of columns
        let col_sigs = grid.complete_col_signatures();
        for i in 0..col_sigs.len() {
            for j in (i + 1)..col_sigs.len() {
                if col_sigs[i].1 == col_sigs[j].1 {
                    return false;
                }
            }
        }

        // Verify quotas (should already be satisfied)
        let max = n / 2;
        for r in 0..n {
            let row = grid.row(r);
            if Grid::count_in_line(row, false) != max || Grid::count_in_line(row, true) != max {
                return false;
            }
        }

        true
    }
}