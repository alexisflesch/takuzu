use crate::grid::Grid;
use crate::solver::techniques::*;

pub struct HumanSolver {
    difficulty: Difficulty,
}

impl HumanSolver {
    pub fn new(difficulty: Difficulty) -> Self {
        Self { difficulty }
    }

    /// Solves the grid using a progressive strategy:
    /// Start with Level 1 rules, apply until stable.
    /// If not complete, enable Level 2 rules and continue, etc.
    /// Returns the solved grid if solvable within the solver's difficulty level.
    pub fn solve(&self, grid: &Grid) -> Option<Grid> {
        let (solved_grid, _) = self.solve_with_level_tracking(grid, self.difficulty as u8)?;
        Some(solved_grid)
    }

    /// Evaluates the minimal difficulty required to solve the grid.
    pub fn evaluate_difficulty(&self, grid: &Grid) -> Option<Difficulty> {
        let (_, level) = self.solve_with_level_tracking(grid, 4)?;
        Some(Difficulty::from(level))
    }

    fn solve_with_level_tracking(&self, grid: &Grid, max_level: u8) -> Option<(Grid, u8)> {
        let mut current_grid = grid.clone();

        for level in 1..=max_level {
            let mut changed_at_this_level = true;
            
            while changed_at_this_level && !current_grid.is_complete() {
                changed_at_this_level = false;

                // Always try simplest rules first (1 to current level)
                'outer: for tech_level in 1..=level {
                    let techniques = self.get_techniques_for_level_only(tech_level);
                    for technique in techniques {
                        let deductions = technique(&current_grid);
                        let mut applied = false;
                        for (r, c, val) in deductions {
                            if current_grid.get(r, c).is_none() {
                                current_grid.set(r, c, Some(val));
                                applied = true;
                                changed_at_this_level = true;
                            }
                        }
                        if applied {
                            // Progress made with tech_level rule. 
                            // Restart from Level 1 rules to stay minimal.
                            break 'outer;
                        }
                    }
                }
            }

            if current_grid.is_complete() {
                return Some((current_grid, level));
            }
        }

        None
    }

    fn get_techniques_for_level_only(&self, level: u8) -> Vec<fn(&Grid) -> Deductions> {
        match level {
            1 => vec![
                avoid_triplets,
                simple_counting,
                sandwich_patterns,
                extended_sandwich_patterns,
                extended_triplet_avoidance,
            ],
            2 => vec![
                anticipation,
                line_uniqueness,
                near_duplicate_detection,
                forced_completion,
            ],
            3 => vec![single_contradiction],
            4 => vec![multi_step_hypothesis],
            _ => vec![],
        }
    }
}