pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let l = nums.len();

        let mut best_sum = nums[0] + nums[1] + nums[2];
        let mut short_distance = (best_sum - target).abs();

        for i in 0..l - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let sum = nums[i] + nums[i + 1] + nums[i + 2];

            if sum > target {
                if (sum - target).abs() < short_distance {
                    return sum;
                } else {
                    break;
                }
            }

            let sum = nums[i] + nums[l - 1] + nums[l - 2];
            if sum < target {
                if (sum - target).abs() < short_distance {
                    best_sum = sum;
                    short_distance = (sum - target).abs();
                }
                continue;
            }

            let mut left = i + 1;
            let mut right = l - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                let distance = sum - target;

                if distance.abs() < short_distance {
                    best_sum = sum;
                    short_distance = distance.abs();
                }

                match distance.signum() {
                    1 => right -= 1,
                    -1 => left += 1,
                    0 => return target,
                    _ => unreachable!(),
                }
            }
        }

        best_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // [-1,2,1,-4], target=1 → -1+2+1=2 is closest
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }

    #[test]
    fn test_example_2() {
        // [0,0,0], target=1 → 0+0+0=0
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }

    #[test]
    fn test_exact_match() {
        // sum equals target exactly
        assert_eq!(Solution::three_sum_closest(vec![1, 1, 1, 0], 3), 3);
    }

    #[test]
    fn test_all_negative() {
        // [-5,-3,-1], target=-4 → -5+-3+-1=-9, -5+-3+-1, best=-3+-5? no, only 3 elements → -9
        // Actually only one triple: -5+-3+-1=-9; closest to -4 is -9? No wait: -3+-5+-1=-9. Hmm.
        // Let's use [-4,-2,-1], target=-4 → -4+-2+-1=-7, -4+-2+(-1) no only [-4,-2,-1]
        // Use [-1,-2,-3], target=-5 → -1+-2+-3=-6 (distance 1), or no: -6 vs -5, distance=1
        assert_eq!(Solution::three_sum_closest(vec![-1, -2, -3], -5), -6);
    }

    #[test]
    fn test_large_positive_target() {
        // [1,2,3,4,5], target=100 → max triple = 3+4+5=12
        assert_eq!(Solution::three_sum_closest(vec![1, 2, 3, 4, 5], 100), 12);
    }

    #[test]
    fn test_large_negative_target() {
        // [1,2,3,4,5], target=-100 → min triple = 1+2+3=6
        assert_eq!(Solution::three_sum_closest(vec![1, 2, 3, 4, 5], -100), 6);
    }
}
