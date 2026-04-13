pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut ans: i64 = 0;
        let s: Vec<char> = s.trim().chars().collect();
        if s.is_empty() {
            return 0;
        }
        let is_pos = s[0] != '-';
        let mut start = 0;
        if s[0] == '-' || s[0] == '+' {
            start = 1;
        }

        while ans <= (1 << 31) && start < s.len() && s[start].is_ascii_digit() {
            ans = ans * 10 + s[start].to_digit(10).unwrap() as i64;
            start += 1;
        }

        if !is_pos {
            ans *= -1;
        }
        ans = std::cmp::min(ans, (1 << 31) - 1);
        ans = std::cmp::max(ans, -(1 << 31));

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // "42" → 42
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
    }

    #[test]
    fn test_example_2() {
        // "   -042" → -42 (leading spaces, leading zeros)
        assert_eq!(Solution::my_atoi("   -042".to_string()), -42);
    }

    #[test]
    fn test_example_3() {
        // "1337c0d3" → 1337 (stops at non-digit)
        assert_eq!(Solution::my_atoi("1337c0d3".to_string()), 1337);
    }

    #[test]
    fn test_example_4() {
        // "0-1" → 0 (stops at '-' after digit)
        assert_eq!(Solution::my_atoi("0-1".to_string()), 0);
    }

    #[test]
    fn test_example_5() {
        // "words and 987" → 0 (starts with non-digit)
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    }

    #[test]
    fn test_overflow_positive() {
        // exceeds i32::MAX → clamped to 2147483647
        assert_eq!(Solution::my_atoi("99999999999".to_string()), 2147483647);
    }

    #[test]
    fn test_overflow_negative() {
        // exceeds i32::MIN → clamped to -2147483648
        assert_eq!(Solution::my_atoi("-99999999999".to_string()), -2147483648);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::my_atoi("".to_string()), 0);
    }

    #[test]
    fn test_plus_sign() {
        assert_eq!(Solution::my_atoi("+42".to_string()), 42);
    }
}
