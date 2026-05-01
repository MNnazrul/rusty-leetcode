# 13. Roman to Integer

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

There are six **subtractive** pairs where a smaller numeral precedes a larger one and is subtracted from it:

| Pair | Value |
|------|-------|
| `IV` | 4 |
| `IX` | 9 |
| `XL` | 40 |
| `XC` | 90 |
| `CD` | 400 |
| `CM` | 900 |

Given a Roman numeral string `s`, convert it to an integer. The input is guaranteed to be a valid Roman numeral in the range `[1, 3999]`.

**Examples:**

| Input | Output | Reason |
|-------|--------|--------|
| `"III"` | `3` | `I + I + I = 1 + 1 + 1` |
| `"LVIII"` | `58` | `L + V + III = 50 + 5 + 3` |
| `"MCMXCIV"` | `1994` | `M + CM + XC + IV = 1000 + 900 + 90 + 4` |

[LeetCode Link](https://leetcode.com/problems/roman-to-integer/)

---

## Intuition

This problem is the inverse of *Integer to Roman*. There, we converted a number into a string by repeatedly peeling off the largest value that fit. Here, we read a string and add up the values it represents.

The classic approach scans character-by-character with a "look ahead" rule: if the current symbol is smaller than the next one, subtract it; otherwise add it. That works, but it forces you to remember the six subtractive special cases at runtime.

There is a cleaner mental model that mirrors the Integer-to-Roman solution exactly: **treat each subtractive pair as its own multi-character symbol**. Now we have 13 symbols (`M`, `CM`, `D`, `CD`, `C`, `XC`, `L`, `XL`, `X`, `IX`, `V`, `IV`, `I`), each with a fixed value, and the input string is just a left-to-right concatenation of these 13 tokens in **non-increasing** order of value.

Walking the symbol table from largest to smallest and consuming as many copies of each as match the current prefix of the string reproduces the original number.

---

## Approach: Greedy Prefix Matching With Expanded Symbol Table — O(n)

### The Key Insight

Any valid Roman numeral in `[1, 3999]` can be written as a concatenation of these 13 tokens, in descending order of value, with each token repeated 0–3 times:

```
1000→"M"   900→"CM"   500→"D"   400→"CD"
 100→"C"    90→"XC"    50→"L"    40→"XL"
  10→"X"     9→"IX"     5→"V"     4→"IV"
   1→"I"
```

So we can decode by walking the table top-to-bottom and, at each step, **eating** as many copies of the current symbol as appear next in the input. Each match contributes its value to the running total.

### Algorithm

1. Define parallel arrays `values` (descending) and `symbols` — same as Integer to Roman.
2. Maintain a cursor `id` into `s`, starting at 0, and an accumulator `ans = 0`.
3. For each `(value, symbol)` pair in order:
   - While the substring of `s` starting at `id` begins with `symbol`:
     - Add `value` to `ans`.
     - Advance `id` by `symbol.len()`.
4. Return `ans`.

Because the table is in descending order and each token is matched as a whole prefix, we never confuse `IV` (4) with `I` followed by `V`: the `IV` row sits **above** the `I` row, so it consumes those two characters first.

---

### Why Does This Work?

The crucial property is that valid Roman numerals are unambiguous under this 13-symbol decomposition. At any position in `s`, exactly one of two things is true:

- The next characters are one of the 13 symbols at the current row of the table (or a later row).
- The next characters cannot match any earlier (larger) row, because the input is canonical.

So when we reach row `i`, every prefix that *could* match a row `< i` has already been consumed. The remaining string starts with a symbol of value `≤ values[i]`. The `while` loop greedily eats every row-`i` match it finds before moving on, which is correct because no later (smaller) row can produce more value than row `i` for the same characters.

---

### Dry Run

Input: `"MCMXCIV"` (expected: `1994`)

`id` starts at 0; `ans` starts at 0. The table walk:

| Row | symbol | s[id..] starts with? | matches | ans after | id after |
|-----|--------|----------------------|---------|-----------|----------|
| 1000 / `M`  | `M`  | `MCMXCIV` → yes (1) | 1 × 1000 | 1000 | 1 |
| 900 / `CM`  | `CM` | `CMXCIV` → yes (1)  | 1 × 900  | 1900 | 3 |
| 500 / `D`   | `D`  | `XCIV` → no         | 0        | 1900 | 3 |
| 400 / `CD`  | `CD` | `XCIV` → no         | 0        | 1900 | 3 |
| 100 / `C`   | `C`  | `XCIV` → no         | 0        | 1900 | 3 |
| 90 / `XC`   | `XC` | `XCIV` → yes (1)    | 1 × 90   | 1990 | 5 |
| 50 / `L`    | `L`  | `IV` → no           | 0        | 1990 | 5 |
| 40 / `XL`   | `XL` | `IV` → no           | 0        | 1990 | 5 |
| 10 / `X`    | `X`  | `IV` → no           | 0        | 1990 | 5 |
| 9 / `IX`    | `IX` | `IV` → no           | 0        | 1990 | 5 |
| 5 / `V`     | `V`  | `IV` → no           | 0        | 1990 | 5 |
| 4 / `IV`    | `IV` | `IV` → yes (1)      | 1 × 4    | 1994 | 7 |
| 1 / `I`     | `I`  | `""` → no           | 0        | 1994 | 7 |

Answer: **`1994`** ✅

---

### Code Walkthrough

```rust
let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
let symbols = [
    "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
];
```

The same parallel arrays as Integer to Roman, in descending order of value.

```rust
let mut ans = 0;
let mut id: usize = 0;
```

`ans` accumulates the integer value; `id` is a byte cursor into `s`. Roman numerals are pure ASCII, so byte indices and character indices coincide — no UTF-8 trouble.

```rust
for (idx, &sym) in symbols.iter().enumerate() {
    while id < s.len() && s[id..].starts_with(sym) {
        ans += values[idx];
        id += sym.len();
    }
}
```

- `s[id..]` slices from the cursor to the end of the string. `String` implements `Index<RangeFrom<usize>>`, so this returns a `&str`.
- `.starts_with(sym)` on `&str` checks whether `sym` is a prefix.
- On a hit, add the symbol's value and advance the cursor past those bytes.
- The `while` loop handles repetition (e.g. `MMM` for 3000): row `M` matches three times before the cursor moves past it.
- The outer `for` then moves to the next, smaller symbol.

```rust
ans
```

Return the total.

---

### Edge Cases

| Case | Behavior |
|------|----------|
| `"I"` | Only the last row matches once. Returns `1`. |
| `"IV"` | Row `(4, "IV")` matches once. The `(1, "I")` row sees an empty remainder. Returns `4`. |
| `"III"` | Row `(1, "I")` matches three times. Returns `3`. |
| `"MMMCMXCIX"` (3999, max) | Three matches for `M`, one each for `CM`, `XC`, `IX`. Returns `3999`. |
| `"MMMDCCXLIX"` (3749) | `MMM` then `D` then `CC` then `XL` then `IX`. Note `D` is consumed before `C` even though `CD` (400) is a row — `CD` doesn't match the prefix `DCC...`, so the walk falls through to `D`. |

### Complexity

| | |
|---|---|
| **Time** | O(n) where n = `s.len()`. The cursor advances by at least one byte per iteration that does work, and the outer loop is a constant 13 rows |
| **Space** | O(1) extra — only the cursor and accumulator |

---

## Rust Solution

```rust
{{#include ../../../src/p0013_roman_to_integer.rs}}
```
