pub struct Solution;

impl Solution {
    pub fn find_kth_element(
        vec1: &[i32],
        vec2: &[i32],
        mut i: usize,
        mut j: usize,
        mut k: usize,
    ) -> i32 {
        let n = vec1.len();
        let m = vec2.len();

        loop {
            if i >= n {
                return vec2[j + k - 1];
            }
            if j >= m {
                return vec1[i + k - 1];
            }
            if k == 1 {
                return std::cmp::min(vec1[i], vec2[j]);
            }

            let p = k / 2;

            let x = if i + p - 1 < n {
                vec1[i + p - 1]
            } else {
                i32::MAX
            };

            let y = if j + p - 1 < m {
                vec2[j + p - 1]
            } else {
                i32::MAX
            };

            if x < y {
                i += p;
                k -= p;
            } else {
                j += p;
                k -= p;
            }
        }
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total = nums1.len() + nums2.len();

        if total % 2 == 1 {
            Self::find_kth_element(&nums1, &nums2, 0, 0, total / 2 + 1) as f64
        } else {
            let left = Self::find_kth_element(&nums1, &nums2, 0, 0, total / 2);
            let right = Self::find_kth_element(&nums1, &nums2, 0, 0, total / 2 + 1);
            (left as f64 + right as f64) / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1], vec![]), 1.0);
    }

    #[test]
    fn test_empty_first() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![2, 3]), 2.5);
    }

    #[test]
    fn test_larger_arrays() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]),
            5.5
        );
    }

    #[test]
    fn test_interleaved() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3, 5], vec![2, 4, 6]),
            3.5
        );
    }
}
