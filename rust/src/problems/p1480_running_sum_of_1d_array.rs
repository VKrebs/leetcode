pub struct Solution;

impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple_1() {
        assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    }

    #[test]
    fn exemple_2() {
        assert_eq!(
            Solution::running_sum(vec![1, 1, 1, 1, 1]),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn exemple_3() {
        assert_eq!(
            Solution::running_sum(vec![3, 1, 2, 10, 1]),
            vec![3, 4, 6, 16, 17]
        );
    }

    #[test]
    fn array_with_one_number_returns_array_with_same_number() {
        assert_eq!(Solution::running_sum(vec![3]), vec![3]);
    }
}
