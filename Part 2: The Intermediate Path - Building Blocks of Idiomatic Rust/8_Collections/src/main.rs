/**
 * @file 8_Collections/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 8: Using common collections to store lists and key-value pairs.
 *
 * ## Storing Multiple Values: Collections
 *
 * Unlike the built-in array and tuple types, collections store their data on the heap.
 * This means their size doesn't need to be known at compile time and can grow or shrink
 * as the program runs. We will cover the two most essential collections.
 *
 * ### Key Concepts in this Lesson:
 * - **`Vec<T>` (Vector):** A dynamic, growable list of items, all of the same type `T`.
 *   It's the most common list type in Rust.
 * - **`HashMap<K, V>` (Hash Map):** A collection of key-value pairs. Given a key `K`, it
 *   can find the corresponding value `V` very quickly.
 * - **Ownership in Collections:** We'll see how ownership rules (moving, borrowing) apply
 *   when we add items to or read items from collections.
 * - **Iterating:** How to loop over the elements in a collection, both immutably and mutably.
 *
 * ### How to Run This Program:
 * - `cargo run`
 */
// We need to bring HashMap into scope explicitly from the standard library.
use std::collections::HashMap;

fn main() {
    println!("--- Lesson 8: Common Collections ---\n");

    // --- 1. Vectors (`Vec<T>`) for Storing Lists ---
    println!("--- 1. Vectors (`Vec<T>`) ---");

    // A) Creating Vectors
    // Create an empty vector. We must specify the type it will hold.
    let mut v1: Vec<i32> = Vec::new();
    v1.push(5);
    v1.push(6);
    v1.push(7);
    println!("v1 (created with Vec::new and push): {:?}", v1);

    // Use the `vec!` macro for a more convenient way to create a vector with initial values.
    let v2 = vec![1, 2, 3];
    println!("v2 (created with the `vec!` macro): {:?}", v2);

    // B) Reading Elements from a Vector
    let third_element_by_index: &i32 = &v2[2]; // Indexing starts at 0.
    println!(
        "The third element of v2 (via index) is: {}",
        third_element_by_index
    );

    // Using `.get()` is often safer because it returns an `Option<&T>`.
    // This won't crash your program if you try to access an index that doesn't exist.
    match v2.get(2) {
        Some(third) => println!("The third element of v2 (via .get()) is: {}", third),
        None => println!("There is no third element."),
    }

    // C) Ownership and the Borrow Checker with Vectors
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // Immutable borrow here.

    // v.push(6); // ERROR! Uncommenting this line will cause a compiler error.
    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    // Why? Because `push` might need to reallocate the vector's memory on the heap if it's full.
    // If that happens, the memory location `first` is pointing to would be invalid ("dangling").
    // The borrow checker prevents this bug before it can happen!

    println!(
        "The first element is {}. We can no longer push to `v` in this scope.",
        first
    );
    // The borrow of `first` ends here, because it's the last time it's used.

    // D) Iterating over a Vector
    println!("Iterating over v2 immutably:");
    for i in &v2 {
        println!("  Got: {}", i);
    }

    println!("Iterating over v1 mutably (and adding 10 to each element):");
    for i in &mut v1 {
        // We need to use the dereference operator `*` to get to the value.
        *i += 10;
    }
    println!("v1 after mutable iteration: {:?}", v1);

    // --- 2. Hash Maps (`HashMap<K, V>`) for Key-Value Storage ---
    println!("\n--- 2. Hash Maps (`HashMap<K, V>`) ---");

    // A) Creating and Inserting into a Hash Map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // For types that have the `Copy` trait (like `i32`), the values are copied.
    // For owned types like `String`, the values are MOVED.
    let team_name = String::from("Red");
    scores.insert(team_name, 25);
    // `team_name` is no longer valid here; its ownership was moved into the hash map.
    // println!("{}", team_name); // ERROR: borrow of moved value

    println!("Current scores: {:?}", scores);

    // B) Accessing Values
    let blue_score = scores.get("Blue"); // Returns an Option<&V>
    match blue_score {
        Some(score) => println!("The score for the Blue team is: {}", score),
        None => println!("The Blue team doesn't have a score."),
    }

    // C) Iterating over a Hash Map
    println!("Iterating over all scores:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }

    // D) Updating Values in a Hash Map
    // i. Overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("Updated Blue team score (overwritten): {:?}", scores);

    // ii. Only inserting if the key has no value. This is very idiomatic!
    // `entry` returns an enum `Entry` that represents a value that might or might not exist.
    // `or_insert` then gets the value out or inserts the argument and returns a mutable reference.
    scores.entry(String::from("Blue")).or_insert(30); // Blue exists, so this does nothing.
    scores.entry(String::from("Green")).or_insert(30); // Green does not exist, so it's inserted.
    println!("Scores after using `or_insert`: {:?}", scores);

    // iii. Updating a value based on its old value (e.g., word count)
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // `or_insert(0)` returns a mutable reference to the count for this word.
        let count = map.entry(word).or_insert(0);
        *count += 1; // We dereference `count` to modify the value.
    }
    println!("Word count from text: {:?}", map);

    println!("\n--- End of Lesson 8 ---");
}
