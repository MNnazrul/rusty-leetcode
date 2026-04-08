pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // Edge case: no zigzag needed
        if num_rows == 1 {
            return s;
        }

        let num_rows = num_rows as usize;

        // Create rows
        let mut rows: Vec<String> = vec![String::new(); num_rows];

        let mut curr_row = 0;
        let mut going_down = true;

        for ch in s.chars() {
            rows[curr_row].push(ch);

            // Change direction at boundaries
            if curr_row == 0 {
                going_down = true;
            } else if curr_row == num_rows - 1 {
                going_down = false;
            }

            // Move row pointer
            if going_down {
                curr_row += 1;
            } else {
                curr_row -= 1;
            }
        }

        // Combine all rows
        rows.concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // "PAYPALISHIRING" with 3 rows → "PAHNAPLSIIGYIR"
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
    }

    #[test]
    fn test_example_2() {
        // "PAYPALISHIRING" with 4 rows → "PINALSIGYAHRPI"
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
    }

    #[test]
    fn test_single_row() {
        assert_eq!(Solution::convert("A".to_string(), 1), "A");
    }

    #[test]
    fn test_two_rows() {
        // "AB" with 2 rows → "AB"
        assert_eq!(Solution::convert("AB".to_string(), 2), "AB");
    }

    #[test]
    fn test_num_rows_exceeds_length() {
        // More rows than characters — no zigzag effect
        assert_eq!(Solution::convert("ABC".to_string(), 10), "ABC");
    }
}
