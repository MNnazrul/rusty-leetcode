pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let n = s.len();
        let m = p.len();

        // dp[i][j] = true means s[0..i] matches p[0..j]
        let mut dp = vec![vec![false; m + 1]; n + 1];
        dp[0][0] = true;

        // Handle patterns like a*, a*b*, a*b*c* that can match empty string
        for j in 1..=m {
            if p[j - 1] == '*' {
                dp[0][j] = dp[0][j - 2];
            }
        }

        for i in 1..=n {
            for j in 1..=m {
                if p[j - 1] == '*' {
                    // Option 1: treat "x*" as zero occurrences of x
                    dp[i][j] = dp[i][j - 2];
                    // Option 2: treat "x*" as one more occurrence of x
                    if p[j - 2] == '.' || p[j - 2] == s[i - 1] {
                        dp[i][j] = dp[i][j] || dp[i - 1][j];
                    }
                } else if p[j - 1] == '.' || p[j - 1] == s[i - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                }
            }
        }

        dp[n][m]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!(!Solution::is_match("aa".to_string(), "a".to_string()));
    }

    #[test]
    fn test_example_2() {
        assert!(Solution::is_match("aa".to_string(), "a*".to_string()));
    }

    #[test]
    fn test_example_3() {
        assert!(Solution::is_match("ab".to_string(), ".*".to_string()));
    }

    #[test]
    fn test_star_zero_occurrences() {
        assert!(Solution::is_match("aab".to_string(), "c*a*b".to_string()));
    }

    #[test]
    fn test_no_match() {
        assert!(!Solution::is_match(
            "mississippi".to_string(),
            "mis*is*p*.".to_string()
        ));
    }

    #[test]
    fn test_empty_string_with_star_pattern() {
        assert!(Solution::is_match("".to_string(), "a*".to_string()));
    }

    #[test]
    fn test_dot_matches_any() {
        assert!(Solution::is_match("abc".to_string(), "a.c".to_string()));
    }
}
