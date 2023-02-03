#[cfg(test)]
use std::num;

#[cfg(test)]
struct Solution {}

#[cfg(test)]
impl Solution {
    // benched at 0ms and 2MB of memory
    pub fn my_atoi(s: String) -> i32 {
        let mut expect_only_digits = false;
        let mut num_string = String::with_capacity(10);

        for c in s.chars() {
            match (c, expect_only_digits) {
                (' ', false) => continue,
                ('-' | '+', false) => (),
                ('0'..='9', _) => (),
                _ => break,
            }
            expect_only_digits = true;
            num_string.push(c);
        }

        str::parse(&num_string).unwrap_or_else(|e: num::ParseIntError| match e.kind() {
            num::IntErrorKind::PosOverflow => i32::MAX,
            num::IntErrorKind::NegOverflow => i32::MIN,
            _ => 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _empty_string_is_0() {
        assert_eq!(Solution::my_atoi(String::from("")), 0);
    }

    #[test]
    fn _space_is_0() {
        assert_eq!(Solution::my_atoi(String::from(" ")), 0);
    }

    #[test]
    fn _neg_five_with_extra_negative() {
        assert_eq!(Solution::my_atoi(String::from("-5-")), -5);
    }

    #[test]
    fn _postive_overflow() {
        assert_eq!(
            Solution::my_atoi(String::from("9999999999999999999")),
            i32::MAX
        );
    }

    #[test]
    fn _all_hyphens_is_0() {
        assert_eq!(Solution::my_atoi(String::from("----")), 0);
    }

    #[test]
    fn _leading_spaces_are_ignored() {
        assert_eq!(Solution::my_atoi(String::from("    123")), 123);
    }

    #[test]
    fn _0_is_0() {
        assert_eq!(Solution::my_atoi(String::from("0")), 0);
    }

    #[test]
    fn _neg_1_is_neg_1() {
        assert_eq!(Solution::my_atoi(String::from("-1")), -1);
    }

    #[test]
    fn _neg_2_is_neg_2() {
        assert_eq!(Solution::my_atoi(String::from("-2")), -2);
    }

    #[test]
    fn _12_is_12() {
        assert_eq!(Solution::my_atoi(String::from("12")), 12);
    }

    #[test]
    fn _single_digits_are_single_digits() {
        assert_eq!(Solution::my_atoi(String::from("1")), 1);
        assert_eq!(Solution::my_atoi(String::from("2")), 2);
        assert_eq!(Solution::my_atoi(String::from("3")), 3);
        assert_eq!(Solution::my_atoi(String::from("4")), 4);
        assert_eq!(Solution::my_atoi(String::from("5")), 5);
        assert_eq!(Solution::my_atoi(String::from("6")), 6);
        assert_eq!(Solution::my_atoi(String::from("7")), 7);
        assert_eq!(Solution::my_atoi(String::from("8")), 8);
        assert_eq!(Solution::my_atoi(String::from("9")), 9);
    }
}
