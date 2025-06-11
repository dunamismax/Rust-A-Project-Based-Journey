use _12_modulesandcrates::get_random_number;
/**
 * @file src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 12: The Binary Crate - Consuming our Library.
 *
 * This `main.rs` file is the entry point for our executable program.
 * Its job is to use the functionality we've neatly organized in our library crate.
 *
 * ### Key Concepts in this File:
 * - **Paths and `use`:** We use the `use` keyword to bring items from our library
 *   into the scope of `main.rs`, making them easier to call.
 * - **Crate Name:** To refer to our library, we use its name as defined in `Cargo.toml`.
 *   Rust normalizes this to `_12_modulesandcrates` because the original name is not a
 *   valid Rust identifier.
 *
 * ### How to Run This Program:
 * - `cargo run`
 *   This single command will compile the library, compile the binary (linking against
 *   the library), and run the final executable.
 */
// We use the `use` keyword to bring parts of our library into the local scope.
// The path starts with the crate name. Because "12_modulesandcrates" is not a
// valid Rust identifier, Cargo renames it to `_12_modulesandcrates` for use in code.
use _12_modulesandcrates::network;
use _12_modulesandcrates::network::client;

fn main() {
    println!("--- Lesson 12: Modules and Crates ---\n");
    println!("Welcome to the main application!");
    println!("Let's use the functions from our library crate.\n");

    println!("1. Calling a function from a sub-module:");
    // Because we brought `client` into scope, we can call it directly.
    client::connect();

    println!("\n2. Calling a function from a parent module:");
    // Similarly, we can call `ping()` directly on the `network` module.
    network::ping();

    println!("\n3. Calling a top-level library function that uses an external crate:");
    let random_num = get_random_number();
    println!("  -> [library] The random number is: {}", random_num);

    println!("\n--- End of Lesson 12 ---");
    println!("Congratulations on finishing Part 2! You now have the tools to build well-structured Rust programs.");
}
