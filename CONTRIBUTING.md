# Contributing

Thanks for your interest in contributing! Here's how you can help.

## Project Structure

```
rusty-leetcode/
├── src/               # Rust solutions
│   ├── lib.rs         # Module declarations
│   └── p0001_two_sum.rs
├── book/              # Tutorials (mdbook)
│   ├── book.toml
│   └── src/
│       ├── SUMMARY.md
│       └── problems/
│           └── p0001_two_sum.md
├── Cargo.toml
├── README.md
├── CONTRIBUTING.md
└── LICENSE
```

## Setup

1. **Fork** this repository on GitHub (click the "Fork" button on the top right)

2. **Clone your fork** and set up the upstream remote:
   ```bash
   git clone git@github.com:<your-username>/rusty-leetcode.git
   cd rusty-leetcode
   git remote add upstream git@github.com:MNnazrul/rusty-leetcode.git
   ```

3. **Create a new branch** for the problem you're solving:
   ```bash
   git checkout -b p0042-trapping-rain-water
   ```

4. **Verify everything works**:
   ```bash
   cargo test
   ```

To view tutorials in the browser:

```bash
cargo install mdbook
cd book && mdbook serve
```

## Adding a New Solution

1. Create a solution file in `src/` following the naming convention: `p{number}_{problem_name}.rs`
   ```
   src/p0042_trapping_rain_water.rs
   ```
   Every solution file must start with `pub struct Solution;`:
   ```rust
   pub struct Solution;

   impl Solution {
       // your solution here
   }
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

## Before Submitting a PR

Run these checks locally before opening a PR — CI will run the same checks:

```bash
cargo fmt --check    # formatting
cargo clippy         # linting
cargo test           # tests
cd book && mdbook build  # book builds correctly
```

## Avoiding Merge Conflicts

Three files are touched by every PR: `src/lib.rs`, `README.md`, and `book/src/SUMMARY.md`. If another problem is merged while your PR is open, you'll get a conflict in one or more of these files.

To resolve it:

```bash
git fetch upstream
git rebase upstream/main
```

Then open the conflicting file and add your line after the existing entries — the order should match the problem number. The conflict will always be a simple sequential addition, never overlapping logic.

## Pull Request Rules

- **Fork** the repo and work on your fork — do not push directly to the main repository
- Create a **separate branch** for each problem (e.g., `p0042-trapping-rain-water`)
- Each PR should contain **only one problem** — do not combine multiple problems in a single PR
- Push to your fork and open a PR against `MNnazrul/rusty-leetcode:main`
- All CI checks must pass before merging

## Guidelines

- Write clean, idiomatic Rust
- Every solution must have tests using LeetCode's example cases
- Tutorials should be beginner-friendly with clear explanations
- Keep all content in English
