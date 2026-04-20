# 12. Integer to Roman

## Problem

Roman numerals are written using seven symbols:

| Symbol | Value |
|--------|-------|
| I | 1 |
| V | 5 |
| X | 10 |
| L | 50 |
| C | 100 |
| D | 500 |
| M | 1000 |

Numerals are normally written from largest to smallest, left to right, and added together — `XVI = 10 + 5 + 1 = 16`.

There are six **subtractive** exceptions where a smaller numeral precedes a larger one and is subtracted:

| Pair | Value |
|------|-------|
| `IV` | 4 |
| `IX` | 9 |
| `XL` | 40 |
| `XC` | 90 |
| `CD` | 400 |
| `CM` | 900 |

Given an integer `num` where `1 <= num <= 3999`, convert it to a Roman numeral.

**Examples:**

| Input | Output | Reason |
|-------|--------|--------|
| `3749` | `"MMMDCCXLIX"` | `3000 + 700 + 40 + 9 = MMM + DCC + XL + IX` |
| `58` | `"LVIII"` | `50 + 5 + 3 = L + V + III` |
| `1994` | `"MCMXCIV"` | `1000 + 900 + 90 + 4 = M + CM + XC + IV` |

[LeetCode Link](https://leetcode.com/problems/integer-to-roman/)

---

## Intuition

If we only had the seven base symbols, the problem would be trivial: repeatedly take out the largest value that fits. The twist is the six subtractive pairs (`IV`, `IX`, `XL`, `XC`, `CD`, `CM`) — without them we'd emit `IIII` instead of `IV`.

The clean trick: **treat each subtractive pair as its own symbol**. Now we have 13 "symbols" (7 originals + 6 pairs) that together cover every integer from 1 to 3999 without overlap or ambiguity. Greedy reduction becomes correct.

---

## Approach: Greedy With Expanded Symbol Table — O(1)

### The Key Insight

List all 13 value/symbol pairs in **descending order of value**:

```
1000→"M"   900→"CM"   500→"D"   400→"CD"
 100→"C"    90→"XC"    50→"L"    40→"XL"
  10→"X"     9→"IX"     5→"V"     4→"IV"
   1→"I"
```

For each pair, find how many times its value fits into what's left of `num`, emit that many copies of the symbol, then take the remainder. Because the table includes every subtractive exception, greedy never gets stuck producing an invalid form like `IIII`.

### Algorithm

1. Define parallel arrays `values` (descending) and `symbols`.
2. Walk the table in order. For each `(value, symbol)`:
   - `count = num / value` — how many times the symbol appears.
   - Append `symbol` that many times.
   - `num %= value`.
3. Return the built string.

The outer loop runs exactly 13 times; the inner loop runs at most 3 times per symbol (no Roman numeral ever repeats a symbol four times — the subtractive pair intervenes). So the total work is bounded by a small constant.

---

### Why Does Greedy Work?

The expanded table is carefully designed so that, at every point, taking the largest value that fits is optimal:

- Between `1000` (`M`) and `900` (`CM`): any `num ≥ 900` but `< 1000` must start with `CM`, not `DCCC` or anything else.
- Between `500` (`D`) and `400` (`CD`): the only way to write 400–499 is `CD` followed by the rest.
- ... and so on down the table.

Each descending pair `(x, x')` with `x' < x` is placed so that no combination of **smaller** symbols can match or beat `x'` without using an invalid repeat. In other words, the table encodes exactly one canonical Roman representation per integer, and the greedy walk reconstructs it.

---

### Dry Run

Input: `1994`

| Step | value | symbol | count = num/value | emit | num after `%=` | ans |
|------|-------|--------|-------------------|------|----------------|-----|
| 1 | 1000 | M  | 1 | `M`  | 994 | `M` |
| 2 | 900  | CM | 1 | `CM` | 94  | `MCM` |
| 3 | 500  | D  | 0 | —    | 94  | `MCM` |
| 4 | 400  | CD | 0 | —    | 94  | `MCM` |
| 5 | 100  | C  | 0 | —    | 94  | `MCM` |
| 6 | 90   | XC | 1 | `XC` | 4   | `MCMXC` |
| 7 | 50   | L  | 0 | —    | 4   | `MCMXC` |
| 8 | 40   | XL | 0 | —    | 4   | `MCMXC` |
| 9 | 10   | X  | 0 | —    | 4   | `MCMXC` |
| 10 | 9   | IX | 0 | —    | 4   | `MCMXC` |
| 11 | 5   | V  | 0 | —    | 4   | `MCMXC` |
| 12 | 4   | IV | 1 | `IV` | 0   | `MCMXCIV` |
| 13 | 1   | I  | 0 | —    | 0   | `MCMXCIV` |

Answer: **`MCMXCIV`**. ✅

---

### Code Walkthrough

```rust
let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
let symbols = [
    "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
];
```

Two parallel arrays: index `i` in `values` corresponds to index `i` in `symbols`. Descending order is what makes the greedy walk correct.

```rust
let mut ans = String::new();
let mut num = num;
```

Output buffer and a mutable shadow of `num` so we can reduce it in place.

```rust
for i in 0..values.len() {
    let count = num / values[i];
    for _ in 0..count {
        ans.push_str(symbols[i]);
    }
    num %= values[i];
}
```

- `count = num / values[i]` — integer division tells us how many copies of this symbol fit.
- Inner `for _ in 0..count` appends the symbol `count` times. We don't need the loop variable; `_` avoids an unused-variable lint.
- `num %= values[i]` reduces `num` to the remainder, ready for the next (smaller) symbol.

```rust
ans
```

Return the assembled numeral.

---

### Edge Cases

| Case | Behavior |
|------|----------|
| `1` | Only the final row hits: `count = 1`, emit `I`. |
| `4` | Row `(4, IV)` fires with `count = 1`; `I` row sees `num = 0`. |
| `3999` (max) | `MMM` + `CM` + `XC` + `IX` = `MMMCMXCIX`. The inner loop peaks at 3 iterations (for `M`). |
| `3749` | `MMM` + `DCC` + `XL` + `IX`. Demonstrates `D` (500) then two `C` (100s), not `CD` — greedy picks 500 before touching 400. |

### Complexity

| | |
|---|---|
| **Time** | O(1) — the outer loop is a constant 13 iterations, and for `num ≤ 3999` the inner loop adds at most 12 more symbol writes total across the whole run |
| **Space** | O(1) extra — output string length is bounded by 15 characters for `num ≤ 3999` |

---

## Rust Solution

```rust
{{#include ../../../src/p0012_integer_to_roman.rs}}
```
