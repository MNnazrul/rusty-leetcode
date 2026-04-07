# 5. Longest Palindromic Substring

## Problem

Given a string `s`, return the **longest palindromic substring** in `s`.

[LeetCode Link](https://leetcode.com/problems/longest-palindromic-substring/)

## Approach: Manacher's Algorithm — O(n)

### Idea

A naive approach would check every possible center and expand outward — O(n²). Manacher's algorithm does it in **O(n)** by reusing previously computed palindrome lengths.

The key insight: if we're inside a known palindrome (bounded by center `c` and right edge `r`), then the mirror of the current position around `c` already tells us the minimum palindrome radius at the current position. We only need to expand beyond what we already know.

To handle both odd and even-length palindromes uniformly, we transform the string by inserting `#` between every character (and sentinel characters `$` and `^` at the boundaries to avoid bounds checks):

```
"babad" → "$#b#a#b#a#d#^"
```

Now every palindrome is odd-length in the transformed string, and `p[i]` stores the radius of the palindrome centered at `i`.

### Dry Run

```
s = "babad"
t = ['$', '#', 'b', '#', 'a', '#', 'b', '#', 'a', '#', 'd', '#', '^']
         0    1    2    3    4    5    6    7    8    9   10   11   12

Processing i=2 ('b'): expand → p[2]=1
Processing i=4 ('a'): expand → p[4]=3  (covers b#a#b)
Processing i=6 ('b'): i < r=7, mirror=2, p[2]=1, min(1,1)=1
  → no further expansion, p[6]=1
Processing i=8 ('a'): expand → p[8]=3  (covers b#a#b#a → wait, let's check)
  actually p[8]=1, since 'd' ≠ 'b'

Max p[i] = p[4] = 3, center_index = 4
start = (4 - 3) / 2 = 0
result = s[0..3] = "bab" ✓
```

### Code Walkthrough

```rust
let mut t = Vec::new();
t.push('$');
for ch in s.chars() {
    t.push('#');
    t.push(ch);
}
t.push('#');
t.push('^');
```
Transforms `s` into the augmented string. The sentinel characters `$` and `^` are chosen to never match any real character, so the `while` loop in the expansion step never reads out of bounds — no explicit bounds check needed.

```rust
let mut c = 0;
let mut r = 0;
```
`c` is the center of the rightmost palindrome found so far, `r` is its right boundary.

```rust
let mirror = 2 * c as isize - i as isize;

if i < r {
    if mirror >= 0 {
        p[i] = std::cmp::min((r - i) as i32, p[mirror as usize]);
    }
}
```
If `i` is within the current rightmost palindrome (`i < r`), its mirror `2*c - i` has the same palindrome radius, up to the boundary `r - i`. We take the minimum so we don't claim more than what's been verified.

```rust
while t[i + (p[i] + 1) as usize] == t[i - (p[i] + 1) as usize] {
    p[i] += 1;
}
```
Attempt to expand further. Starts from the already-known radius and only does new comparisons — this is what gives Manacher's its linear time.

```rust
if i + p[i] as usize > r {
    c = i;
    r = i + p[i] as usize;
}
```
Update `c` and `r` when we've found a palindrome that extends further right.

```rust
let start = (center_index - max_len as usize) / 2;
s.chars().skip(start).take(max_len as usize).collect()
```
Converts back from transformed-string coordinates to original string coordinates. The `/2` accounts for the `#` separators.

### Key Rust Concepts

- **`Vec<char>`** — We build the transformed string as a `Vec<char>` for O(1) indexed access. Rust strings are UTF-8 and don't support direct indexing.
- **`isize` for mirror** — The mirror index `2*c - i` can temporarily be negative, so we compute it as `isize` before using it as a `usize` index.
- **Sentinel characters** — `$` and `^` act as boundary guards, eliminating the need for bounds checks in the expansion loop. This is a common pattern for simplifying array traversal code.
- **`.skip().take().collect()`** — Idiomatic Rust for extracting a substring by character position, which correctly handles multi-byte Unicode characters.

### Complexity

| | |
|---|---|
| **Time** | O(n) — each character is expanded at most once across all iterations |
| **Space** | O(n) — for the transformed string `t` and the `p` array |

## Rust Solution

```rust
{{#include ../../../src/p0005_longest_palindromic_substring.rs}}
```
