use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Puzzle {
    pub size: usize,
    pub puzzle: Vec<Vec<Option<u8>>>,
    pub solution: Vec<Vec<u8>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid {
    pub size: usize,
    values: Vec<u32>,
    filled: Vec<u32>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            values: vec![0; size],
            filled: vec![0; size],
        }
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> Option<bool> {
        let mask = 1u32 << col;
        if self.filled[row] & mask != 0 {
            Some(self.values[row] & mask != 0)
        } else {
            None
        }
    }

    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: Option<bool>) {
        let mask = 1u32 << col;
        match value {
            Some(v) => {
                self.filled[row] |= mask;
                if v {
                    self.values[row] |= mask;
                } else {
                    self.values[row] &= !mask;
                }
            }
            None => {
                self.filled[row] &= !mask;
                self.values[row] &= !mask;
            }
        }
    }

    #[inline]
    pub fn set_row(&mut self, row: usize, values: u32) {
        let full_mask = (1u32 << self.size) - 1;
        self.values[row] = values;
        self.filled[row] = full_mask;
    }

    #[inline]
    pub fn get_row_values(&self, row: usize) -> u32 {
        self.values[row]
    }

    #[inline]
    pub fn get_row_filled(&self, row: usize) -> u32 {
        self.filled[row]
    }

    #[inline]
    pub fn get_col_values(&self, col: usize) -> u32 {
        let mut result = 0u32;
        for row in 0..self.size {
            if self.values[row] & (1 << col) != 0 {
                result |= 1 << row;
            }
        }
        result
    }

    #[inline]
    pub fn get_col_filled(&self, col: usize) -> u32 {
        let mut result = 0u32;
        for row in 0..self.size {
            if self.filled[row] & (1 << col) != 0 {
                result |= 1 << row;
            }
        }
        result
    }

    #[inline]
    pub fn is_complete(&self) -> bool {
        let full_mask = (1u32 << self.size) - 1;
        self.filled.iter().all(|&f| f == full_mask)
    }

    #[inline]
    pub fn filled_count(&self) -> usize {
        self.filled.iter().map(|f| f.count_ones() as usize).sum()
    }

    #[inline]
    pub fn count_ones_in_row(&self, row: usize) -> u32 {
        (self.values[row] & self.filled[row]).count_ones()
    }

    #[inline]
    pub fn count_zeros_in_row(&self, row: usize) -> u32 {
        (!self.values[row] & self.filled[row]).count_ones()
    }

    #[inline]
    pub fn count_ones_in_col(&self, col: usize) -> u32 {
        let vals = self.get_col_values(col);
        let filled = self.get_col_filled(col);
        (vals & filled).count_ones()
    }

    #[inline]
    pub fn count_zeros_in_col(&self, col: usize) -> u32 {
        let vals = self.get_col_values(col);
        let filled = self.get_col_filled(col);
        (!vals & filled).count_ones()
    }

    #[inline]
    pub fn would_create_triple(&self, row: usize, col: usize, value: bool) -> bool {
        self.check_horizontal_triple(row, col, value) || self.check_vertical_triple(row, col, value)
    }

    #[inline]
    fn check_horizontal_triple(&self, row: usize, col: usize, value: bool) -> bool {
        let filled = self.filled[row];
        let vals = if value {
            self.values[row]
        } else {
            !self.values[row]
        };

        if col >= 2 {
            let mask = 0b11 << (col - 2);
            if filled & mask == mask && vals & mask == mask {
                return true;
            }
        }
        if col >= 1 && col + 1 < self.size {
            let mask_left = 1 << (col - 1);
            let mask_right = 1 << (col + 1);
            if filled & mask_left != 0
                && filled & mask_right != 0
                && vals & mask_left != 0
                && vals & mask_right != 0
            {
                return true;
            }
        }
        if col + 2 < self.size {
            let mask = 0b11 << (col + 1);
            if filled & mask == mask && vals & mask == mask {
                return true;
            }
        }
        false
    }

    #[inline]
    fn check_vertical_triple(&self, row: usize, col: usize, value: bool) -> bool {
        let mask = 1u32 << col;

        let get_val = |r: usize| -> Option<bool> {
            if self.filled[r] & mask != 0 {
                Some(self.values[r] & mask != 0)
            } else {
                None
            }
        };

        if row >= 2 && get_val(row - 1) == Some(value) && get_val(row - 2) == Some(value) {
            return true;
        }
        if row >= 1
            && row + 1 < self.size
            && get_val(row - 1) == Some(value)
            && get_val(row + 1) == Some(value)
        {
            return true;
        }
        if row + 2 < self.size
            && get_val(row + 1) == Some(value)
            && get_val(row + 2) == Some(value)
        {
            return true;
        }
        false
    }

    #[inline]
    pub fn would_exceed_quota(&self, row: usize, col: usize, value: bool) -> bool {
        let max = (self.size / 2) as u32;

        if value {
            self.count_ones_in_row(row) >= max || self.count_ones_in_col(col) >= max
        } else {
            self.count_zeros_in_row(row) >= max || self.count_zeros_in_col(col) >= max
        }
    }

    pub fn complete_row_signatures(&self) -> Vec<u32> {
        let full_mask = (1u32 << self.size) - 1;
        (0..self.size)
            .filter(|&r| self.filled[r] == full_mask)
            .map(|r| self.values[r])
            .collect()
    }

    pub fn complete_col_signatures(&self) -> Vec<u32> {
        let full_mask = (1u32 << self.size) - 1;
        (0..self.size)
            .filter(|&c| self.get_col_filled(c) == full_mask)
            .map(|c| self.get_col_values(c))
            .collect()
    }

    pub fn rows_unique(&self) -> bool {
        let sigs = self.complete_row_signatures();
        for i in 0..sigs.len() {
            for j in (i + 1)..sigs.len() {
                if sigs[i] == sigs[j] {
                    return false;
                }
            }
        }
        true
    }

    pub fn cols_unique(&self) -> bool {
        let sigs = self.complete_col_signatures();
        for i in 0..sigs.len() {
            for j in (i + 1)..sigs.len() {
                if sigs[i] == sigs[j] {
                    return false;
                }
            }
        }
        true
    }

    pub fn to_puzzle(&self, solution: &Grid) -> Puzzle {
        Puzzle {
            size: self.size,
            puzzle: (0..self.size)
                .map(|r| {
                    (0..self.size)
                        .map(|c| self.get(r, c).map(|v| if v { 1 } else { 0 }))
                        .collect()
                })
                .collect(),
            solution: (0..solution.size)
                .map(|r| {
                    (0..solution.size)
                        .map(|c| if solution.get(r, c).unwrap() { 1 } else { 0 })
                        .collect()
                })
                .collect(),
        }
    }
}