# 16. 3Sum Closest

## Problem

Given an integer array `nums` of length `n` and an integer `target`, find three integers in `nums` such that the sum is **closest to `target`**. Return the **sum** of the three integers.

You may assume that each input would have **exactly one solution**.

**Examples:**

| Input | Target | Output | Reason |
|-------|--------|--------|--------|
| `[-1, 2, 1, -4]` | `1` | `2` | `-1 + 2 + 1 = 2` is closest to `1` |
| `[0, 0, 0]` | `1` | `0` | `0 + 0 + 0 = 0` is the only option |

[LeetCode Link](https://leetcode.com/problems/3sum-closest/)

---

## Intuition

For each element, we need to find a pair among the remaining elements whose sum brings the total closest to `target`. A brute-force approach examines every triple in `O(n³)`. Sorting the array first unlocks a two-pointer scan that reduces the inner search to `O(n)`, giving an overall `O(n²)` solution.

---

## Approach: Sort + Two Pointers

### Algorithm

1. **Sort** `nums`.
2. Initialize `best_sum` to the first triple and `short_distance` to its distance from `target`.
3. For each anchor index `i` from `0` to `n - 3`:
   - Skip duplicate anchors (`nums[i] == nums[i - 1]`).
   - **Early-exit (too large):** if the minimum possible triple for this `i` — `nums[i] + nums[i+1] + nums[i+2]` — already exceeds `target`, then all future triples are at least as large. Update `best_sum` if this triple is closer and stop.
   - **Skip (too small):** if the maximum possible triple for this `i` — `nums[i] + nums[l-1] + nums[l-2]` — is still below `target`, update `best_sum` if closer and advance to the next anchor.
   - **Two-pointer sweep:** set `left = i + 1` and `right = n - 1`. While `left < right`:
     - Compute `sum = nums[i] + nums[left] + nums[right]`.
     - If `sum` is closer to `target` than `best_sum`, update `best_sum`.
     - If `sum == target`, return immediately.
     - If `sum > target`, move `right` left (reduce the sum).
     - If `sum < target`, move `left` right (increase the sum).
4. Return `best_sum`.

---

### Dry Run

Input: `[-1, 2, 1, -4]`, target = `1`

After sorting: `[-4, -1, 1, 2]`

Initial: `best_sum = -4 + -1 + 1 = -4`, `short_distance = |-4 - 1| = 5`

**i = 0** (`nums[0] = -4`):
- Min triple: `-4 + -1 + 1 = -4` < target → not early-exit
- Max triple: `-4 + 2 + 1 = -1` < target → update: `|-1 - 1| = 2 < 5` → `best_sum = -1`, `short_distance = 2`. Skip.

**i = 1** (`nums[1] = -1`):
- Min triple: `-1 + 1 + 2 = 2` > target (`1`) → check: `|2 - 1| = 1 < 2` → return `2`.

Answer: **2**. ✅

---

### Code Walkthrough

```rust
nums.sort_unstable();
let l = nums.len();
let mut best_sum = nums[0] + nums[1] + nums[2];
let mut short_distance = (best_sum - target).abs();
```

Sort first. Seed `best_sum` with the first triple so we always have a valid answer.

```rust
for i in 0..l - 2 {
    if i > 0 && nums[i] == nums[i - 1] { continue; }
```

Skip duplicate anchors — they would produce the same set of candidates.

```rust
    let sum = nums[i] + nums[i + 1] + nums[i + 2];
    if sum > target {
        if (sum - target).abs() < short_distance { return sum; }
        else { break; }
    }
```

`nums[i] + nums[i+1] + nums[i+2]` is the **smallest** triple anchored at `i` (array is sorted). If it already exceeds `target`, every subsequent triple does too — pick the closest and exit.

```rust
    let sum = nums[i] + nums[l - 1] + nums[l - 2];
    if sum < target {
        if (sum - target).abs() < short_distance {
            best_sum = sum;
            short_distance = (sum - target).abs();
        }
        continue;
    }
```

`nums[i] + nums[l-1] + nums[l-2]` is the **largest** triple anchored at `i`. If it still falls short of `target`, the two-pointer loop cannot find a closer answer for this anchor — skip to the next `i`.

```rust
    while left < right {
        let sum = nums[i] + nums[left] + nums[right];
        let distance = sum - target;
        if distance.abs() < short_distance {
            best_sum = sum; short_distance = distance.abs();
        }
        match distance.signum() {
            1  => right -= 1,
            -1 => left += 1,
            0  => return target,
            _  => unreachable!(),
        }
    }
```

Standard two-pointer: overshoot → shrink right, undershoot → grow left, exact hit → return immediately.

---

### Edge Cases

| Case | Behavior |
|------|----------|
| `target` far above all triples | Early-large-exit triggers immediately after seeding with the max triple |
| `target` far below all triples | Each anchor hits the too-small branch; `best_sum` is updated to the minimum triple overall |
| Exact match exists | `distance == 0` returns `target` without scanning further |
| Duplicate values | Anchor-skip skips repeated anchors; inner two-pointer naturally processes duplicates without extra logic |

### Complexity

| | |
|---|---|
| **Time** | O(n²) — O(n log n) sort + O(n) two-pointer scan per anchor |
| **Space** | O(1) — sorting in-place; only a handful of scalars |

---

## Rust Solution

```rust
{{#include ../../../src/p0016_3sum_closest.rs}}
```
