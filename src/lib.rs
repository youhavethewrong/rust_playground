mod guess;
mod rect;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    // run `cargo test -- --nocapture` to get the output
    // run `cargo test -- --ignored` to run ignored tests
    // run `cargo test one_hundred` to run matching tests
    use super::*;

    #[test]
    fn should_equal_four() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn should_equal_five() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn should_equal_one_hundred_two() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
        assert_eq!(2, add_two(0));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    fn this_test_will_pass() {
        assert_eq!(10, prints_and_returns_10(8));
    }

    #[test]
    fn this_test_will_fail() {
        assert_eq!(5, prints_and_returns_10(4));
    }
}
