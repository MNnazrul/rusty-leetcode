pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let n = nums.len();
        if n < 3 {
            return result;
        }

        nums.sort();

        for i in 0..n - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            if nums[i] + nums[i + 1] + nums[i + 2] > 0 {
                break;
            }

            let mut left = i + 1;
            let mut right = n - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum == 0 {
                    result.push(vec![nums[i], nums[left], nums[right]]);

                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }

                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sort_triples(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for t in v.iter_mut() {
            t.sort();
        }
        v.sort();
        v
    }

    #[test]
    fn test_example_1() {
        // [-1,0,1,2,-1,-4] → [[-1,-1,2],[-1,0,1]]
        let got = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        let want = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(sort_triples(got), sort_triples(want));
    }

    #[test]
    fn test_example_2() {
        // [0,1,1] → []
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_example_3() {
        // [0,0,0] → [[0,0,0]]
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::three_sum(vec![]), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::three_sum(vec![1, -1]), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_all_zeros() {
        // multiple zeros must not produce duplicates
        assert_eq!(Solution::three_sum(vec![0, 0, 0, 0]), vec![vec![0, 0, 0]]);
    }

    #[test]
    fn test_all_positive() {
        // smallest three sum > 0 → early break, no triples
        assert_eq!(
            Solution::three_sum(vec![1, 2, 3, 4, 5]),
            Vec::<Vec<i32>>::new()
        );
    }

    #[test]
    fn test_all_negative() {
        assert_eq!(
            Solution::three_sum(vec![-5, -4, -3, -2, -1]),
            Vec::<Vec<i32>>::new()
        );
    }

    #[test]
    fn test_duplicates_skipped() {
        // [-2,0,0,2,2] → [[-2,0,2]] only once
        assert_eq!(
            Solution::three_sum(vec![-2, 0, 0, 2, 2]),
            vec![vec![-2, 0, 2]]
        );
    }

    #[test]
    fn test_mixed_larger() {
        // [-4,-2,-2,-2,0,1,2,2,2,3,3,4,4,6,6]
        let got = Solution::three_sum(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6]);
        let want = vec![
            vec![-4, -2, 6],
            vec![-4, 0, 4],
            vec![-4, 1, 3],
            vec![-4, 2, 2],
            vec![-2, -2, 4],
            vec![-2, 0, 2],
        ];
        assert_eq!(sort_triples(got), sort_triples(want));
    }
}
