# 8. String to Integer (atoi)

## Problem

Implement the `myAtoi(string s)` function, which converts a string to a 32-bit signed integer.

The algorithm:
1. Skip leading whitespace.
2. Read an optional `+` or `-` sign.
3. Read digits until a non-digit character or end of string.
4. Clamp the result to `[-2³¹, 2³¹ - 1]` if it overflows.

**Examples:**

| Input | Output | Reason |
|-------|--------|--------|
| `"42"` | `42` | straightforward |
| `"   -042"` | `-42` | leading spaces, leading zeros |
| `"1337c0d3"` | `1337` | stops at non-digit |
| `"0-1"` | `0` | stops at `-` after first digit |
| `"words and 987"` | `0` | no leading digits |
| `"99999999999"` | `2147483647` | clamped to i32::MAX |

[LeetCode Link](https://leetcode.com/problems/string-to-integer-atoi/)

## Approach: Linear Scan with i64 Accumulator — O(n)

### Idea

We scan the string left to right:

1. **Trim** leading whitespace (Rust's `.trim()` handles this).
2. **Read the sign** from `s[0]` — if it's `-` we remember `is_pos = false`, then advance past the sign character.
3. **Accumulate digits** into an `i64`. Using `i64` lets us collect the raw number without worrying about overflow mid-loop — we clamp to `i32` range at the end.
4. **Stop** as soon as we see a non-digit character.
5. **Apply sign**, then **clamp** to `[i32::MIN, i32::MAX]` using `min`/`max`.
6. **Cast** to `i32` and return.

### Dry Run

```
Input: "   -042"

After trim: s = ['-', '0', '4', '2']
is_pos = false   (s[0] == '-')
start  = 1       (skip the sign)

Iteration 1 — s[1] = '0':  ans = 0*10 + 0 = 0,  start = 2
Iteration 2 — s[2] = '4':  ans = 0*10 + 4 = 4,  start = 3
Iteration 3 — s[3] = '2':  ans = 4*10 + 2 = 42, start = 4

Loop ends (start == s.len()).

Apply sign: ans = -42
Clamp: -42 is within [-2147483648, 2147483647] → no change
Return: -42
```

---

```
Input: "99999999999"

After trim: s = ['9','9','9','9','9','9','9','9','9','9','9']
is_pos = true, start = 0

The loop accumulates until ans > (1 << 31) = 2147483648,
at which point the while condition fails and we stop early.

ans at that point is larger than i32::MAX.
Clamp: min(ans, 2147483647) = 2147483647
Return: 2147483647
```

### Code Walkthrough

```rust
let s: Vec<char> = s.trim().chars().collect();
```
`.trim()` strips leading and trailing ASCII whitespace in one call. We collect into a `Vec<char>` for O(1) indexed access.

---

```rust
let is_pos = s[0] != '-';
let mut start = 0;
if s[0] == '-' || s[0] == '+' {
    start = 1;
}
```
We only treat `'-'` as negative; everything else (including `'+'`) is positive. We advance `start` past the sign so the digit loop doesn't try to parse it.

---

```rust
while ans <= (1 << 31) && start < s.len() && s[start].is_ascii_digit() {
    ans = ans * 10 + s[start].to_digit(10).unwrap() as i64;
    start += 1;
}
```
Three guard conditions:
- `ans <= (1 << 31)` — stop accumulating once we've exceeded `i32::MAX` in magnitude (the clamp handles the exact boundary).
- `start < s.len()` — bounds check.
- `s[start].is_ascii_digit()` — stop at the first non-digit.

We use `i64` so intermediate values can exceed `i32` range without panic.

---

```rust
ans = std::cmp::min(ans, (1 << 31) - 1);
ans = std::cmp::max(ans, -(1 << 31));
```
Clamp to `[i32::MIN, i32::MAX]`. Because `ans` is `i64`, `1 << 31` is computed as an `i64` shift — no overflow here.

### Key Rust Concepts

- **`.trim()`** — strips leading/trailing whitespace; returns a `&str` view with no allocation.
- **`.is_ascii_digit()`** — true for `'0'`..=`'9'`; more precise than `.is_numeric()`, which also matches Unicode digit characters.
- **`to_digit(10)`** — converts a char to its decimal value; returns `Option<u32>`. Safe to `.unwrap()` here because we already checked `is_ascii_digit()`.
- **`i64` accumulator** — avoids overflow during parsing; clamping to `i32` range happens once at the end.

### Complexity

| | |
|---|---|
| **Time** | O(n) — single pass through the string |
| **Space** | O(n) — the `Vec<char>` copy of the input |

## Rust Solution

```rust
{{#include ../../../src/p0008_string_to_integer_atoi.rs}}
```
