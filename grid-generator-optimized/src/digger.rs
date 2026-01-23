use crate::grid::Grid;
use crate::solver::{BacktrackSolver, Difficulty, HumanSolver};
use rand::seq::SliceRandom;
use std::time::{Duration, Instant};

pub struct Digger {
    difficulty: Difficulty,
    target_fill_ratio: f64,
    timeout: Duration,
}

impl Digger {
    pub fn new(difficulty: Difficulty, size: usize) -> Self {
        // Adapter le ratio selon taille ET difficulté
        let target_fill_ratio = match (difficulty, size) {
            // Petites grilles : on peut creuser beaucoup
            (Difficulty::Level1, _) => 0.65,
            (Difficulty::Level2, _) => 0.55,
            (Difficulty::Level3, s) if s <= 12 => 0.45,
            (Difficulty::Level3, _) => 0.50,
            (Difficulty::Level4, s) if s <= 12 => 0.40,
            (Difficulty::Level4, s) if s <= 16 => 0.45,
            (Difficulty::Level4, _) => 0.50,
            (Difficulty::Level5, s) if s <= 10 => 0.35,
            (Difficulty::Level5, s) if s <= 14 => 0.40,
            (Difficulty::Level5, s) if s <= 16 => 0.45,
            (Difficulty::Level5, _) => 0.50, // 18+ : on creuse moins
        };

        // Timeout adaptatif
        let timeout = match size {
            s if s <= 10 => Duration::from_secs(5),
            s if s <= 14 => Duration::from_secs(10),
            s if s <= 16 => Duration::from_secs(20),
            _ => Duration::from_secs(30),
        };

        Self {
            difficulty,
            target_fill_ratio,
            timeout,
        }
    }

    pub fn dig(&self, complete_grid: &Grid) -> Option<Grid> {
        let start = Instant::now();
        let mut grid = complete_grid.clone();
        let size = grid.size;
        let total_cells = size * size;
        let min_filled = (total_cells as f64 * self.target_fill_ratio) as usize;

        let mut positions: Vec<(usize, usize)> = (0..size)
            .flat_map(|r| (0..size).map(move |c| (r, c)))
            .collect();

        positions.shuffle(&mut rand::thread_rng());

        let human_solver = HumanSolver::new(self.difficulty);

        let batch_size = match size {
            s if s <= 6 => 4,
            s if s <= 10 => 6,
            s if s <= 14 => 8,
            _ => 10,
        };

        let mut idx = 0;
        while idx < positions.len() && grid.filled_count() > min_filled {
            // Timeout check
            if start.elapsed() > self.timeout {
                break;
            }

            let batch_end = (idx + batch_size).min(positions.len());
            let mut removed = Vec::new();

            for &(row, col) in &positions[idx..batch_end] {
                if grid.get(row, col).is_some() {
                    let value = grid.get(row, col);
                    removed.push((row, col, value));
                    grid.set(row, col, None);
                }
            }

            if !BacktrackSolver::has_unique_solution(&grid) {
                for (row, col, value) in removed {
                    grid.set(row, col, value);
                }
                for &(row, col) in &positions[idx..batch_end] {
                    if start.elapsed() > self.timeout {
                        break;
                    }
                    if grid.filled_count() <= min_filled {
                        break;
                    }
                    if grid.get(row, col).is_some() {
                        let value = grid.get(row, col);
                        grid.set(row, col, None);

                        if !BacktrackSolver::has_unique_solution(&grid) {
                            grid.set(row, col, value);
                        }
                    }
                }
            }

            idx = batch_end;
        }

        // Vérification finale avec timeout
        if start.elapsed() > self.timeout {
            // On accepte ce qu'on a si c'est valide
            if BacktrackSolver::has_unique_solution(&grid) {
                return Some(grid);
            }
            return None;
        }

        if human_solver.solve(&grid).is_some() && BacktrackSolver::has_unique_solution(&grid) {
            Some(grid)
        } else {
            self.make_human_solvable(&mut grid, complete_grid, &human_solver, start)
        }
    }

    fn make_human_solvable(
        &self,
        grid: &mut Grid,
        solution: &Grid,
        human_solver: &HumanSolver,
        start: Instant,
    ) -> Option<Grid> {
        let size = grid.size;
        let mut positions: Vec<(usize, usize)> = (0..size)
            .flat_map(|r| (0..size).map(move |c| (r, c)))
            .filter(|&(r, c)| grid.get(r, c).is_none())
            .collect();

        positions.shuffle(&mut rand::thread_rng());

        for (row, col) in positions {
            if start.elapsed() > self.timeout {
                break;
            }
            if human_solver.solve(grid).is_some() {
                return Some(grid.clone());
            }
            grid.set(row, col, solution.get(row, col));
        }

        if human_solver.solve(grid).is_some() {
            Some(grid.clone())
        } else {
            None
        }
    }
}