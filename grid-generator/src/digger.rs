use crate::grid::Grid;
use crate::solver::{BacktrackSolver, Difficulty, HumanSolver};
use rand::seq::SliceRandom;

pub struct Digger {
    difficulty: Difficulty,
    target_fill_ratio: f64,
}

impl Digger {
    pub fn new(difficulty: Difficulty) -> Self {
        let target_fill_ratio = match difficulty {
            Difficulty::Level1 => 0.65,
            Difficulty::Level2 => 0.55,
            Difficulty::Level3 => 0.45,
            Difficulty::Level4 => 0.40,
            Difficulty::Level5 => 0.35,
        };

        Self {
            difficulty,
            target_fill_ratio,
        }
    }

    pub fn dig(&self, complete_grid: &Grid) -> Option<Grid> {
        let mut grid = complete_grid.clone();
        let size = grid.size;
        let total_cells = size * size;
        let min_filled = (total_cells as f64 * self.target_fill_ratio) as usize;

        // Create shuffled list of all positions
        let mut positions: Vec<(usize, usize)> = (0..size)
            .flat_map(|r| (0..size).map(move |c| (r, c)))
            .collect();

        positions.shuffle(&mut rand::thread_rng());

        let human_solver = HumanSolver::new(self.difficulty);

        for (row, col) in positions {
            if grid.filled_count() <= min_filled {
                break;
            }

            let value = grid.get(row, col);
            grid.set(row, col, None);

            // Check 1: Can human solver (at this difficulty) solve it?
            let human_can_solve = human_solver.solve(&grid).is_some();

            // Check 2: Is the solution still unique?
            let is_unique = BacktrackSolver::has_unique_solution(&grid);

            if !human_can_solve || !is_unique {
                // Restore the cell
                grid.set(row, col, value);
            }
        }

        // Verify we have a valid puzzle
        if BacktrackSolver::has_unique_solution(&grid) {
            Some(grid)
        } else {
            None
        }
    }
}