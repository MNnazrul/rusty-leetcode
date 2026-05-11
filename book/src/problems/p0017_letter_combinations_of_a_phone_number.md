# 17. Letter Combinations of a Phone Number

## Problem

Given a string containing digits from `2` to `9`, return all possible letter combinations that the number could represent, in any order. The mapping follows the standard telephone keypad:

| Digit | Letters |
|-------|---------|
| 2 | a b c |
| 3 | d e f |
| 4 | g h i |
| 5 | j k l |
| 6 | m n o |
| 7 | p q r s |
| 8 | t u v |
| 9 | w x y z |

**Examples:**

| Input | Output |
|-------|--------|
| `"23"` | `["ad","ae","af","bd","be","bf","cd","ce","cf"]` |
| `""` | `[]` |
| `"2"` | `["a","b","c"]` |

**Constraints:**

- `0 <= digits.length <= 4`
- `digits[i]` is a digit in the range `['2', '9']`

[LeetCode Link](https://leetcode.com/problems/letter-combinations-of-a-phone-number/)

---

## Intuition

The combinations form a **Cartesian product**: for input `"23"`, you want every letter from `"abc"` paired with every letter from `"def"`. That's 3 × 3 = 9 combinations for two digits, 3 × 3 × 3 = 27 for three, and so on.

One clean way to compute a Cartesian product iteratively is to **grow a list one dimension at a time**:

1. Start with a list containing just the empty string: `[""]`.
2. For the first digit, replace every string in the list with all its letter-extensions: `[""] → ["a", "b", "c"]`.
3. For the second digit, do the same again: `["a","b","c"] → ["ad","ae","af","bd","be","bf","cd","ce","cf"]`.
4. Repeat for every remaining digit.

After processing all digits, the list holds exactly the complete set of combinations. This is essentially a **BFS level-by-level expansion**, where each digit represents one level and each level multiplies the size of the list by the number of letters for that digit.

---

## Approach: Iterative BFS Expansion — O(4ⁿ · n)

### The Big Picture

Process digits left to right. Maintain a running list `ans` of all combinations built so far. For each digit, create a new list by appending every letter of that digit to every string already in `ans`. Replace `ans` with the new list and continue.

```
ans = [""]

for each digit d:
    letters = map[d]                  // e.g. "abc" for digit 2
    temp = []
    for each string s in ans:
        for each letter l in letters:
            temp.push(s + l)
    ans = temp

return ans
```

### Why Start with `[""]`?

The empty string is the **identity element** for string concatenation. Starting the list with `[""]` means the first digit's loop works identically to every subsequent digit's loop — no special-casing needed. After the first digit the empty-string seed is gone and `ans` contains only single-letter strings.

### Growth Pattern

| After digit | List size | Example (`"23"`) |
|-------------|-----------|------------------|
| (start) | 1 | `[""]` |
| digit 1 ("2" = abc) | 3 | `["a","b","c"]` |
| digit 2 ("3" = def) | 9 | `["ad","ae","af","bd",…]` |

Each digit multiplies the list size by 3 or 4 (depending on whether the digit maps to 3 or 4 letters). The constraint `digits.length ≤ 4` keeps this bounded: at most 4⁴ = 256 combinations.

---

### Dry Run

Input: `"23"`

**Initialization:**
```
map = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"]
ans = [""]
```

**Digit `'2'` → `letters = "abc"`:**

| prev_str | letter | new string |
|----------|--------|------------|
| "" | 'a' | "a" |
| "" | 'b' | "b" |
| "" | 'c' | "c" |

`ans = ["a", "b", "c"]`

**Digit `'3'` → `letters = "def"`:**

| prev_str | letter | new string |
|----------|--------|------------|
| "a" | 'd' | "ad" |
| "a" | 'e' | "ae" |
| "a" | 'f' | "af" |
| "b" | 'd' | "bd" |
| "b" | 'e' | "be" |
| "b" | 'f' | "bf" |
| "c" | 'd' | "cd" |
| "c" | 'e' | "ce" |
| "c" | 'f' | "cf" |

`ans = ["ad","ae","af","bd","be","bf","cd","ce","cf"]` ✅

---

### Code Walkthrough

```rust
if digits.is_empty() {
    return Vec::new();
}
```

Empty input → empty output. Without this guard, the loop never runs and we'd return `[""]` — the seed — instead of `[]`.

```rust
let map = vec![
    "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
];
```

Direct index lookup: `map[2]` = `"abc"`, `map[7]` = `"pqrs"`, etc. Indices 0 and 1 are empty strings because digits `0` and `1` have no keypad letters. The constraint guarantees we'll never be asked about them, but the empty strings mean nothing bad happens if we do.

```rust
let mut ans = vec!["".to_string()];
```

The seed. Every expansion loop reads from `ans` and writes to a fresh `temp_vec`, then replaces `ans` with `temp_vec`. The seed disappears after the very first digit is processed.

```rust
for ch in digits.chars() {
    if let Some(digit) = ch.to_digit(10) {
        let letters = map[digit as usize];
        let mut temp_vec = Vec::new();

        for prev_str in &ans {
            for letter in letters.chars() {
                temp_vec.push(format!("{}{}", prev_str, letter));
            }
        }
        ans = temp_vec;
    }
}
```

The outer `for ch` loop walks digits left to right. `ch.to_digit(10)` converts the character `'2'` to the integer `2`, which we use to index into `map`. The double `for` loop is the Cartesian product: every existing string × every new letter. `format!` builds the extended string by appending `letter` to `prev_str`. After both inner loops finish, `ans` is replaced entirely — no partial state leaks between digits.

```rust
ans
```

Return the fully expanded list.

---

### Edge Cases

| Case | Behavior |
|------|----------|
| Empty input `""` | Early return `[]`. The seed `[""]` is never seen. |
| Single digit `"2"` | One expansion pass: `[""] → ["a","b","c"]`. |
| Digit `"7"` or `"9"` | `map[7] = "pqrs"` and `map[9] = "wxyz"` have four letters each; the inner loop just runs four iterations instead of three. No special handling needed. |
| Four digits `"2222"` | Four expansion passes, final list has 3⁴ = 81 strings. |

---

### Complexity

| | |
|---|---|
| **Time** | O(4ⁿ · n). There are at most 4ⁿ combinations (n = number of digits; 4 is the max letters per digit). Each combination has length n, and building it with `format!` costs O(n). So the total work is O(4ⁿ · n). With n ≤ 4 this is at most O(1024) — effectively constant. |
| **Space** | O(4ⁿ · n) for the output. The intermediate `temp_vec` is at most the same size as the final output. No extra data structures are used. |

---

## Rust Solution

```rust
{{#include ../../../src/p0017_letter_combinations_of_a_phone_number.rs}}
```
