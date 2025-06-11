/**
 * @file 9_ErrorHandling/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 9: The professional way to handle errors in Rust.
 *
 * ## Writing Resilient Code: Error Handling
 *
 * Rust takes errors very seriously and provides tools to handle them in a robust and
 * explicit way. It groups errors into two major categories:
 *
 * 1. **Unrecoverable Errors (`panic!`)**: For when things are in a state so bad the
 *    program cannot possibly continue. This is for true programmer errors or states
 *    that should be impossible.
 * 2. **Recoverable Errors (`Result<T, E>`)**: For errors that you can reasonably
 *    expect to happen, like a file not being found or a network connection failing.
 *    This is how you write functions that can fail without crashing the whole program.
 *
 * This lesson focuses on the idiomatic way to handle recoverable errors.
 *
 * ### Key Concepts in this Lesson:
 * - **`panic!` vs. `Result`:** Understanding when to crash and when to return an error.
 * - **The `Result<T, E>` Enum:** The primary tool for returning recoverable errors.
 * - **Propagating Errors:** The pattern of passing an error up the call stack for a
 *   higher-level function to handle.
 * - **The `?` Operator:** The "question mark" operator, which provides incredibly
 *   clean and ergonomic syntax for propagating errors. This is a game-changer.
 *
 * ### How to Run This Program:
 * - `cargo run`
 * - To see the file reading succeed, create a file named `username.txt` in the root
 *   of the repository (next to the main `Cargo.toml`) and write your name in it.
 */
use std::fs::File;
use std::io::{self, Read};

// A function that demonstrates returning a Result.
// It can succeed (returning an `Ok` variant with an `f64`) or fail
// (returning an `Err` variant with a `String` message).
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        // If the denominator is zero, we can't divide. We return an error.
        Err(String::from("Cannot divide by zero!"))
    } else {
        // Otherwise, we return the successful result wrapped in `Ok`.
        Ok(numerator / denominator)
    }
}

// This function shows the "old way" of propagating errors using a `match` statement.
// It's verbose, but it's important to understand what the `?` operator is doing for us.
fn read_username_from_file_verbose() -> Result<String, io::Error> {
    let f = File::open("username.txt");

    let mut file = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // If opening the file failed, return the error early.
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username), // If reading succeeded, return the username.
        Err(e) => Err(e),      // If reading failed, return that error.
    }
}

// This function does the EXACT same thing as the one above, but uses the `?` operator.
// The `?` operator placed after a `Result` value does the following:
// - If the `Result` is `Ok`, the value inside the `Ok` is returned from this expression,
//   and the program continues.
// - If the `Result` is `Err`, the value inside the `Err` is returned from the whole function
//   immediately, just like our `return Err(e)` lines above.
fn read_username_from_file_concise() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?; // Fails here if file can't be opened.
    let mut username = String::new();
    file.read_to_string(&mut username)?; // Fails here if read fails.
    Ok(username) // If both `?` operations succeeded, return the username.
}

fn main() {
    println!("--- Lesson 9: Error Handling ---\n");

    // --- 1. `panic!` (Unrecoverable) ---
    // Uncommenting the next line would immediately stop the program with an error message.
    // Use this when there's no way to recover and continuing is unsafe.
    // panic!("This is a forced panic!");
    println!("--- 1. `panic!` is for unrecoverable errors (code is commented out).");

    // --- 2. `Result` (Recoverable) ---
    println!("\n--- 2. Handling `Result` with `match` ---");
    let result_ok = divide(10.0, 2.0);
    let result_err = divide(10.0, 0.0);

    // We can use `match` to handle both possibilities.
    match result_ok {
        Ok(value) => println!("10 / 2 = {}", value),
        Err(msg) => println!("Error: {}", msg),
    }
    match result_err {
        Ok(value) => println!("10 / 0 = {}", value),
        Err(msg) => println!("Error dividing 10 by 0: {}", msg),
    }

    // --- 3. The `?` Operator for Clean Error Propagation ---
    println!("\n--- 3. Using the `?` operator to propagate errors ---");
    println!("(Note: This part needs a 'username.txt' file to succeed.)");

    // We call our fallible function. The code inside `read_username_from_file_concise`
    // handles the intermediate steps, propagating any errors up to us.
    // We only have to handle the FINAL `Result`.
    match read_username_from_file_concise() {
        Ok(username) => println!("Successfully read username: {}", username),
        Err(error) => {
            // `error` is of type `io::Error`. We can inspect it further.
            println!("Error reading username file: {}", error);
            println!("Maybe you forgot to create the 'username.txt' file?");
        }
    }

    println!("\n--- End of Lesson 9 ---");
    // Takeaway: Use `Result` and `?` for any function that might fail in an expected way.
    // This makes your code robust, explicit, and much easier to read!
}
