# Contributing

Thanks for your interest in contributing! Here's how you can help.

## Adding a New Solution

1. Create a solution file in `src/` following the naming convention: `p{number}_{problem_name}.rs`
   ```
   src/p0042_trapping_rain_water.rs
   ```

2. Register the module in `src/lib.rs`:
   ```rust
   pub mod p0042_trapping_rain_water;
   ```

3. Include test cases using inline `#[cfg(test)]` blocks:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_example() {
           // Use examples from the LeetCode problem
       }
   }
   ```

4. Add a tutorial in `book/src/problems/`:
   - Create `p{number}_{problem_name}.md`
   - Include: problem statement, approach explanation, dry run, code walkthrough, and complexity analysis
   - Add the entry to `book/src/SUMMARY.md`

5. Add a row to the problems table in `README.md`

## Running Tests

```bash
# Run all tests
cargo test

# Run a specific problem's tests
cargo test p0042
```

## Building the Book

```bash
cd book && mdbook serve
```

## Before Submitting a PR

Run these checks locally before opening a PR — CI will run the same checks:

```bash
cargo fmt --check    # formatting
cargo clippy         # linting
cargo test           # tests
cd book && mdbook build  # book builds correctly
```

## Pull Request Rules

- Create a **separate branch** for each problem (e.g., `p0042-trapping-rain-water`)
- Each PR should contain **only one problem** — do not combine multiple problems in a single PR
- All CI checks must pass before merging

## Guidelines

- Write clean, idiomatic Rust
- Every solution must have tests using LeetCode's example cases
- Tutorials should be beginner-friendly with clear explanations
- Keep all content in English
