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

        let mut positions: Vec<(usize, usize)> = (0..size)
            .flat_map(|r| (0..size).map(move |c| (r, c)))
            .collect();

        positions.shuffle(&mut rand::thread_rng());

        let human_solver = HumanSolver::new(self.difficulty);

        // Phase 1: Retrait agressif par batch (sans vérifier l'unicité à chaque fois)
        let batch_size = match size {
            s if s <= 6 => 4,
            s if s <= 10 => 6,
            _ => 8,
        };

        let mut idx = 0;
        while idx < positions.len() && grid.filled_count() > min_filled {
            let batch_end = (idx + batch_size).min(positions.len());
            let mut removed = Vec::new();

            // Retirer un batch
            for &(row, col) in &positions[idx..batch_end] {
                if grid.get(row, col).is_some() {
                    let value = grid.get(row, col);
                    removed.push((row, col, value));
                    grid.set(row, col, None);
                }
            }

            // Vérifier unicité une seule fois pour le batch
            if !BacktrackSolver::has_unique_solution(&grid) {
                // Remettre tout et passer en mode cellule par cellule
                for (row, col, value) in removed {
                    grid.set(row, col, value);
                }
                // Essayer une par une
                for &(row, col) in &positions[idx..batch_end] {
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

        // Phase 2: Vérifier que le solveur humain peut résoudre
        if human_solver.solve(&grid).is_some() && BacktrackSolver::has_unique_solution(&grid) {
            Some(grid)
        } else {
            // Fallback: remettre des cellules jusqu'à ce que ce soit soluble
            self.make_human_solvable(&mut grid, complete_grid, &human_solver)
        }
    }

    fn make_human_solvable(
        &self,
        grid: &mut Grid,
        solution: &Grid,
        human_solver: &HumanSolver,
    ) -> Option<Grid> {
        let size = grid.size;
        let mut positions: Vec<(usize, usize)> = (0..size)
            .flat_map(|r| (0..size).map(move |c| (r, c)))
            .filter(|&(r, c)| grid.get(r, c).is_none())
            .collect();

        positions.shuffle(&mut rand::thread_rng());

        for (row, col) in positions {
            if human_solver.solve(grid).is_some() {
                return Some(grid.clone());
            }
            // Remettre la valeur de la solution
            grid.set(row, col, solution.get(row, col));
        }

        if human_solver.solve(grid).is_some() {
            Some(grid.clone())
        } else {
            None
        }
    }
}