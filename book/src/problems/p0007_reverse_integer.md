# 7. Reverse Integer

## Problem

Given a signed 32-bit integer `x`, return `x` with its digits reversed. If reversing `x` causes the value to go outside the signed 32-bit integer range `[-2³¹, 2³¹ - 1]`, return `0`.

**Examples:**

| Input | Output | Reason |
|-------|--------|--------|
| `123` | `321` | straightforward reversal |
| `-123` | `-321` | sign is preserved |
| `120` | `21` | trailing zero is dropped |
| `1534236469` | `0` | reversed value overflows i32 |

[LeetCode Link](https://leetcode.com/problems/reverse-integer/)

## Approach: Digit-by-Digit Extraction — O(log n)

### Idea

We extract digits from the right side of the number one at a time using `% 10`, build the reversed number by multiplying the accumulator by 10 and adding each digit, and **check for overflow before each multiplication** so we never actually overflow.

The sign is handled separately: strip it at the start, work with a positive number, and reapply the sign at the end.

### Dry Run

```
Input: x = -123

Step 0 — strip sign:
  flag = true, x = 123, ans = 0

Step 1 — x = 123:
  i = 123 % 10 = 3
  x = 123 / 10 = 12
  overflow check: 0 > 214748364? No → safe
  ans = 0 * 10 + 3 = 3

Step 2 — x = 12:
  i = 12 % 10 = 2
  x = 12 / 10 = 1
  overflow check: 3 > 214748364? No → safe
  ans = 3 * 10 + 2 = 32

Step 3 — x = 1:
  i = 1 % 10 = 1
  x = 1 / 10 = 0
  overflow check: 32 > 214748364? No → safe
  ans = 32 * 10 + 1 = 321

Loop ends (x == 0).

Reapply sign: ans = -321
Return: -321
```

---

```
Input: x = 1534236469

flag = false, x = 1534236469, ans = 0

... (several steps) ...

At some point ans = 964324351, next digit i = 5:
  overflow check: 964324351 > 214748364? YES → return 0
```

### Code Walkthrough

```rust
if x < 0 {
    x *= -1;
    flag = true;
}
```
We convert the number to positive and remember the sign with `flag`. This way the digit-extraction loop works the same for positive and negative numbers.

---

```rust
let i = x % 10;
x /= 10;
```
`% 10` gives us the **last digit**. `/ 10` removes that digit from `x`. Repeating this in a loop extracts every digit from right to left.

For example, with `x = 123`:
- Round 1: `i = 3`, `x = 12`
- Round 2: `i = 2`, `x = 1`
- Round 3: `i = 1`, `x = 0` → loop ends

---

```rust
if ans > i32::MAX / 10 || (ans == i32::MAX / 10 && i > i32::MAX % 10) {
    return 0;
}
```
This is the **overflow guard** — the most important part of this problem.

`i32::MAX` is `2_147_483_647`.

The next step would do `ans = ans * 10 + i`. If `ans > 214_748_364`, then `ans * 10` already overflows. If `ans == 214_748_364`, it only overflows when `i > 7` (since `i32::MAX % 10 == 7`).

We check **before** multiplying, so we never actually overflow the integer.

---

```rust
ans = ans * 10 + i;
```
Standard digit reversal: shift the existing result left (multiply by 10) and append the new digit.

---

```rust
if flag {
    ans *= -1;
}
ans as i32
```
Reapply the sign if the original number was negative.

### Why Not Use i64?

A common alternative is to cast to `i64`, reverse, then check if the result fits in `i32`. That is simpler code but this solution avoids any wider integer type — it guards against overflow using only `i32` arithmetic. Both approaches are correct.

### Key Rust Concepts

- **`i32::MAX`** — The constant `2_147_483_647` (`2³¹ - 1`). Using the constant instead of a magic number makes the overflow check self-documenting.
- **`i32::MAX / 10` and `i32::MAX % 10`** — Computed at compile time; no runtime cost.
- **`ans as i32`** — Since `ans` is already an `i32` here, this cast is a no-op. It exists to make the return type explicit.

### Complexity

| | |
|---|---|
| **Time** | O(log n) — the loop runs once per digit, and a 32-bit integer has at most 10 digits |
| **Space** | O(1) — a fixed number of variables regardless of input size |

## Rust Solution

```rust
{{#include ../../../src/p0007_reverse_integer.rs}}
```
