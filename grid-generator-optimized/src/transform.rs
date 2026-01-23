use crate::grid::Grid;
use rand::seq::SliceRandom;
use rand::RngCore;

pub struct Transformer;

impl Transformer {
    pub fn generate_variants(source: &Grid, count: usize, rng: &mut dyn RngCore) -> Vec<Grid> {
        let mut variants = Vec::with_capacity(count);
        let mut seen_signatures: std::collections::HashSet<Vec<u32>> =
            std::collections::HashSet::new();

        seen_signatures.insert(Self::signature(source));

        let mut attempts = 0;
        let max_attempts = count * 100;

        while variants.len() < count && attempts < max_attempts {
            attempts += 1;

            let mut grid = source.clone();

            // Seulement les transformations SÛRES (préservent les règles)
            let mut transforms: Vec<u8> = (0..6).collect();
            transforms.shuffle(rng);

            let num_transforms = 2 + (rng.next_u32() % 3) as usize;
            for &t in transforms.iter().take(num_transforms) {
                match t {
                    0 => Self::swap_values(&mut grid),
                    1 => Self::rotate_90(&mut grid),
                    2 => Self::rotate_180(&mut grid),
                    3 => Self::rotate_270(&mut grid),
                    4 => Self::mirror_horizontal(&mut grid),
                    5 => Self::mirror_vertical(&mut grid),
                    _ => {}
                }
            }

            // Double vérification que la grille est valide
            if !Self::is_valid(&grid) {
                continue;
            }

            let sig = Self::signature(&grid);
            if !seen_signatures.contains(&sig) {
                seen_signatures.insert(sig);
                variants.push(grid);
            }
        }

        variants
    }

    fn signature(grid: &Grid) -> Vec<u32> {
        (0..grid.size).map(|r| grid.get_row_values(r)).collect()
    }

    /// Vérifie que la grille respecte toutes les règles du Takuzu
    fn is_valid(grid: &Grid) -> bool {
        let n = grid.size;
        let max = (n / 2) as u32;

        // Vérifier pas de triplets et quotas corrects pour chaque ligne
        for r in 0..n {
            let row_val = grid.get_row_values(r);
            
            // Check triplets horizontaux
            for c in 0..n.saturating_sub(2) {
                let window = (row_val >> c) & 0b111;
                if window == 0b111 || window == 0b000 {
                    return false;
                }
            }

            // Check quota ligne
            if grid.count_ones_in_row(r) != max || grid.count_zeros_in_row(r) != max {
                return false;
            }
        }

        // Vérifier pas de triplets verticaux et quotas colonnes
        for c in 0..n {
            let col_val = grid.get_col_values(c);
            
            // Check triplets verticaux
            for r in 0..n.saturating_sub(2) {
                let window = (col_val >> r) & 0b111;
                if window == 0b111 || window == 0b000 {
                    return false;
                }
            }

            // Check quota colonne
            if grid.count_ones_in_col(c) != max || grid.count_zeros_in_col(c) != max {
                return false;
            }
        }

        // Vérifier unicité des lignes et colonnes
        if !grid.rows_unique() || !grid.cols_unique() {
            return false;
        }

        true
    }

    fn swap_values(grid: &mut Grid) {
        let mask = (1u32 << grid.size) - 1;
        for row in 0..grid.size {
            let vals = grid.get_row_values(row);
            grid.set_row(row, (!vals) & mask);
        }
    }

    fn rotate_90(grid: &mut Grid) {
        let n = grid.size;
        let mut new_grid = Grid::new(n);

        for r in 0..n {
            for c in 0..n {
                let val = grid.get(r, c).unwrap();
                new_grid.set(c, n - 1 - r, Some(val));
            }
        }

        *grid = new_grid;
    }

    fn rotate_180(grid: &mut Grid) {
        let n = grid.size;
        let mut new_grid = Grid::new(n);

        for r in 0..n {
            for c in 0..n {
                let val = grid.get(r, c).unwrap();
                new_grid.set(n - 1 - r, n - 1 - c, Some(val));
            }
        }

        *grid = new_grid;
    }

    fn rotate_270(grid: &mut Grid) {
        let n = grid.size;
        let mut new_grid = Grid::new(n);

        for r in 0..n {
            for c in 0..n {
                let val = grid.get(r, c).unwrap();
                new_grid.set(n - 1 - c, r, Some(val));
            }
        }

        *grid = new_grid;
    }

    fn mirror_horizontal(grid: &mut Grid) {
        let n = grid.size;
        for r in 0..n / 2 {
            let top = grid.get_row_values(r);
            let bottom = grid.get_row_values(n - 1 - r);
            grid.set_row(r, bottom);
            grid.set_row(n - 1 - r, top);
        }
    }

    fn mirror_vertical(grid: &mut Grid) {
        let n = grid.size;
        for r in 0..n {
            let vals = grid.get_row_values(r);
            let reversed = Self::reverse_bits(vals, n);
            grid.set_row(r, reversed);
        }
    }

    fn reverse_bits(val: u32, size: usize) -> u32 {
        let mut result = 0u32;
        for i in 0..size {
            if val & (1 << i) != 0 {
                result |= 1 << (size - 1 - i);
            }
        }
        result
    }
}