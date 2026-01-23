use serde::Serialize;

/// Cell value: None = empty, Some(false) = 0, Some(true) = 1
pub type Cell = Option<bool>;

#[derive(Clone, Debug, Serialize)]
pub struct Puzzle {
    pub size: usize,
    pub puzzle: Vec<Vec<Option<u8>>>,
    pub solution: Vec<Vec<u8>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid {
    pub size: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            cells: vec![vec![None; size]; size],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: Cell) {
        self.cells[row][col] = value;
    }

    pub fn is_complete(&self) -> bool {
        self.cells.iter().all(|row| row.iter().all(|c| c.is_some()))
    }

    /// Count filled cells
    pub fn filled_count(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|c| c.is_some())
            .count()
    }

    /// Get row as slice of cells
    pub fn row(&self, idx: usize) -> &[Cell] {
        &self.cells[idx]
    }

    /// Get column as Vec of cells
    pub fn col(&self, idx: usize) -> Vec<Cell> {
        (0..self.size).map(|r| self.cells[r][idx]).collect()
    }

    /// Count occurrences of a value in a line (row or col)
    pub fn count_in_line(line: &[Cell], value: bool) -> usize {
        line.iter()
            .filter(|c| **c == Some(value))
            .count()
    }

    /// Count empty cells in a line
    pub fn empty_in_line(line: &[Cell]) -> usize {
        line.iter().filter(|c| c.is_none()).count()
    }

    /// Check if placing `value` at (row, col) would create a triple
    pub fn would_create_triple(&self, row: usize, col: usize, value: bool) -> bool {
        let v = Some(value);

        // Horizontal check
        let left2 = col >= 2 && self.cells[row][col - 1] == v && self.cells[row][col - 2] == v;
        let left_right = col >= 1
            && col + 1 < self.size
            && self.cells[row][col - 1] == v
            && self.cells[row][col + 1] == v;
        let right2 =
            col + 2 < self.size && self.cells[row][col + 1] == v && self.cells[row][col + 2] == v;

        // Vertical check
        let up2 = row >= 2 && self.cells[row - 1][col] == v && self.cells[row - 2][col] == v;
        let up_down = row >= 1
            && row + 1 < self.size
            && self.cells[row - 1][col] == v
            && self.cells[row + 1][col] == v;
        let down2 =
            row + 2 < self.size && self.cells[row + 1][col] == v && self.cells[row + 2][col] == v;

        left2 || left_right || right2 || up2 || up_down || down2
    }

    /// Check if value would exceed quota in row or column
    pub fn would_exceed_quota(&self, row: usize, col: usize, value: bool) -> bool {
        let max = self.size / 2;
        let row_count = Self::count_in_line(self.row(row), value);
        let col_count = Self::count_in_line(&self.col(col), value);
        row_count >= max || col_count >= max
    }

    /// Check if a complete row/col would duplicate another
    pub fn line_as_bits(line: &[Cell]) -> Option<u32> {
        let mut bits = 0u32;
        for (i, cell) in line.iter().enumerate() {
            match cell {
                Some(true) => bits |= 1 << i,
                Some(false) => {}
                None => return None,
            }
        }
        Some(bits)
    }

    /// Get all complete row signatures
    pub fn complete_row_signatures(&self) -> Vec<(usize, u32)> {
        (0..self.size)
            .filter_map(|r| Self::line_as_bits(self.row(r)).map(|sig| (r, sig)))
            .collect()
    }

    /// Get all complete column signatures
    pub fn complete_col_signatures(&self) -> Vec<(usize, u32)> {
        (0..self.size)
            .filter_map(|c| Self::line_as_bits(&self.col(c)).map(|sig| (c, sig)))
            .collect()
    }

    /// Convert to Puzzle for serialization
    pub fn to_puzzle(&self, solution: &Grid) -> Puzzle {
        Puzzle {
            size: self.size,
            puzzle: self
                .cells
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|c| c.map(|b| if b { 1 } else { 0 }))
                        .collect()
                })
                .collect(),
            solution: solution
                .cells
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|c| if c.unwrap() { 1 } else { 0 })
                        .collect()
                })
                .collect(),
        }
    }
}