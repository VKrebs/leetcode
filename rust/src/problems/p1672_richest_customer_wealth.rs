pub struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut best = 0;

        for account in accounts {
            best = best.max(account.iter().sum());
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple_1() {
        assert_eq!(
            Solution::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]),
            6
        );
    }

    #[test]
    fn exemple_2() {
        assert_eq!(
            Solution::maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]),
            10
        );
    }

    #[test]
    fn exemple_3() {
        assert_eq!(
            Solution::maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]),
            17
        );
    }
}
