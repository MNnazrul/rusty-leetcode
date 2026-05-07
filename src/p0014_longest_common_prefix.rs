pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ans = strs[0].clone();

        for st in strs {
            while !st.starts_with(&ans) {
                ans.pop();
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(strs: &[&str]) -> Vec<String> {
        strs.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_example_1() {
        // ["flower","flow","flight"] → "fl"
        assert_eq!(
            Solution::longest_common_prefix(v(&["flower", "flow", "flight"])),
            "fl"
        );
    }

    #[test]
    fn test_example_2() {
        // ["dog","racecar","car"] → ""
        assert_eq!(
            Solution::longest_common_prefix(v(&["dog", "racecar", "car"])),
            ""
        );
    }

    #[test]
    fn test_single_string() {
        assert_eq!(Solution::longest_common_prefix(v(&["alone"])), "alone");
    }

    #[test]
    fn test_all_identical() {
        assert_eq!(
            Solution::longest_common_prefix(v(&["abc", "abc", "abc"])),
            "abc"
        );
    }

    #[test]
    fn test_one_is_prefix_of_others() {
        assert_eq!(
            Solution::longest_common_prefix(v(&["interspecies", "interstellar", "interstate"])),
            "inters"
        );
    }

    #[test]
    fn test_first_is_shortest() {
        assert_eq!(
            Solution::longest_common_prefix(v(&["ab", "abc", "abcd"])),
            "ab"
        );
    }

    #[test]
    fn test_first_is_longest() {
        assert_eq!(
            Solution::longest_common_prefix(v(&["abcd", "abc", "ab"])),
            "ab"
        );
    }

    #[test]
    fn test_single_char_match() {
        assert_eq!(Solution::longest_common_prefix(v(&["a", "ab", "abc"])), "a");
    }

    #[test]
    fn test_empty_string_in_list() {
        assert_eq!(Solution::longest_common_prefix(v(&["", "abc"])), "");
    }
}
