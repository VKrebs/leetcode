// 0001. Two Sum
// https://leetcode.com/problems/two-sum/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::with_capacity(nums.len());

        for (index, &value) in nums.iter().enumerate() {
            let complement = target - value;

            if let Some(&complement_index) = seen.get(&complement) {
                return vec![
                    i32::try_from(complement_index).unwrap(),
                    i32::try_from(index).unwrap(),
                ];
            }

            seen.insert(value, index);
        }

        unreachable!()
    }

    pub fn two_sum_naive(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i32::try_from(i).unwrap(), i32::try_from(j).unwrap()];
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The answer may be returned in any order, so compare sorted indices.
    fn sorted(mut indices: Vec<i32>) -> Vec<i32> {
        indices.sort_unstable();
        indices
    }

    #[test]
    fn example1_naive() {
        assert_eq!(
            sorted(Solution::two_sum_naive(vec![2, 7, 11, 15], 9)),
            vec![0, 1]
        );
    }

    #[test]
    fn example2_naive() {
        assert_eq!(
            sorted(Solution::two_sum_naive(vec![3, 2, 4], 6)),
            vec![1, 2]
        );
    }

    #[test]
    fn example3_naive() {
        assert_eq!(sorted(Solution::two_sum_naive(vec![3, 3], 6)), vec![0, 1]);
    }

    #[test]
    fn example1() {
        assert_eq!(sorted(Solution::two_sum(vec![2, 7, 11, 15], 9)), vec![0, 1]);
    }

    #[test]
    fn example2() {
        assert_eq!(sorted(Solution::two_sum(vec![3, 2, 4], 6)), vec![1, 2]);
    }

    #[test]
    fn example3() {
        assert_eq!(sorted(Solution::two_sum(vec![3, 3], 6)), vec![0, 1]);
    }
}
