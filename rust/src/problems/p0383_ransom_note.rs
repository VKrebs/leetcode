pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut available_letters = HashMap::new();

        for letter in magazine.chars() {
            *available_letters.entry(letter).or_insert(0) += 1;
        }

        for letter in ransom_note.chars() {
            if let Some(letter_count) = available_letters.get_mut(&letter)
                && *letter_count > 0
            {
                *letter_count -= 1;
            } else {
                return false;
            }
        }

        true
    }

    fn index(letter: u8) -> usize {
        (letter - b'a') as usize
    }

    pub fn can_construct_array(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut available_letters = [0; 26];

        for letter in magazine.bytes() {
            available_letters[Self::index(letter)] += 1;
        }

        for letter in ransom_note.bytes() {
            let count = &mut available_letters[Self::index(letter)];
            *count -= 1;

            if *count < 0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // can_construct tests
    #[test]
    fn example1() {
        assert!(!Solution::can_construct(
            String::from("a"),
            String::from("b")
        ));
    }

    #[test]
    fn example2() {
        assert!(!Solution::can_construct(
            String::from("aa"),
            String::from("ab")
        ));
    }

    #[test]
    fn example3() {
        assert!(Solution::can_construct(
            String::from("aa"),
            String::from("aab")
        ));
    }

    #[test]
    fn letter_not_present_in_magazine_returns_false() {
        assert!(!Solution::can_construct(
            String::from("aa"),
            String::from("bbb")
        ));
    }

    // can_construct_array tests
    #[test]
    fn can_construct_array_example1() {
        assert!(!Solution::can_construct_array(
            String::from("a"),
            String::from("b")
        ));
    }

    #[test]
    fn can_construct_array_example2() {
        assert!(!Solution::can_construct_array(
            String::from("aa"),
            String::from("ab")
        ));
    }

    #[test]
    fn can_construct_array_example3() {
        assert!(Solution::can_construct_array(
            String::from("aa"),
            String::from("aab")
        ));
    }

    #[test]
    fn can_construct_array_letter_not_present_in_magazine_returns_false() {
        assert!(!Solution::can_construct_array(
            String::from("aa"),
            String::from("bbb")
        ));
    }
}
