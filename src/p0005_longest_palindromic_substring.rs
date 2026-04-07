pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return "".to_string();
        }

        let mut t = Vec::new();
        t.push('$');

        for ch in s.chars() {
            t.push('#');
            t.push(ch);
        }
        t.push('#');
        t.push('^');

        let n = t.len();
        let mut p = vec![0; n];

        let mut c = 0;
        let mut r = 0;

        for i in 1..n - 1 {
            let mirror = 2 * c as isize - i as isize;

            if i < r && mirror >= 0 {
                p[i] = std::cmp::min((r - i) as i32, p[mirror as usize]);
            }

            while t[i + (p[i] + 1) as usize] == t[i - (p[i] + 1) as usize] {
                p[i] += 1;
            }

            if i + p[i] as usize > r {
                c = i;
                r = i + p[i] as usize;
            }
        }

        let mut max_len = 0;
        let mut center_index = 0;

        for (i, &radius) in p.iter().enumerate().take(n - 1).skip(1) {
            if radius > max_len {
                max_len = radius;
                center_index = i;
            }
        }

        let start = (center_index - max_len as usize) / 2;
        s.chars().skip(start).take(max_len as usize).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab");
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a");
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::longest_palindrome("aaaa".to_string()), "aaaa");
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::longest_palindrome("".to_string()), "");
    }

    #[test]
    fn test_even_palindrome() {
        assert_eq!(Solution::longest_palindrome("abccba".to_string()), "abccba");
    }
}
