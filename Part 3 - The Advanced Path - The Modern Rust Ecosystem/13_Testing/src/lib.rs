/**
 * @file 13_Testing/src/lib.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 13: Writing automated tests to verify code correctness.
 *
 * ## The Discipline of Testing
 *
 * A key part of writing professional, reliable software is creating automated tests.
 * Tests are Rust functions that verify that the non-test code is functioning in the
 * expected manner. They allow you to make changes and refactor your code with
 * confidence, knowing that you can easily check if you've broken anything.
 *
 * Rust has a first-class testing framework built right in.
 *
 * ### Key Concepts in this Lesson:
 * - **The `#[test]` Attribute:** How to write a function that Cargo knows how to run
 *   as a test.
 * - **The `assert!` Macros:** The primary tools for checking if a condition is true
 *   (`assert!`), or if two values are equal (`assert_eq!`) or not equal (`assert_ne!`).
 * - **The `#[should_panic]` Attribute:** How to write tests that confirm your code
 *   panics when it's supposed to. This is useful for testing error conditions.
 * - **Test Modules (`#[cfg(test)]`):** The idiomatic way to organize your test code so
 *   it doesn't get included in your final compiled binary.
 * - **Running Tests:** How to use the `cargo test` command to run all tests in your
 *   project.
 *
 * ### How to Run This Program:
 * This is a library, so we don't `cargo run` it. Instead, we test it:
 * 1. Navigate to the `13_Testing` directory in your terminal.
 * 2. Run the command: `cargo test`
 *
 * Cargo will compile and run all functions marked with `#[test]`.
 */

// --- Code to be Tested ---

/// Adds two to the number given.
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    /// Creates a new `Guess`.
    ///
    /// # Panics
    ///
    /// Panics if `value` is not between 1 and 100, inclusive.
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}

// --- Test Module ---

// This attribute, `#[cfg(test)]`, tells Rust to compile and run the code inside
// this module ONLY when we run `cargo test`, not when we run `cargo build`.
#[cfg(test)]
mod tests {
    // `use super::*;` brings all the items from the outer module (our library)
    // into the scope of the `tests` module, so we can use them easily.
    use super::*;

    // This is the simplest kind of test. The `#[test]` attribute marks it as a test function.
    // The test passes if the function completes without panicking.
    #[test]
    fn exploration() {
        // We use an assertion macro to check a result.
        // `assert_eq!` will panic if the two arguments are not equal.
        let result = add_two(2);
        assert_eq!(
            result, 4,
            "Custom failure message: add_two(2) did not equal 4"
        );
    }

    #[test]
    fn another() {
        // This test will fail because `2 + 2` does not equal `5`.
        // Run `cargo test` and see the output!
        // assert_eq!(add_two(2), 5); // Uncomment this line to see a test failure.
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // `assert!` checks if a boolean expression is true.
        // It panics if the expression is false.
        assert!(
            larger.can_hold(&smaller),
            "The larger rectangle should be able to hold the smaller one."
        );
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // We can also test for "false" conditions.
        assert!(
            !smaller.can_hold(&larger),
            "The smaller rectangle should NOT be able to hold the larger one."
        );
    }

    // This attribute checks that the code inside the function panics.
    // The test will PASS if it panics, and FAIL if it does not.
    // This is perfect for testing our `Guess::new` function's error condition.
    #[test]
    #[should_panic]
    fn guess_new_should_panic_if_greater_than_100() {
        Guess::new(200);
    }

    // We can also be more specific about the panic message.
    // The test will only pass if the function panics AND the panic message
    // contains the specified substring.
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn guess_new_should_panic_if_less_than_1() {
        Guess::new(0);
    }
}
