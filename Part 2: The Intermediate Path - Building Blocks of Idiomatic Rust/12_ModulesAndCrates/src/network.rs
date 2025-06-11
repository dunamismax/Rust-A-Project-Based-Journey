/**
 * @file src/network.rs
 * @brief The `network` module.
 *
 * This file's contents correspond to the `mod network;` declaration in `lib.rs`.
 * It defines the public API of the `network` module.
 */
// This declares a public sub-module named `client`.
// Rust will look for this module's code in either:
// 1. `src/network/client.rs` (which is what we are using)
// 2. `src/network/client/mod.rs` (an older style)
pub mod client;

// A function at the top level of the `network` module.
pub fn ping() {
    println!("  -> [network] Pinging network...");
}
