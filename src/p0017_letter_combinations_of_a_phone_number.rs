pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }

        let map = vec![
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];

        let mut ans = vec!["".to_string()];

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

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut v: Vec<String>) -> Vec<String> {
        v.sort();
        v
    }

    #[test]
    fn test_example_1() {
        // "23" → all combinations of "abc" × "def"
        let got = Solution::letter_combinations("23".to_string());
        let want = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        assert_eq!(
            sorted(got),
            sorted(want.iter().map(|s| s.to_string()).collect())
        );
    }

    #[test]
    fn test_example_2() {
        // empty string → empty result
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_example_3() {
        // single digit "2" → ["a","b","c"]
        let got = Solution::letter_combinations("2".to_string());
        let want = vec!["a", "b", "c"];
        assert_eq!(
            sorted(got),
            sorted(want.iter().map(|s| s.to_string()).collect())
        );
    }

    #[test]
    fn test_four_letter_digit() {
        // "7" maps to "pqrs" — four letters
        let got = Solution::letter_combinations("7".to_string());
        let want = vec!["p", "q", "r", "s"];
        assert_eq!(
            sorted(got),
            sorted(want.iter().map(|s| s.to_string()).collect())
        );
    }

    #[test]
    fn test_nine_maps_to_wxyz() {
        // "9" maps to "wxyz"
        let got = Solution::letter_combinations("9".to_string());
        let want = vec!["w", "x", "y", "z"];
        assert_eq!(
            sorted(got),
            sorted(want.iter().map(|s| s.to_string()).collect())
        );
    }

    #[test]
    fn test_three_digits() {
        // "234" → 3 × 3 × 3 = 27 combinations
        let got = Solution::letter_combinations("234".to_string());
        assert_eq!(got.len(), 27);
        // spot-check a few
        assert!(got.contains(&"adg".to_string()));
        assert!(got.contains(&"cfi".to_string()));
    }

    #[test]
    fn test_all_combinations_length() {
        // each result string must have the same length as the input
        let input = "29".to_string();
        let got = Solution::letter_combinations(input.clone());
        for s in &got {
            assert_eq!(s.len(), input.len());
        }
        // "2" = abc (3), "9" = wxyz (4) → 12 combinations
        assert_eq!(got.len(), 12);
    }
}
