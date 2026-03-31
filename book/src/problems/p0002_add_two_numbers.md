# 2. Add Two Numbers

## Problem

You are given two non-empty linked lists representing two non-negative integers. The digits are stored in **reverse order**, and each node contains a single digit. Add the two numbers and return the sum as a linked list.

[LeetCode Link](https://leetcode.com/problems/add-two-numbers/)

## Approach: Iterative with Carry — O(n)

### Idea

Since the digits are already in reverse order (least significant digit first), we can add them just like we do by hand — digit by digit from left to right, carrying over when the sum exceeds 9.

We use a **dummy head node** to simplify list construction. A `carry` variable tracks overflow between digits. The loop continues as long as there are remaining nodes in either list **or** there's a carry left.

At each step:
1. Sum up the values from both lists (if available) plus the carry
2. Create a new node with `sum % 10`
3. Update the carry to `sum / 10`

### Dry Run

```
l1: 2 → 4 → 3       (represents 342)
l2: 5 → 6 → 4       (represents 465)

Step 1: sum = 2 + 5 + 0 = 7,  carry = 0, node = 7
Step 2: sum = 4 + 6 + 0 = 10, carry = 1, node = 0
Step 3: sum = 3 + 4 + 1 = 8,  carry = 0, node = 8

Result: 7 → 0 → 8   (represents 807) ✓
```

### Code Walkthrough

```rust
let mut main_list = Box::new(ListNode::new(0));
let mut carry = 0;
let mut temp_list = &mut main_list;
```
We create a **dummy head** node (`main_list`) with value 0. This dummy node makes it easy to build the result list without special-casing the first node. `temp_list` is a mutable reference that always points to the current tail of the result list.

```rust
while l1.is_some() || l2.is_some() || carry != 0 {
```
The loop runs as long as there are digits left in either list or there's a remaining carry. This handles cases where the lists have different lengths or the final addition produces a carry (e.g., 999 + 1 = 1000).

```rust
let mut sum = carry;

if let Some(node) = l1 {
    sum += node.val;
    l1 = node.next;
} else {
    l1 = None;
}
```
Start with the carry from the previous step. If `l1` has a node, add its value to `sum` and advance `l1` to the next node. The `if let Some(node)` pattern destructures the `Option<Box<ListNode>>` — if it's `Some`, we get the inner node; if `None`, we skip. The same logic applies to `l2`.

```rust
carry = sum / 10;
temp_list.next = Some(Box::new(ListNode::new(sum % 10)));
temp_list = temp_list.next.as_mut().unwrap();
```
Integer division (`sum / 10`) gives us the carry for the next iteration (0 or 1). The remainder (`sum % 10`) is the digit for this position. We create a new node, attach it to the tail, and advance `temp_list` to this new node.

```rust
main_list.next
```
We return `main_list.next` to skip the dummy head node, giving us the actual result list.

### Key Rust Concepts

- **`Option<Box<ListNode>>`** — Rust's way of representing nullable linked list nodes. `Box` provides heap allocation, `Option` handles the null case.
- **`if let Some(node) = l1`** — Pattern matching that destructures and moves ownership of the inner value.
- **`as_mut().unwrap()`** — Converts `&mut Option<Box<ListNode>>` to `&mut Box<ListNode>`, allowing us to mutate the next pointer.

### Complexity

| | |
|---|---|
| **Time** | O(max(n, m)) — where n and m are the lengths of the two lists |
| **Space** | O(max(n, m)) — for the result list |

## Rust Solution

```rust
{{#include ../../../src/p0002_add_two_numbers.rs}}
```
