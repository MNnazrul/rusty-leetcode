# 9. Palindrome Number

## Problem

Given an integer `x`, return `true` if `x` is a palindrome, and `false` otherwise.

An integer is a palindrome when it reads the same forward and backward.

**Examples:**

| Input | Output | Reason |
|-------|--------|--------|
| `121` | `true` | reads the same both ways |
| `-121` | `false` | reads as `-121` forward, `121-` backward |
| `10` | `false` | reads as `01` backward |

[LeetCode Link](https://leetcode.com/problems/palindrome-number/)

## Approach: String Conversion — O(n)

### Idea

Convert the integer to its string representation, then compare it against the reversed string. If they are equal, the number is a palindrome.

1. **Convert** `x` to a `String` via `.to_string()`.
2. **Reverse** the characters with `.chars().rev()`.
3. **Collect** the reversed chars back into a `String`.
4. **Compare** the two strings and return the result.

### Dry Run

```
Input: 121

to_string()          → "121"
chars().rev()        → ['1', '2', '1']
collect::<String>()  → "121"

"121" == "121" → true
```

---

```
Input: -121

to_string()          → "-121"
chars().rev()        → ['1', '2', '1', '-']
collect::<String>()  → "121-"

"-121" == "121-" → false
```

### Code Walkthrough

```rust
x.to_string() == x.to_string().chars().rev().collect::<String>()
```

- **`.to_string()`** — converts the `i32` to its decimal string, including the `-` sign for negative numbers. Negative numbers can never be palindromes because the `-` appears only at the start.
- **`.chars()`** — yields an iterator over Unicode scalar values (each ASCII digit is one char).
- **`.rev()`** — reverses the iterator lazily — no extra allocation until we collect.
- **`.collect::<String>()`** — materializes the reversed iterator into a new `String`.

### Complexity

| | |
|---|---|
| **Time** | O(n) — where n is the number of digits |
| **Space** | O(n) — two strings of length n are allocated |

## Rust Solution

```rust
{{#include ../../../src/p0009_palindrome_number.rs}}
```
