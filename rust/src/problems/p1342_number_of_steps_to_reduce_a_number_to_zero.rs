pub struct Solution;

impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut steps = 0;

        while num != 0 {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num -= 1;
            }
            steps += 1;
        }

        steps
    }
}

// TODO: bitwise approach

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple_1() {
        assert_eq!(Solution::number_of_steps(14), 6);
    }

    #[test]
    fn exemple_2() {
        assert_eq!(Solution::number_of_steps(8), 4);
    }

    #[test]
    fn exemple_3() {
        assert_eq!(Solution::number_of_steps(123), 12);
    }
}
