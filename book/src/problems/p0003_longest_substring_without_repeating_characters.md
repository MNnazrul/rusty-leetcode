# 3. Longest Substring Without Repeating Characters

## Problem

Given a string `s`, find the length of the **longest substring** without repeating characters.

[LeetCode Link](https://leetcode.com/problems/longest-substring-without-repeating-characters/)

## Approach: Sliding Window with HashMap — O(n)

### Idea

We maintain a **sliding window** over the string using a HashMap that maps each character to its most recent index. The variable `latest` tracks the rightmost index of any repeated character we've seen — essentially the left boundary of our current valid window.

For each character at index `i`:
1. If we've seen this character before, update `latest` to be the max of its current value and the previous index of this character
2. Record the character's current index in the map
3. The current window length is `i - latest` — update the answer if this is larger

### Dry Run

```
s = "abcabcbb"

i=0, c='a': map={a:0}, latest=-1, window=0-(-1)=1, ans=1
i=1, c='b': map={a:0,b:1}, latest=-1, window=1-(-1)=2, ans=2
i=2, c='c': map={a:0,b:1,c:2}, latest=-1, window=2-(-1)=3, ans=3
i=3, c='a': seen at 0, latest=max(-1,0)=0, map={a:3,b:1,c:2}, window=3-0=3, ans=3
i=4, c='b': seen at 1, latest=max(0,1)=1, map={a:3,b:4,c:2}, window=4-1=3, ans=3
i=5, c='c': seen at 2, latest=max(1,2)=2, map={a:3,b:4,c:5}, window=5-2=3, ans=3
i=6, c='b': seen at 4, latest=max(2,4)=4, map={a:3,b:6,c:5}, window=6-4=2, ans=3
i=7, c='b': seen at 6, latest=max(4,6)=6, map={a:3,b:7,c:5}, window=7-6=1, ans=3

Result: 3 ("abc") ✓
```

### Code Walkthrough

```rust
let mut map = HashMap::new();
let mut latest = -1;
let mut ans = 0;
```
`map` stores each character's most recent index. `latest` is initialized to -1, representing the position just before the string starts — meaning the entire string is initially a valid window. `ans` tracks the maximum window size found.

```rust
for (i, c) in s.chars().enumerate() {
```
We iterate through each character with its index. `chars()` gives us an iterator over Unicode scalar values, and `enumerate()` pairs each with its position.

```rust
if let Some(&prev) = map.get(&c) {
    latest = std::cmp::max(latest, prev as i32);
}
```
If we've seen this character before, `prev` is the index where it last appeared. We update `latest` to the maximum of its current value and `prev`. The `max` is important — without it, we might accidentally shrink the window backwards past a different repeated character.

```rust
map.insert(c, i);
ans = std::cmp::max(ans, i as i32 - latest);
```
We update the map with the character's current position. The valid window spans from `latest + 1` to `i`, so its length is `i - latest`. We update `ans` if this window is the largest we've seen.

### Key Rust Concepts

- **`HashMap::get(&key)`** — Returns `Option<&V>`. The `&` in `Some(&prev)` destructures the reference to get the value directly.
- **`s.chars().enumerate()`** — Chains two iterators: `chars()` produces Unicode characters, `enumerate()` adds indices.
- **`std::cmp::max`** — Returns the larger of two values. Used instead of `if`/`else` for cleaner comparisons.

### Complexity

| | |
|---|---|
| **Time** | O(n) — single pass through the string |
| **Space** | O(min(n, m)) — where m is the character set size (at most 26 for lowercase English) |

## Rust Solution

```rust
{{#include ../../../src/p0003_longest_substring_without_repeating_characters.rs}}
```
