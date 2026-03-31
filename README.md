# LeetCode Solutions in Rust

LeetCode problem solutions implemented in Rust, with detailed explanations.

## Project Structure

```
leetcode-c-n-rust/
├── src/               # Rust solutions
├── book/              # Explanations via mdbook (mdbook serve)
└── Cargo.toml
```

## How to Run

```bash
# Run all tests
cargo test

# Run a specific problem's test
cargo test p0001

# View explanations in browser
cd book && mdbook serve
```

## Problems

| #    | Problem | Difficulty | Tutorial | Solution |
|------|---------|------------|-------------|----------|
| 0001 | [Two Sum](https://leetcode.com/problems/two-sum/) | Easy | [tutorial](book/src/problems/p0001_two_sum.md) | [Rust](src/p0001_two_sum.rs) |
