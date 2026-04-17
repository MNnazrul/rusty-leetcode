# 11. Container With Most Water

## Problem

You are given an integer array `height` of length `n`. There are `n` vertical lines drawn such that the two endpoints of the `i`-th line are `(i, 0)` and `(i, height[i])`.

Find two lines that, together with the x-axis, form a **container** that holds the most water. Return the **maximum amount of water** the container can store.

> You may **not** tilt the container.

**Examples:**

| Input | Output | Reason |
|-------|--------|--------|
| `[1,8,6,2,5,4,8,3,7]` | `49` | pick the `8` at index 1 and the `7` at index 8 → `min(8, 7) * (8 - 1) = 7 * 7 = 49` |
| `[1,1]` | `1` | `min(1, 1) * (1 - 0) = 1` |

[LeetCode Link](https://leetcode.com/problems/container-with-most-water/)

---

## Intuition

The amount of water trapped between two lines at indices `l` and `r` is:

```
area = min(height[l], height[r]) * (r - l)
```

- The **width** is the horizontal distance `r - l`.
- The **height** is limited by the **shorter** of the two lines — water would spill over the shorter side otherwise.

A naive solution tries every pair `(l, r)` — that's `O(n²)`. We can do better.

---

## Approach: Two Pointers — O(n)

### The Key Insight

Start with the **widest** container possible: `l = 0` and `r = n - 1`. The width is maximal here. From this position, any move inward reduces the width by `1`. To compensate for losing width, we need to find a **taller** line.

Here is the crucial observation:

> **The shorter line is the bottleneck.** If we move the pointer at the taller line inward, the area can never increase — the new width is smaller, and the height is still capped by the same short line (or becomes even shorter).

So the only move that has a chance of improving the answer is to **discard the shorter line** and move its pointer inward, hoping to find a taller line.

### Algorithm

1. Set `l = 0`, `r = height.len() - 1`, `ans = 0`.
2. While `l < r`:
   - Compute `area = min(height[l], height[r]) * (r - l)`.
   - Update `ans = max(ans, area)`.
   - If `height[l] < height[r]`, increment `l`. Otherwise, decrement `r`.
3. Return `ans`.

Each iteration moves exactly one pointer, and the pointers meet after `n - 1` steps — hence `O(n)`.

---

### Why Is It Safe to Discard the Shorter Line?

Suppose `height[l] < height[r]`. Consider any container that uses line `l` paired with some line `k` where `l < k < r`. Its area is:

```
min(height[l], height[k]) * (k - l)
    ≤ height[l] * (k - l)
    ≤ height[l] * (r - l)      -- because (k - l) < (r - l)
    = area at (l, r)
```

So **no container using the shorter line `l` can be better** than the one we just measured. We can safely drop `l` from consideration and advance.

---

### Dry Run

Input: `[1, 8, 6, 2, 5, 4, 8, 3, 7]` (length 9)

| Step | l | r | h[l] | h[r] | width | area | ans | Move |
|------|---|---|------|------|-------|------|-----|------|
| 1 | 0 | 8 | 1 | 7 | 8 | **8** | 8 | `h[l] < h[r]` → l++ |
| 2 | 1 | 8 | 8 | 7 | 7 | **49** | 49 | `h[l] ≥ h[r]` → r-- |
| 3 | 1 | 7 | 8 | 3 | 6 | 18 | 49 | r-- |
| 4 | 1 | 6 | 8 | 8 | 5 | 40 | 49 | r-- |
| 5 | 1 | 5 | 8 | 4 | 4 | 16 | 49 | r-- |
| 6 | 1 | 4 | 8 | 5 | 3 | 15 | 49 | r-- |
| 7 | 1 | 3 | 8 | 2 | 2 | 4 | 49 | r-- |
| 8 | 1 | 2 | 8 | 6 | 1 | 6 | 49 | r-- |
| — | 1 | 1 | — | — | — | — | — | loop ends |

Answer: **49**. ✅

---

### Code Walkthrough

```rust
let mut l = 0;
let mut r = height.len() - 1;
let mut ans = 0;
```

Two pointers start at the outermost lines, and `ans` tracks the best area seen so far.

```rust
while l < r {
    ans = std::cmp::max(ans, std::cmp::min(height[l], height[r]) * ((r - l) as i32));
```

- `std::cmp::min(height[l], height[r])` — the limiting height (water can't rise above the shorter line).
- `(r - l) as i32` — the width. `r` and `l` are `usize`, so we cast to `i32` to match the element type for multiplication.
- `std::cmp::max(ans, ...)` — keep the running best.

```rust
    if height[l] < height[r] {
        l += 1;
    } else {
        r -= 1;
    }
}
```

Move the pointer sitting on the **shorter** line inward. When the two lines are equal, either choice works — moving either pointer discards a line that cannot beat the current area (same proof as above applies symmetrically).

```rust
ans
```

Return the best area found.

---

### Edge Cases

| Case | Behavior |
|------|----------|
| `[1, 1]` | Loop runs once: `min(1,1) * 1 = 1`. Returns `1`. |
| Strictly increasing `[1,2,3,4,5]` | Left pointer keeps advancing; best area `6` from `(0, 4)` initially, then we move inward and nothing beats it. |
| Tall edges, flat middle `[8,1,1,1,1,1,1,8]` | First step: `min(8,8) * 7 = 56` — the answer, found immediately. |

### Complexity

| | |
|---|---|
| **Time** | O(n) — each pointer moves at most `n` steps total |
| **Space** | O(1) — only a handful of scalars |

---

## Rust Solution

```rust
{{#include ../../../src/p0011_container_with_most_water.rs}}
```
