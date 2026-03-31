# 1. Two Sum

## Problem

Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to `target`.

Each input has **exactly one solution**, and you may not use the same element twice.

[LeetCode Link](https://leetcode.com/problems/two-sum/)

## Approach: HashMap — O(n)

### Idea

A brute force approach would check every pair — O(n²). But we don't need to check every pair.

When we're at `nums[i]`, we need another number equal to `target - nums[i]`. The question is — **have we seen this number before?**

To answer this instantly, we maintain a **HashMap** where the key is the number and the value is its index. For each element:

1. Check if `target - nums[i]` exists in the HashMap
2. If yes — we found our answer, return both indices
3. If no — insert the current number into the HashMap

### Dry Run

```
nums = [2, 7, 11, 15], target = 9

i=0: value=2, need=7, map={} → 7 not found → map={2:0}
i=1: value=7, need=2, map={2:0} → 2 found at index 0! → return [0, 1] ✓
```

### Why this works

We traverse left to right in a single pass. For each number, we do an O(1) HashMap lookup. Total: **O(n) time, O(n) space**.

### Code Walkthrough

```rust
let mut marks = HashMap::new();
```
Create an empty HashMap to store previously seen numbers and their indices.

```rust
for (idx, value) in nums.iter().enumerate() {
```
`enumerate()` gives us both the index and value at each step.

```rust
match marks.get(&(target - value)) {
    Some(&prev) => {
        return vec![prev as i32, idx as i32];
    }
    None => { }
}
```
`target - value` is the complement — the number needed to reach the target. If found in the map, we return `prev` (previous index) and `idx` (current index). Rust's `match` handles pattern matching — `Some` means found, `None` means not found.

```rust
marks.insert(value, idx);
```
If the complement wasn't found, insert the current value into the map so future iterations can find it.

### Complexity

| | |
|---|---|
| **Time** | O(n) — single pass |
| **Space** | O(n) — for the HashMap |

## Rust Solution

```rust
{{#include ../../../src/p0001_two_sum.rs}}
```
