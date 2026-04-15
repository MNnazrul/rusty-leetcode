# 10. Regular Expression Matching

## Problem

Given an input string `s` and a pattern `p`, implement regular expression matching with support for:

- **`.`** — matches any single character
- **`*`** — matches zero or more of the preceding element

The matching must cover the **entire** input string (not just a prefix).

**Examples:**

| Input `s` | Input `p` | Output | Reason |
|-----------|-----------|--------|--------|
| `"aa"` | `"a"` | `false` | `"a"` matches only one `'a'`, not two |
| `"aa"` | `"a*"` | `true` | `'a'*` matches two `'a'`s |
| `"ab"` | `".*"` | `true` | `'.'*` matches any sequence |
| `"aab"` | `"c*a*b"` | `true` | `c*` → 0 c's, `a*` → 2 a's, `b` → `b` |

[LeetCode Link](https://leetcode.com/problems/regular-expression-matching/)

---

## Approach: 2-D Dynamic Programming — O(n·m)

### Idea

We build a 2-D boolean table `dp` where:

```
dp[i][j] = true  iff  s[0..i]  matches  p[0..j]
```

Think of `i` as "how many characters of `s` we've consumed" and `j` as "how many characters of `p` we've consumed."

### Base cases

| State | Value | Why |
|-------|-------|-----|
| `dp[0][0]` | `true` | empty string matches empty pattern |
| `dp[0][j]` (j ≥ 1) | depends | a pattern like `a*b*` can still match an empty string — only if every character is followed by `*` |

For the second base case we run a separate loop **before** the main DP:

```
j=1  p[0]='a'  → not '*', dp[0][1] = false
j=2  p[1]='*'  → dp[0][2] = dp[0][0] = true   ("a*" = zero a's)
j=3  p[2]='b'  → not '*', dp[0][3] = false
j=4  p[3]='*'  → dp[0][4] = dp[0][2] = true   ("b*" = zero b's)
```

### Transition rules

For each cell `dp[i][j]` we look at `p[j-1]` (the current pattern character):

**Case 1 — `p[j-1] == '*'`**

A `*` never stands alone; it always pairs with the character before it (`p[j-2]`).  
We have two sub-options, and the cell is `true` if **either** works:

- **Zero occurrences** of `p[j-2]`: pretend `x*` doesn't exist → `dp[i][j] = dp[i][j-2]`
- **One more occurrence** of `p[j-2]`: only valid when the current `s` character matches `p[j-2]`  
  (either an exact match or `p[j-2] == '.'`) → `dp[i][j] |= dp[i-1][j]`

**Case 2 — `p[j-1] == '.'` or `p[j-1] == s[i-1]`**

The pattern character matches the string character exactly (`.` matches anything).  
→ `dp[i][j] = dp[i-1][j-1]`

**Case 3 — everything else**

No match → `dp[i][j]` stays `false`.

---

### Dry Run

**Input:** `s = "aab"`, `p = "c*a*b"`

Table indices: rows = s characters (0 = empty), columns = p characters (0 = empty).

```
     ""   c   *   a   *   b
""  [ T   F   T   F   T   F ]
a   [ F   F   F   T   T   F ]
a   [ F   F   F   F   T   F ]
b   [ F   F   F   F   F   T ]
```

Answer: `dp[3][5] = true` ✓

Step-by-step highlights:

- `dp[0][2]`: `p[1]='*'` → zero c's → `dp[0][0]=T` → `T`
- `dp[0][4]`: `p[3]='*'` → zero a's → `dp[0][2]=T` → `T`
- `dp[1][4]`: `p[3]='*'`, `p[2]='a'` matches `s[0]='a'` → `dp[0][4]=T` → `T`
- `dp[2][4]`: same logic, `s[1]='a'` matches → `dp[1][4]=T` → `T`
- `dp[3][5]`: `p[4]='b'` matches `s[2]='b'` → `dp[2][4]=T` → `T`

---

### Code Walkthrough

```rust
let mut dp = vec![vec![false; m + 1]; n + 1];
dp[0][0] = true;
```

Allocate `(n+1) × (m+1)` table; `dp[0][0]` is the only guaranteed base case.

```rust
for j in 1..=m {
    if p[j - 1] == '*' {
        dp[0][j] = dp[0][j - 2];
    }
}
```

Handle the "empty string vs pattern" base cases: a `*` pattern can collapse to zero occurrences.

```rust
for i in 1..=n {
    for j in 1..=m {
        if p[j - 1] == '*' {
            dp[i][j] = dp[i][j - 2];                          // zero occurrences
            if p[j - 2] == '.' || p[j - 2] == s[i - 1] {
                dp[i][j] = dp[i][j] || dp[i - 1][j];          // one more occurrence
            }
        } else if p[j - 1] == '.' || p[j - 1] == s[i - 1] {
            dp[i][j] = dp[i - 1][j - 1];                      // single char match
        }
    }
}
```

Fill table left-to-right, top-to-bottom.  
`dp[i-1][j]` (not `dp[i-1][j-1]`) for the "one more" case because `*` can keep consuming — staying on column `j` lets us reuse the same `x*` for additional characters.

```rust
dp[n][m]
```

The bottom-right cell: does all of `s` match all of `p`?

---

### Complexity

| | |
|---|---|
| **Time** | O(n · m) — fill every cell of the table once |
| **Space** | O(n · m) — the DP table |

---

## Rust Solution

```rust
{{#include ../../../src/p0010_regular_expression_matching.rs}}
```
