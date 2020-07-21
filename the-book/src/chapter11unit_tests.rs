pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

// Unit tests go in the same files as the code

// The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run
// cargo test, not when you run cargo build. This saves compile time when you only want to build the library and
// saves space in the resulting compiled artifact because the tests are not included.
#[cfg(test)]
mod tests {
    // tests module is an inner module!
    use super::*;

    #[test]
    fn addition() {
        assert_eq!(4, 2 + 2);
        assert_ne!(5, 2 + 2);
    }

    #[test]
    #[ignore]
    fn test_that_would_fail_if_it_wasnt_ignored() {
        panic!("Make this test fail");
        // If we want to run only the ignored tests, we can use 'cargo test -- --ignored'
    }


    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be greater than or equal to 1")]
    fn less_than_1() {
        Guess::new(0);
    }
}

// Integration tests go into the /tests directory.

// If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file,
// we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs file
// into scope with a use statement. Only library crates expose functions that other crates can use;
// binary crates are meant to be run on their own.

// We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)].
// Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test.