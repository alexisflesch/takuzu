use crate::grid::Grid;
use crate::solver::techniques::*;

pub struct HumanSolver {
    difficulty: Difficulty,
}

impl HumanSolver {
    pub fn new(difficulty: Difficulty) -> Self {
        Self { difficulty }
    }

    /// Try to solve the grid using only techniques up to the configured difficulty.
    /// Returns Some(grid) if solved, None if stuck or contradiction.
    pub fn solve(&self, grid: &Grid) -> Option<Grid> {
        let mut grid = grid.clone();
        let mut changed = true;

        while changed && !grid.is_complete() {
            changed = false;

            let techniques: Vec<fn(&Grid) -> Deductions> = match self.difficulty {
                Difficulty::Level1 => vec![avoid_triplets],
                Difficulty::Level2 => vec![avoid_triplets, simple_counting],
                Difficulty::Level3 => vec![avoid_triplets, simple_counting, anticipation],
                Difficulty::Level4 => vec![
                    avoid_triplets,
                    simple_counting,
                    anticipation,
                    line_uniqueness,
                ],
                Difficulty::Level5 => vec![
                    avoid_triplets,
                    simple_counting,
                    anticipation,
                    line_uniqueness,
                    single_contradiction,
                ],
            };

            for technique in techniques {
                let deductions = technique(&grid);
                if !deductions.is_empty() {
                    for (r, c, val) in deductions {
                        if grid.get(r, c).is_none() {
                            grid.set(r, c, Some(val));
                            changed = true;
                        }
                    }
                    break; // Restart from simplest technique
                }
            }
        }

        if grid.is_complete() {
            Some(grid)
        } else {
            None
        }
    }
}