fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn this_test_will_pass() {
        // "I got the value 4" is not displayed
        // without option -- --show-output
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}

#[cfg(test)]
mod tests_add_two {
    use super::add_two;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    // this test runs only when `cargo test -- --ignored`
    #[test]
    #[ignore]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    // run only this test by `cargo test one_hundred`
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}
