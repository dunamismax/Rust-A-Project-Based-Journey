/**
 * @file src/lib.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 12: The Library Crate Root.
 *
 * This file is the entry point for our project's "library crate". A project can have
 * both a binary (`main.rs`) and a library (`lib.rs`). The library contains reusable
 * code that the binary can depend on.
 *
 * ### Key Concepts in this File:
 * - **Crate Root:** `lib.rs` is the root of the library crate.
 * - **Module Declaration (`mod`):** We declare the modules that are part of this crate.
 * - **Public API (`pub`):** The `pub` keyword makes items like modules and functions
 *   visible and usable by code outside of this module (like in `main.rs`).
 * - **Using External Crates:** We `use` the `rand` crate we added to Cargo.toml.
 */
// This declares a public module named `network`.
// Because we created a `src/network.rs` file, the compiler knows to load
// the module's contents from there.
pub mod network;

// We bring the `Rng` trait from our external `rand` crate into scope.
use rand::Rng;

// A public function at the top level of our library crate.
pub fn get_random_number() -> u32 {
    // Use the external crate's functionality.
    rand::thread_rng().gen_range(1..=100)
}
