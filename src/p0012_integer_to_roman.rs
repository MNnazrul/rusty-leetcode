pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let symbols = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];
        let mut ans = String::new();
        let mut num = num;

        for i in 0..values.len() {
            let count = num / values[i];
            for _ in 0..count {
                ans.push_str(symbols[i]);
            }
            num %= values[i];
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // 3749 → "MMMDCCXLIX"
        assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX");
    }

    #[test]
    fn test_example_2() {
        // 58 → "LVIII"
        assert_eq!(Solution::int_to_roman(58), "LVIII");
    }

    #[test]
    fn test_example_3() {
        // 1994 → "MCMXCIV"
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }

    #[test]
    fn test_one() {
        assert_eq!(Solution::int_to_roman(1), "I");
    }

    #[test]
    fn test_four() {
        // subtractive form
        assert_eq!(Solution::int_to_roman(4), "IV");
    }

    #[test]
    fn test_nine() {
        assert_eq!(Solution::int_to_roman(9), "IX");
    }

    #[test]
    fn test_max() {
        // 3999 → "MMMCMXCIX"
        assert_eq!(Solution::int_to_roman(3999), "MMMCMXCIX");
    }
}
