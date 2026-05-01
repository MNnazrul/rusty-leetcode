pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let symbols = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];
        let mut ans = 0;
        let mut id: usize = 0;

        for (idx, &sym) in symbols.iter().enumerate() {
            while id < s.len() && s[id..].starts_with(sym) {
                ans += values[idx];
                id += sym.len();
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // "III" → 3
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn test_example_2() {
        // "LVIII" → 58
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn test_example_3() {
        // "MCMXCIV" → 1994
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }

    #[test]
    fn test_one() {
        assert_eq!(Solution::roman_to_int("I".to_string()), 1);
    }

    #[test]
    fn test_four_subtractive() {
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    }

    #[test]
    fn test_nine_subtractive() {
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
    }

    #[test]
    fn test_max() {
        // 3999 → "MMMCMXCIX"
        assert_eq!(Solution::roman_to_int("MMMCMXCIX".to_string()), 3999);
    }

    #[test]
    fn test_3749() {
        // mixes D and CC, plus XL and IX
        assert_eq!(Solution::roman_to_int("MMMDCCXLIX".to_string()), 3749);
    }
}
