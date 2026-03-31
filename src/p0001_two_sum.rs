use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut marks = HashMap::new();
    for (idx, value) in nums.iter().enumerate() {
        match marks.get(&(target - value)) {
            Some(&prev) => {
                return vec![prev as i32, idx as i32];
            }
            None => {
                //
            }
        }
        marks.insert(value, idx);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
