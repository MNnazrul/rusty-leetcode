pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut ans = 0;

        while l < r {
            ans = std::cmp::max(ans, std::cmp::min(height[l], height[r]) * ((r - l) as i32));
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
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
        // [1,8,6,2,5,4,8,3,7] → 49 (between indices 1 and 8, min(8,7) * 7 = 49)
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn test_example_2() {
        // [1,1] → 1 (min(1,1) * 1 = 1)
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }

    #[test]
    fn test_increasing() {
        // [1,2,3,4,5] → min(1,5)*4 = 4, min(2,5)*3 = 6, min(3,5)*2 = 6, min(4,5)*1 = 4 → 6
        assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5]), 6);
    }

    #[test]
    fn test_decreasing() {
        // [5,4,3,2,1] → max is min(5,4)*1 stepping inward: actual max is min(5,1)*4=4, min(5,2)*3=6, etc → 6
        assert_eq!(Solution::max_area(vec![5, 4, 3, 2, 1]), 6);
    }

    #[test]
    fn test_two_tall_edges() {
        // [8,1,1,1,1,1,1,8] → min(8,8) * 7 = 56
        assert_eq!(Solution::max_area(vec![8, 1, 1, 1, 1, 1, 1, 8]), 56);
    }
}
