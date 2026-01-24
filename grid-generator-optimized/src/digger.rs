use crate::grid::Grid;
use crate::solver::{BacktrackSolver, Difficulty, HumanSolver};
use rand::seq::SliceRandom;
use std::time::{Duration, Instant};

pub struct Digger {
    difficulty: Difficulty,
    target_fill_ratio: f64,
    timeout: Duration,
    try_harder: bool,
}

impl Digger {
    pub fn new(difficulty: Difficulty, size: usize) -> Self {
        // Adjust fill ratio based on both difficulty AND grid size.
        // For small grids (<10), we target 100% potential digging (remove as much as possible).
        let target_fill_ratio = if size < 10 {
            0.0
        } else {
            match (difficulty, size) {
                // Level 1: Basic deductions
                (Difficulty::Level1, s) if s <= 12 => 0.65,
                (Difficulty::Level1, _) => 0.70,

                // Level 2: Line-level reasoning
                (Difficulty::Level2, s) if s <= 12 => 0.55,
                (Difficulty::Level2, _) => 0.60,

                // Level 3: Hypothesis (contradiction)
                (Difficulty::Level3, s) if s <= 12 => 0.45,
                (Difficulty::Level3, s) if s <= 16 => 0.50,
                (Difficulty::Level3, _) => 0.55,

                // Level 4: Multi-step hypothesis
                (Difficulty::Level4, s) if s <= 10 => 0.35,
                (Difficulty::Level4, s) if s <= 14 => 0.40,
                (Difficulty::Level4, s) if s <= 16 => 0.45,
                (Difficulty::Level4, _) => 0.50,
            }
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
            try_harder: false,
        }
    }

    pub fn set_try_harder(&mut self, try_harder: bool) {
        self.try_harder = try_harder;
        if try_harder {
            // Push limits even further
            self.target_fill_ratio -= 0.10;
            self.timeout *= 2;
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

        // Stratégie : creuser jusqu'à atteindre le ratio cible OU ne plus pouvoir creuser du tout
        // On vérifie à chaque étape que la difficulté reste stable (ne dépasse pas la cible)
        let mut idx = 0;
        
        // If try_harder is true, we ignore the target_fill_ratio and try to dig EVERY possible cell
        let stop_condition = |filled: usize| -> bool {
            if self.try_harder {
                // For small grids, no safety floor. For larger ones, stay above 25%
                if size < 10 {
                    filled > 0
                } else {
                    filled > (total_cells / 4)
                }
            } else {
                filled > min_filled
            }
        };

        while idx < positions.len() && stop_condition(grid.filled_count()) {
            // Timeout check
            if start.elapsed() > self.timeout {
                break;
            }

            let (row, col) = positions[idx];
            idx += 1;

            let value = grid.get(row, col);
            if value.is_none() {
                continue;
            }

            grid.set(row, col, None);

            // Vérification : 
            // 1. Unicité de la solution
            // 2. Solvabilité par le niveau de difficulté cible (donc difficulté <= cible)
            if !BacktrackSolver::has_unique_solution(&grid) || human_solver.solve(&grid).is_none() {
                // Si la difficulté a augmenté au-delà de la cible ou si pas de solution unique, on remet
                grid.set(row, col, value);
            }
        }

        // Vérification finale
        if human_solver.solve(&grid).is_some() {
            Some(grid)
        } else {
            None
        }
    }
}