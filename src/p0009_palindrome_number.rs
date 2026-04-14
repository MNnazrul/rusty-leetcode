pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string() == x.to_string().chars().rev().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // 121 → true
        assert!(Solution::is_palindrome(121));
    }

    #[test]
    fn test_example_2() {
        // -121 → false (negative numbers are not palindromes)
        assert!(!Solution::is_palindrome(-121));
    }

    #[test]
    fn test_example_3() {
        // 10 → false
        assert!(!Solution::is_palindrome(10));
    }

    #[test]
    fn test_zero() {
        assert!(Solution::is_palindrome(0));
    }

    #[test]
    fn test_single_digit() {
        assert!(Solution::is_palindrome(7));
    }
}
