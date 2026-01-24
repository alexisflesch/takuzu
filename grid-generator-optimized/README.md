# Takuzu Grid Generator

A high-performance Rust-based generator for Takuzu (also known as Binairo or Binary Sudoku) puzzles with configurable difficulty levels.

## What is Takuzu?

Takuzu is a logic puzzle where you fill a grid with 0s and 1s following these rules:
- Each row and column must contain exactly half 0s and half 1s
- No more than two identical numbers can be adjacent in any row or column
- No two rows or columns can be identical

## How It Works

The generator creates puzzles through a multi-stage process designed to produce valid, solvable grids with precise difficulty control.

### 1. Complete Grid Generation

The generator starts by creating fully solved Takuzu grids that satisfy all rules:

- **Precomputation of Valid Rows**: For a given grid size `n`, precomputes all possible row patterns with exactly `n/2` ones and zeros, no three identical numbers in a row.
- **Constraint-Based Filling**: Uses backtracking to fill the grid row by row, ensuring:
  - Column balance (exactly `n/2` ones and zeros per column)
  - No three identical numbers vertically adjacent
  - All rows are unique
- **Parallel Generation**: Generates multiple source grids in parallel using Rayon for performance.

### 2. Grid Variants

For each source grid, the generator creates multiple variants using safe transformations that preserve validity:
- Value swapping (0 ↔ 1)
- Rotations (90°, 180°, 270°)
- Horizontal and vertical mirroring

### 3. Puzzle Creation (Digging)

The core difficulty control happens in the "digging" phase, where cells are selectively removed to create the puzzle. The goal is to dig the grid **as much as possible** while satisfying two strict conditions:
- The puzzle has exactly one unique solution.
- The puzzle's difficulty level remains **stable** (does not exceed the target difficulty).

#### Digging Algorithm

The digging process uses a progressive, stability-focused approach:

1. **Target Fill Ratio**: Sets a threshold of emptiness based on difficulty and grid size. Larger grids (18x18+) maintain a higher density (approx. 50%+) to support valid human reasoning paths.
2. **Stable Removal**: Shuffles all cell positions and attempts to remove them one by one. After each removal:
   - Verifies uniqueness via a backtracking solver.
   - Verifies human solvability via the `HumanSolver` using a progressive strategy.
3. **Difficulty Stability**: If removing a cell makes the puzzle unsolvable for the target level (meaning it now requires a higher level of reasoning), the cell is restored and the process continues with the next position.
4. **Maximized Emptiness**: The process continues until the target ratio is reached OR no more cells can be removed without increasing the difficulty beyond the allowed level.

#### Progressive Difficulty Evaluation

Difficulty is measured as the **minimal level of reasoning** required to fully solve the grid. The solver works in progressive stages, mimicking a human player:

1. Start with only **Level 1** rules enabled. Apply repeatedly until stable.
2. If the grid is not solved, enable **Level 2** rules and continue.
3. Crucially, if a higher-level rule makes progress, the solver **restarts from Level 1** rules to ensure every cell is filled using the simplest possible logic first.
4. The final difficulty of the puzzle is the lowest level at which the grid becomes fully solvable.

#### Difficulty Levels and Techniques

- **Level 1 (Basic Deduction)**: Avoid triplets, simple counting balance, basic and extended sandwich patterns, extended triplet avoidance. (Depth 0: looking at 3-5 cells).
- **Level 2 (Line Reasoning)**: Anticipation (forced moves), line uniqueness checks, near-duplicate detection, forced line completion based on counts. (Depth 1: whole line reasoning).
- **Level 3 (Hypothesis)**: Single-cell contradiction (trying one value and finding a logic error). (Depth 1: single hypothesis).
- **Level 4 (Advanced Hypothesis)**: Controlled multi-step hypotheses. (Depth 2: multiple steps of reasoning/lookahead).

## Automatic Routing and "Try Harder" mode

The generator automatically routes puzzles to their **true difficulty** files. Even if you request 10 puzzles at Difficulty 4, the generator will evaluate each one's minimal reasoning path.

If you use the `--try-harder` flag:
1. The generator will generate more candidate grids.
2. The "digging" process will ignore target fill ratios and try to remove as many cells as possible.
3. Only puzzles that **actually require** the target difficulty (or higher) will be kept.

## Usage

```bash
cargo build --release
./target/release/takuzu-gen --size 12 --count 10 --difficulty 4 --try-harder --output ./grids
```

Parameters:
- `--size`: Grid size (even number ≥ 4)
- `--count`: Number of puzzles to generate
- `--difficulty`: Difficulty level (1-4)
- `--try-harder`: Optional flag to force puzzles to reach the target difficulty level.
- `--output`: Output directory (default: current directory)

## Output Format

Puzzles are saved as JSON files with the format `takuzu_{size}x{size}_d{difficulty}.json`:

```json
[
  {
    "size": 6,
    "puzzle": [
      [null, 0, null, 1, null, null],
      [1, null, null, null, 0, null],
      ...
    ],
    "solution": [
      [1, 0, 1, 1, 0, 0],
      [1, 1, 0, 0, 0, 1],
      ...
    ]
  }
]
```

A summary file `grids_summary.json` tracks all generated puzzles.

## Performance

The generator is optimized for performance:
- Parallel grid generation and variant processing.
- Bitmask-based grid representation for ultra-fast checks.
- Adaptive timeouts based on grid size to prevent hangs on large grids.
- Efficient uniqueness checking using an optimized backtracking engine.

## Future Improvements

While the generator is now highly accurate in its difficulty classification, potential enhancements include:
- Implementing even more advanced human solving techniques (e.g., recursive X-chains).
- Adding difficulty calibration based on actual human solve times or complexity of the reasoning path.
- Further optimizing the source grid generation for very large grids (20x20+).

## Architecture

- `generator.rs`: Main generation orchestration
- `digger.rs`: Puzzle creation through cell removal
- `grid.rs`: Grid data structure with bitmask operations
- `solver/`: Solving engines
  - `backtrack_solver.rs`: Brute-force uniqueness verification
  - `human_solver.rs`: Technique-based solving for difficulty control
  - `techniques.rs`: Individual solving techniques
- `transform.rs`: Grid variant generation
- `export.rs`: JSON serialization and summary management</content>
<parameter name="filePath">/home/aflesch/takuzu/grid-generator-optimized/README.md