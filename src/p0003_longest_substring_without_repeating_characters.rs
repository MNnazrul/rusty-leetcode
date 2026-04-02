use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut map = HashMap::new();
    let mut latest = -1;
    let mut ans = 0;
    for (i, c) in s.chars().enumerate() {
        if let Some(&prev) = map.get(&c) {
            latest = std::cmp::max(latest, prev as i32);
        }
        map.insert(c, i);
        ans = std::cmp::max(ans, i as i32 - latest);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(length_of_longest_substring("".to_string()), 0);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(length_of_longest_substring("a".to_string()), 1);
    }

    #[test]
    fn test_all_unique() {
        assert_eq!(length_of_longest_substring("abcdef".to_string()), 6);
    }
}
