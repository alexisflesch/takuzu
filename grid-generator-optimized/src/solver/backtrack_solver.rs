use crate::grid::Grid;

pub struct BacktrackSolver;

impl BacktrackSolver {
    pub fn count_solutions(grid: &Grid, max_count: usize) -> usize {
        let mut grid = grid.clone();
        let mut count = 0;
        Self::solve_recursive(&mut grid, &mut count, max_count);
        count
    }

    pub fn has_unique_solution(grid: &Grid) -> bool {
        Self::count_solutions(grid, 2) == 1
    }

    fn solve_recursive(grid: &mut Grid, count: &mut usize, max_count: usize) {
        if *count >= max_count {
            return;
        }

        let empty_cell = Self::find_best_empty_cell(grid);

        match empty_cell {
            None => {
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
                        return Some((r, c));
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
        !grid.would_create_triple(row, col, value) && !grid.would_exceed_quota(row, col, value)
    }

    fn is_valid_complete(grid: &Grid) -> bool {
        let n = grid.size;
        let max = (n / 2) as u32;

        // Check row uniqueness
        if !grid.rows_unique() {
            return false;
        }

        // Check column uniqueness
        if !grid.cols_unique() {
            return false;
        }

        // Verify quotas
        for r in 0..n {
            if grid.count_zeros_in_row(r) != max || grid.count_ones_in_row(r) != max {
                return false;
            }
        }

        true
    }
}