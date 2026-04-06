# 4. Median of Two Sorted Arrays

## Problem

Given two sorted arrays `nums1` and `nums2` of size `m` and `n` respectively, return **the median** of the two sorted arrays. The overall run time complexity should be **O(log(m+n))**.

[LeetCode Link](https://leetcode.com/problems/median-of-two-sorted-arrays/)

## Approach: Find Kth Element via Binary Elimination — O(log(m+n))

### Idea

Instead of merging the arrays (which would be O(m+n)), we reduce the median problem to finding the **kth smallest element**. For an odd total length, we need the middle element; for even, we average the two middle elements.

To find the kth element efficiently, we use a **binary elimination** strategy: compare elements at position `k/2` in each array. The array with the smaller value cannot contain the kth element in its first `k/2` positions, so we eliminate them. Each step halves `k`, giving us O(log k) = O(log(m+n)) time.

### Dry Run

```
nums1 = [1, 3], nums2 = [2]
total = 3 (odd) → find kth where k = 2

find_kth_element([1,3], [2], i=0, j=0, k=2):
  p = k/2 = 1
  x = nums1[0] = 1
  y = nums2[0] = 2
  x < y → eliminate nums1[0..1], i=1, k=1

  k == 1 → return min(nums1[1], nums2[0]) = min(3, 2) = 2

Result: 2.0 ✓
```

```
nums1 = [1, 2], nums2 = [3, 4]
total = 4 (even) → find k=2 and k=3

find_kth_element(k=2):
  p=1, x=nums1[0]=1, y=nums2[0]=3
  x < y → i=1, k=1
  k==1 → return min(nums1[1], nums2[0]) = min(2, 3) = 2

find_kth_element(k=3):
  p=1, x=nums1[0]=1, y=nums2[0]=3
  x < y → i=1, k=2
  p=1, x=nums1[1]=2, y=nums2[0]=3
  x < y → i=2, k=1
  i >= n → return nums2[0 + 1 - 1] = nums2[0] = 3

Result: (2 + 3) / 2 = 2.5 ✓
```

### Code Walkthrough

```rust
pub fn find_kth_element(vec1: &[i32], vec2: &[i32], mut i: usize, mut j: usize, mut k: usize) -> i32 {
```
Takes two sorted slices with starting indices `i` and `j`, and finds the `k`th smallest element across both. Using `mut` parameters lets us update them in the loop without separate variables.

```rust
if i >= n { return vec2[j + k - 1]; }
if j >= m { return vec1[i + k - 1]; }
```
**Base cases**: if one array is exhausted, the kth element is simply at position `j + k - 1` (or `i + k - 1`) in the other array.

```rust
if k == 1 { return std::cmp::min(vec1[i], vec2[j]); }
```
When `k == 1`, we need the smallest remaining element — just compare the fronts.

```rust
let p = k / 2;

let x = if i + p - 1 < n { vec1[i + p - 1] } else { i32::MAX };
let y = if j + p - 1 < m { vec2[j + p - 1] } else { i32::MAX };
```
We look `p = k/2` positions ahead in each array. If an array doesn't have enough elements, we use `i32::MAX` so that array won't be chosen for elimination — we can only safely discard elements we can actually compare.

```rust
if x < y {
    i += p;
    k -= p;
} else {
    j += p;
    k -= p;
}
```
The array with the smaller value at position `p` cannot contain the kth element in its first `p` elements. We skip those elements by advancing the index and reduce `k` accordingly.

```rust
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let total = nums1.len() + nums2.len();

    if total % 2 == 1 {
        Self::find_kth_element(&nums1, &nums2, 0, 0, total / 2 + 1) as f64
    } else {
        let left = Self::find_kth_element(&nums1, &nums2, 0, 0, total / 2);
        let right = Self::find_kth_element(&nums1, &nums2, 0, 0, total / 2 + 1);
        (left as f64 + right as f64) / 2.0
    }
}
```
For odd-length combined arrays, the median is the middle element (position `total/2 + 1`). For even-length, it's the average of elements at positions `total/2` and `total/2 + 1`. We cast to `f64` for the division.

### Key Rust Concepts

- **`&[i32]` (slices)** — Borrowed views into arrays/vectors. More flexible than `&Vec<i32>` since they work with any contiguous sequence.
- **`i32::MAX`** — The maximum value for a 32-bit integer (2,147,483,647). Used as a sentinel to ensure an exhausted array is never chosen for elimination.
- **`mut` parameters** — Function parameters declared `mut` can be modified within the function without affecting the caller, avoiding the need for separate local variables.
- **Loop with early returns** — Using `loop` with `return` statements instead of recursion avoids stack overflow for large inputs and is idiomatic Rust for iterative refinement.

### Complexity

| | |
|---|---|
| **Time** | O(log(m+n)) — each iteration halves k |
| **Space** | O(1) — only uses a few index variables |

## Rust Solution

```rust
{{#include ../../../src/p0004_median_of_two_sorted_arrays.rs}}
```
