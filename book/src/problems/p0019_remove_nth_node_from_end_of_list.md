# 19. Remove Nth Node From End of List

## Problem

Given the head of a linked list, remove the `n`-th node from the **end** of the list and return its head.

**Examples:**

| Input | n | Output |
|-------|---|--------|
| `[1,2,3,4,5]` | `2` | `[1,2,3,5]` |
| `[1]` | `1` | `[]` |
| `[1,2]` | `1` | `[1]` |

**Constraints:**

- The number of nodes in the list is `sz`.
- `1 <= sz <= 30`
- `0 <= Node.val <= 100`
- `1 <= n <= sz`

[LeetCode Link](https://leetcode.com/problems/remove-nth-node-from-end-of-list/)

---

## Intuition

Here's the core challenge: linked lists don't have random access. You can't just say "give me the 3rd node from the end" without walking the list first, because you don't know where the end is until you get there.

The simplest honest solution is to **make two passes**:

1. Walk the entire list once to count how many nodes there are. Let's call that `tot`.
2. Now you know the node you want to remove is at position `tot - n` from the **front** (0-indexed). Walk again to that position and snip out the next node.

This is the two-pass approach — straightforward, easy to reason about, and it works perfectly. There's a fancier one-pass solution using two pointers, but the two-pass version is cleaner to read and reason about.

---

## Approach: Two-Pass with a Dummy Head — O(L)

### The Dummy Node Trick

Before diving in, let's talk about the **dummy node** pattern. It's a small but powerful trick for linked list problems.

Normally, removing a node requires a reference to its predecessor. But what about the **head node**? It has no predecessor. That means you'd have to write special-case code: "if we're removing the head, return `head.next`."

The dummy node sidesteps this entirely. You create a fake node whose `next` points to the real head:

```
dummy → 1 → 2 → 3 → 4 → 5 → None
```

Now *every* node in the list has a predecessor, including what was originally the head. The dummy node plays the role of "the node before the head." After you're done, you return `dummy.next` as the new head — which works whether or not the original head was removed.

### The Two-Pass Plan

**Pass 1 — Count:**
Walk from `head` to the end, incrementing a counter. When the loop ends, `tot` holds the total number of nodes.

**Pass 2 — Remove:**
Start from `dummy`. Advance `tot - n` steps. Now `cur` points to the node *just before* the one we want to remove. Rewire `cur.next` to skip over the target node.

```
Before:  cur → [target] → rest
After:   cur → rest
```

That's it. The target node gets dropped because nothing points to it anymore.

### Why `tot - n` steps?

Let's count from the front (0-indexed, starting at `dummy`):

```
dummy(0) → 1(1) → 2(2) → 3(3) → 4(4) → 5(5)
```

The `n`-th from the end is node at index `tot - n + 1` (1-indexed from front). Its predecessor is at index `tot - n`. So we walk `tot - n` steps from `dummy` to reach the predecessor, then remove `cur.next`.

For `[1,2,3,4,5]` with `n=2`:
- `tot = 5`, so walk `5 - 2 = 3` steps from dummy: dummy → 1 → 2 → 3
- `cur` is now node `3`, and `cur.next` is node `4` (the target)
- Skip it: `cur.next = cur.next.next` (which is node `5`)

---

### Dry Run

Input: `[1, 2, 3, 4, 5]`, `n = 2`

**Pass 1 — Counting:**

| Step | Node visited | tot |
|------|-------------|-----|
| 1 | 1 | 1 |
| 2 | 2 | 2 |
| 3 | 3 | 3 |
| 4 | 4 | 4 |
| 5 | 5 | 5 |

`tot = 5`

**Building the structure:**
```
dummy(val=0) → 1 → 2 → 3 → 4 → 5 → None
cur = dummy
```

**Pass 2 — Advancing `tot - n = 3` steps:**

| Step | cur moves to |
|------|-------------|
| 1 | node 1 |
| 2 | node 2 |
| 3 | node 3 |

`cur` now points to node `3`.

**Removal:**
```
cur.next        = node 4  (the target)
cur.next.next   = node 5  (what comes after)

next = cur.next.next.take()  →  next = Some(node 5), node 4's next is now None
cur.next = next              →  node 3 now points to node 5
```

**Result:**
```
dummy → 1 → 2 → 3 → 5 → None
return dummy.next → [1, 2, 3, 5] ✅
```

---

### Edge Case: Removing the Head

Input: `[1, 2, 3]`, `n = 3`

`tot = 3`, advance `3 - 3 = 0` steps — `cur` stays at `dummy`.

```
dummy → 1 → 2 → 3
cur = dummy
cur.next = node 1 (the target, since n equals length)
cur.next.next = node 2

next = node 2
cur.next = node 2
```

Result: `dummy → 2 → 3`, returns `[2, 3]` ✅

The dummy node made this work without any special case.

### Edge Case: Single Node List

Input: `[1]`, `n = 1`

`tot = 1`, advance `1 - 1 = 0` steps — `cur` stays at `dummy`.

```
dummy → 1 → None
cur.next = node 1
cur.next.next = None

next = None (from .take(), node 1's next is already None)
cur.next = None
```

Result: `dummy → None`, returns `None` ✅

---

### Code Walkthrough

```rust
let mut cur = &head;
let mut tot = 0;
while let Some(node) = cur {
    tot += 1;
    cur = &node.next;
}
```

First pass. `cur` is a shared reference that hops along the list. `while let Some(node) = cur` unpacks each `Option<Box<ListNode>>` — when it hits `None` (the end), the loop stops. After this, `tot` is the list length.

Notice that `cur` is a *shared* (`&`) reference here. We're just reading, not modifying. That's an important Rust detail: you can borrow `head` immutably here because we haven't moved it into `dummy` yet.

```rust
let mut dummy = Box::new(ListNode { val: 0, next: head });
```

Create the dummy node on the heap (`Box::new`) and move `head` into it as `dummy.next`. From this point on, `head` is owned by `dummy.next` — the original variable `head` is gone. The dummy acts as our sentinel node before the real list.

```rust
let mut cur = &mut dummy;
for _ in 0..(tot - n) {
    cur = cur.next.as_mut().unwrap();
}
```

Second pass. Now `cur` is a *mutable* reference so we can rewire links later. We advance exactly `tot - n` steps. `cur.next.as_mut()` converts `Option<Box<ListNode>>` to `Option<&mut Box<ListNode>>`, and `.unwrap()` extracts the mutable reference — safe here because the constraints guarantee `1 <= n <= sz`, so we'll never step off the end.

After this loop, `cur` points to the node just *before* the one we want to delete.

```rust
let next = cur.next.as_mut().unwrap().next.take();
cur.next = next;
```

This is the deletion in two steps:

1. `cur.next.as_mut().unwrap()` gets a mutable reference to the target node (the one being deleted).
2. `.next.take()` **takes** the target's `next` field — it moves the value out, replacing it with `None`. This serves two purposes: it gives us what comes after the target, and it severs the target from the rest of the list (preventing a dangling reference).
3. `cur.next = next` replaces `cur`'s next pointer with what used to be after the target. The target node is now unreachable — it will be dropped when it goes out of scope.

Why use `.take()` instead of just reading? Because Rust's ownership model doesn't allow you to move a value out of a `Box` that's behind a mutable reference without using something like `.take()`. The `take()` method on `Option<T>` swaps the value with `None` and returns the original value — it's the idiomatic way to extract ownership through a mutable reference.

```rust
dummy.next
```

Return the new head. If the original head was removed, `dummy.next` is now what was the second node. If not, `dummy.next` is still the original head. Either way, this single line handles both cases correctly.

---

### Complexity

| | |
|---|---|
| **Time** | O(L), where L is the number of nodes. We walk the list twice: once to count, once to find the predecessor. Two linear passes = O(2L) = O(L). |
| **Space** | O(1). We allocate one dummy node (constant), and all other variables are just pointers (references). No extra data structures, no recursion stack. |

---

## Rust Solution

```rust
{{#include ../../../src/p0019_remove_nth_node_from_end_of_list.rs}}
```
