/**
 * @file 17_WorkingWithJSON/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Project 17: Parsing JSON into Rust structs and vice-versa with Serde.
 *
 * ## The Modern Rust Ecosystem: Working with JSON
 *
 * In today's world of web APIs and configuration files, JSON (JavaScript Object
 * Notation) is everywhere. A crucial skill for any modern programmer is being able to
 * reliably parse JSON into native data structures and serialize those structures back
 * into JSON.
 *
 * In the Rust ecosystem, this is dominated by the `serde` framework. `serde` is a
 * powerful and generic framework for serialization and deserialization. We combine it
 * with a data format crate, like `serde_json`, to work with a specific format.
 *
 * ### Key Concepts in this Lesson:
 * - **`serde` and `serde_json`:** The two crates that make this process seamless.
 * - **`#[derive(Serialize, Deserialize)]`:** The magic "derive macros" that
 *   automatically implement the `Serialize` and `Deserialize` traits for our structs.
 * - **Deserialization:** Parsing a JSON string into strongly-typed Rust structs using
 *   `serde_json::from_str()`.
 * - **Serialization:** Converting Rust structs into a JSON string using
 *   `serde_json::to_string_pretty()`.
 *
 * ### How to Run This Program:
 * 1. Navigate to the `17_WorkingWithJSON` directory.
 * 2. Cargo will fetch the new dependencies for you: `cargo build`
 * 3. Run the program: `cargo run`
 */
// Import the derive macros and the main serde_json functions.
use serde::{Deserialize, Serialize};
use serde_json;

// --- Data Structures ---
// We define our Rust structs to mirror the structure of the JSON we expect.
// The `#[derive]` attribute automatically implements the necessary traits from Serde.
// We also derive `Debug` so we can print the structs easily.

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u64,
    username: String,
    email: Option<String>, // Using Option for fields that might be null or missing.
    is_active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Article {
    title: String,
    author_id: u64,
    tags: Vec<String>,
    content: String,
}

fn main() {
    println!("--- Lesson 17: Working with JSON using Serde ---\n");

    // --- 1. Deserialization: JSON String -> Rust Structs ---
    println!("--- 1. Deserializing a JSON string into Rust structs ---");

    // A raw string literal `r#""#` is useful for multi-line strings with quotes.
    let json_data = r#"
    [
        {
            "id": 101,
            "username": "coder_jane",
            "email": "jane.doe@example.com",
            "is_active": true
        },
        {
            "id": 205,
            "username": "rustacean_ralph",
            "email": null,
            "is_active": false
        }
    ]
    "#;

    println!("Original JSON data:\n{}", json_data);

    // `serde_json::from_str` attempts to parse the string. It returns a Result.
    // We specify that we expect a `Vec<User>`.
    let users: Result<Vec<User>, _> = serde_json::from_str(json_data);

    match users {
        Ok(parsed_users) => {
            println!(
                "\nSuccessfully parsed JSON into a Vec<User>:\n{:#?}",
                parsed_users
            );
            // We can now work with this as normal Rust data.
            for user in parsed_users {
                if user.is_active {
                    println!("  - Active user found: {}", user.username);
                }
            }
        }
        Err(e) => {
            println!("\nFailed to parse JSON: {}", e);
        }
    }

    // --- 2. Serialization: Rust Struct -> JSON String ---
    println!("\n--- 2. Serializing a Rust struct into a JSON string ---");

    // Let's create a new Article instance in our Rust code.
    let new_article = Article {
        title: String::from("Mastering Serde in Rust"),
        author_id: 101,
        tags: vec![
            String::from("rust"),
            String::from("json"),
            String::from("serde"),
        ],
        content: String::from("Serde is a powerful framework..."),
    };

    println!("\nOriginal Rust struct:\n{:#?}", new_article);

    // `serde_json::to_string_pretty` serializes the Rust struct into a nicely
    // formatted JSON string. It also returns a Result.
    match serde_json::to_string_pretty(&new_article) {
        Ok(json_string) => {
            println!(
                "\nSuccessfully serialized struct into pretty JSON:\n{}",
                json_string
            );
        }
        Err(e) => {
            println!("\nFailed to serialize struct to JSON: {}", e);
        }
    }

    println!("\n--- End of Lesson 17 ---");
    println!("Congratulations on finishing Part 3! You can now test your code, work with the filesystem, and handle a major data format.");
}
