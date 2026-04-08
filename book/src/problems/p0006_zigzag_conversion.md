# 6. Zigzag Conversion

## Problem

The string `"PAYPALISHIRING"` is written in a zigzag pattern on a given number of rows like this:

```
P   A   H   N
A P L S I I G
Y   I   R
```

And then read line by line: `"PAHNAPLSIIGYIR"`.

Given a string `s` and an integer `numRows`, return the string written in zigzag pattern then read line by line.

[LeetCode Link](https://leetcode.com/problems/zigzag-conversion/)

## Approach: Row Simulation — O(n)

### Idea

Instead of figuring out the mathematical mapping, we simulate the process directly:

- Maintain one `String` per row.
- Walk through the characters of `s`, appending each character to the current row.
- Move the row pointer **down** until we hit the last row, then **up** until we hit the first row, and so on — this is the zigzag motion.
- At the end, concatenate all rows.

### Dry Run

```
s = "PAYPALISHIRING", numRows = 3

Row 0: P   A   H   N
Row 1: A P L S I I G
Row 2: Y   I   R

Step-by-step (curr_row, going_down):
P → row 0, down → curr_row=1
A → row 1, down → curr_row=2
Y → row 2, flip up → curr_row=1
P → row 1, up → curr_row=0
A → row 0, flip down → curr_row=1
L → row 1, down → curr_row=2
I → row 2, flip up → curr_row=1
...

Concatenated: "PAHNAPLSIIGYIR" ✓
```

### Code Walkthrough

```rust
if num_rows == 1 {
    return s;
}
```
With only one row there is no zigzag — return early to avoid an off-by-one in the direction logic (`num_rows - 1` would be 0, equal to `curr_row == 0`, causing an infinite direction flip).

```rust
let mut rows: Vec<String> = vec![String::new(); num_rows];
let mut curr_row = 0;
let mut going_down = true;
```
One `String` bucket per row. `going_down` tracks the current direction of travel.

```rust
if curr_row == 0 {
    going_down = true;
} else if curr_row == num_rows - 1 {
    going_down = false;
}
```
Direction is flipped **after** the character is placed, right at the boundary rows. This means every boundary character is placed once (not duplicated), and the pointer reverses cleanly.

```rust
rows.concat()
```
`concat()` on a `Vec<String>` allocates a single `String` and appends each row in order — one allocation for the final result.

### Key Rust Concepts

- **`Vec<String>`** — A vector of owned strings, one per row. Each `push` on a `String` amortizes to O(1).
- **`vec![String::new(); n]`** — Creates `n` independent empty strings. Note: `String` does not implement `Copy`, so `vec![expr; n]` evaluates the expression **once** then clones it `n-1` times via `Clone`. For an empty `String` this is cheap (no heap data to copy), so all `n` slots end up as independent empty strings.
- **`rows.concat()`** — Idiomatic way to join a `Vec<String>` into one `String` without a separator.

### Complexity

| | |
|---|---|
| **Time** | O(n) — each character is visited exactly once |
| **Space** | O(n) — the row buffers collectively hold all characters |

## Rust Solution

```rust
{{#include ../../../src/p0006_zigzag_conversion.rs}}
```
