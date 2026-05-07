# 14. Longest Common Prefix

## Problem

Write a function to find the **longest common prefix** string amongst an array of strings.

If there is no common prefix, return an empty string `""`.

**Examples:**

| Input | Output |
|-------|--------|
| `["flower","flow","flight"]` | `"fl"` |
| `["dog","racecar","car"]` | `""` |

**Constraints:**

- `1 <= strs.length <= 200`
- `0 <= strs[i].length <= 200`
- `strs[i]` consists of only lowercase English letters if it is non-empty.

[LeetCode Link](https://leetcode.com/problems/longest-common-prefix/)

---

## Intuition

A common prefix of the whole array is, by definition, a prefix of **every** string in it. So an upper bound on the answer is *any single string in the array* — the longest common prefix can never be longer than the shortest input.

That gives a clean strategy: pick one string as a **candidate** prefix, then walk through the rest of the array and shrink the candidate until it actually is a prefix of every string seen so far. When the loop ends, whatever survives is the answer.

The shrinking is one character at a time from the right. Each `pop()` makes the candidate strictly shorter, and the candidate can only ever shrink — never grow — so the work is bounded.

---

## Approach: Shrink-the-Candidate — O(S)

Where `S` is the total number of characters across all input strings.

### The Key Insight

Pick `strs[0]` as the initial candidate. For every other string `st`:

- If `st` already starts with the candidate, the candidate is still valid for `st` — move on.
- Otherwise, repeatedly drop the last character of the candidate until `st` does start with it.

The empty string is a prefix of every string, so the loop is guaranteed to terminate (worst case: candidate becomes `""`).

When all strings have been processed, the candidate is a prefix of every input — i.e. a common prefix. And because we only ever shrink (never re-extend), it is the longest one consistent with every string we have seen.

### Algorithm

1. Initialize `ans = strs[0].clone()`.
2. For each `st` in `strs`:
   - While `st` does not start with `ans`, `pop` the last character of `ans`.
3. Return `ans`.

Step 2 also iterates over `strs[0]` itself, which is harmless: a string always starts with itself, so the inner `while` does zero work.

---

### Why Does This Work?

Two invariants hold every time we finish handling a string `st`:

- **Invariant A — common prefix so far.** `ans` is a prefix of every string in `strs[0..=i]`. True initially (one element, `ans == strs[0]`), and preserved because we only stop popping once `st.starts_with(&ans)` is true; for earlier strings, popping characters off the end keeps `ans` a prefix of them too (any prefix of a prefix is a prefix).
- **Invariant B — maximality.** No longer string than the current `ans` is a prefix of every string in `strs[0..=i]`. The original `ans` was `strs[0]`, the maximum possible. Each `pop` happens only because the *previous* longer candidate failed `starts_with` for `st`, so no longer candidate could survive either.

After the last iteration, both invariants hold for the entire array, so `ans` is the longest common prefix.

---

### Dry Run

Input: `["flower", "flow", "flight"]` (expected: `"fl"`)

| step | st | starts_with(ans)? | pops | ans after |
|------|----|-------------------|------|-----------|
| init | — | — | — | `"flower"` |
| 1 | `"flower"` | yes | 0 | `"flower"` |
| 2 | `"flow"` | no | `r`, `e`, `w` → now `"flow"`, yes | `"flow"` |
| 3 | `"flight"` | no | `w`, `o` → now `"fl"`, yes | `"fl"` |

Answer: **`"fl"`** ✅

A second example with no common prefix — `["dog", "racecar", "car"]`:

| step | st | starts_with(ans)? | pops | ans after |
|------|----|-------------------|------|-----------|
| init | — | — | — | `"dog"` |
| 1 | `"dog"` | yes | 0 | `"dog"` |
| 2 | `"racecar"` | no | `g`, `o`, `d` → now `""`, yes | `""` |
| 3 | `"car"` | yes (every string starts with `""`) | 0 | `""` |

Answer: **`""`** ✅

---

### Code Walkthrough

```rust
let mut ans = strs[0].clone();
```

Take the first string as the starting candidate. The constraints guarantee `strs.len() >= 1`, so indexing `strs[0]` is safe. We `clone()` because we still want to iterate over `strs` afterward, including `strs[0]`.

```rust
for st in strs {
    while !st.starts_with(&ans) {
        ans.pop();
    }
}
```

- `for st in strs` consumes `strs` by value — fine here, we don't need it again.
- `st.starts_with(&ans)` takes a `&str`, and `&ans` (a `&String`) coerces to `&str` via Deref.
- `ans.pop()` removes the last `char` (not byte) from `ans` and returns `Option<char>`. The `Option` is ignored — when `ans` is already empty, `pop` returns `None` but the `while` condition is false anyway (every string starts with `""`), so the loop exits.

```rust
ans
```

Return the surviving prefix.

---

### Edge Cases

| Case | Behavior |
|------|----------|
| Single string `["alone"]` | The for-loop runs once; `"alone".starts_with("alone")` is true, no pops. Returns `"alone"`. |
| All identical | Every `starts_with` succeeds on the first check. Returns the common string. |
| First string is the shortest common prefix already (e.g. `["ab", "abc", "abcd"]`) | Candidate starts as `"ab"`; both later strings already start with it. Returns `"ab"`. |
| Empty string in the array (e.g. `["", "abc"]`) | When `st = ""`, the `while` pops every character of `ans` because `""` only starts with `""`. Returns `""`. |
| No common prefix (e.g. `["dog","racecar","car"]`) | Candidate is shrunk all the way to `""`. Returns `""`. |

### Complexity

| | |
|---|---|
| **Time** | O(S) where `S` is the total number of characters across all inputs. Each `pop` permanently removes a character from the candidate (the candidate never grows back), so total pops are bounded by `strs[0].len()`. Each `starts_with` call costs at most `ans.len()`, and the sum of those across the run is bounded by `S`. |
| **Space** | O(m) where `m = strs[0].len()`, for the `ans` buffer. No extra structures. |

---

## Rust Solution

```rust
{{#include ../../../src/p0014_longest_common_prefix.rs}}
```
