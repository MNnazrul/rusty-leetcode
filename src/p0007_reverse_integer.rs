pub struct Solution;

impl Solution {
    pub fn reverse(x1: i32) -> i32 {
        let mut flag = false;
        let mut x = x1;
        if x < 0 {
            x *= -1;
            flag = true;
        }

        let mut ans = 0;
        while x > 0 {
            let i = x % 10;
            x /= 10;
            if ans > i32::MAX / 10 || (ans == i32::MAX / 10 && i > i32::MAX % 10) {
                return 0;
            }
            ans = ans * 10 + i;
        }
        if flag {
            ans *= -1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // 123 → 321
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn test_example_2() {
        // -123 → -321
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn test_example_3() {
        // 120 → 21 (trailing zero is dropped)
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn test_overflow_positive() {
        // 1534236469 reversed overflows i32 → 0
        assert_eq!(Solution::reverse(1534236469), 0);
    }

    #[test]
    fn test_zero() {
        assert_eq!(Solution::reverse(0), 0);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(Solution::reverse(5), 5);
    }
}
