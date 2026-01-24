# Takuzu Solver & Generator – AI Agent Instructions

You are working on a **Takuzu (Binary Puzzle)** generator and solver.

The goal is to support **five difficulty levels (1 to 5)**, where difficulty is defined **by the maximum depth of logical reasoning required**, not by grid size, randomness, or brute force.

---

## Core constraints

* ❌ No global backtracking
* ❌ No recursive hypotheses
* ✅ All solving must remain *human-like* and explainable

---

## Allowed reasoning techniques

### Depth 0 — No hypothesis (pure deduction)

These rules never branch and must always produce deterministic deductions.

* Avoid three identical values in a row or column
* Line/column balance (equal number of 0s and 1s)
* Basic sandwich patterns: `X . X`
* Extended sandwich patterns: `X . . X`
* Extended triplet avoidance using sliding windows of size 4–5
* Local paired cells reasoning based on counting constraints

---

### Depth 1 — Single-step anticipation (no assumption)

These techniques reason at the **line or column level**, but still without assuming a cell value.

* Line and column uniqueness checks
* Near-duplicate rows or columns (differing in only a few cells)
* Forced line or column completion
* Anticipation limited to completing a single line or column

---

### Depth 1 — Single-cell hypothesis

A *single* hypothetical value may be tested for a cell.

Procedure:

1. Assume a value for one cell
2. Apply **only depth-0 rules** repeatedly
3. If a contradiction occurs, the opposite value is forced

Constraints:

* No second hypothesis allowed
* No recursion

---

### Depth 2 — Controlled multi-step hypothesis

This is the maximum allowed reasoning depth.

* One initial hypothesis only
* Full logical propagation (depth-0 and depth-1 rules)
* Contradictions may appear **after stabilization**, not necessarily immediately

Strictly forbidden:

* Hypothesis inside another hypothesis
* Branching search or backtracking

---

## Difficulty levels

Difficulty is defined **only** by the maximum reasoning depth required to solve the grid.

### Level 1

* Depth 0 only
* Immediate local logic
* No line comparisons

### Level 2

* Depth 0
* Extended local patterns (larger windows, extended sandwiches)

### Level 3

* Depth 1 (line-level reasoning)
* No hypotheses

### Level 4

* Depth 1 with single-cell hypothesis
* Contradictions may be delayed

### Level 5

* Controlled depth-2 reasoning
* No backtracking
* No recursive hypotheses

---

## Grid generation process

1. Generate a complete valid Takuzu grid
2. Remove cells progressively (digging)
3. After each removal, verify:

   * The solution remains **unique**
   * The puzzle is solvable **using only the rules allowed** for the selected difficulty

Difficulty must come from **reasoning depth**, not from excessive emptiness.

---

## Performance considerations

* Grid sizes ≥ 16×16 are expensive
* Prefer fewer removals with deeper reasoning for higher difficulties
* Validation must stop as soon as a rule exceeding the difficulty level is required

---

End goal: produce puzzles that feel challenging, fair, and *human-solvable*, with a clear correspondence between difficulty level and cognitive effort.
