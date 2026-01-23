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
        let max_attempts = count * 50;

        while variants.len() < count && attempts < max_attempts {
            attempts += 1;

            let mut grid = source.clone();

            // Choisir des transformations aléatoires
            let mut transforms: Vec<u8> = (0..8).collect();
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
                    6 => Self::shuffle_rows(&mut grid, rng),
                    7 => Self::shuffle_cols(&mut grid, rng),
                    _ => {}
                }
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

    fn shuffle_rows(grid: &mut Grid, rng: &mut dyn RngCore) {
        let n = grid.size;
        let mut indices: Vec<usize> = (0..n).collect();
        indices.shuffle(rng);

        let old_values: Vec<u32> = (0..n).map(|r| grid.get_row_values(r)).collect();

        for (new_r, &old_r) in indices.iter().enumerate() {
            grid.set_row(new_r, old_values[old_r]);
        }
    }

    fn shuffle_cols(grid: &mut Grid, rng: &mut dyn RngCore) {
        let n = grid.size;
        let mut indices: Vec<usize> = (0..n).collect();
        indices.shuffle(rng);

        let mut new_grid = Grid::new(n);

        for r in 0..n {
            let mut new_val = 0u32;
            for (new_c, &old_c) in indices.iter().enumerate() {
                if grid.get(r, old_c).unwrap() {
                    new_val |= 1 << new_c;
                }
            }
            new_grid.set_row(r, new_val);
        }

        *grid = new_grid;
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