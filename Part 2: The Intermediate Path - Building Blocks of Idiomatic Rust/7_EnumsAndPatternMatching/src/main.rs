/**
 * @file 7_EnumsAndPatternMatching/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 7: Mastering one of Rust's most powerful features for robust data modeling.
 *
 * ## Expressive Types: Enums and Pattern Matching
 *
 * Enums (short for enumerations) allow you to define a type by enumerating its possible
 * *variants*. Where a struct lets you group related fields *together*, an enum lets you say
 * a value is *one of a possible set* of things.
 *
 * In Rust, enums are incredibly powerful because each variant can also hold its own unique
 * data. This allows you to express complex states in a way that the compiler understands.
 *
 * The `match` keyword is the primary way to interact with enums. It lets you run
 * different code for different variants and forces you to handle *every single possibility*,
 * which eliminates a whole class of bugs.
 *
 * ### Key Concepts in this Lesson:
 * - **`enum`:** Defining simple and complex enumerations with data.
 * - **`match`:** The exhaustive control flow operator for matching patterns.
 * - **`Option<T>`:** The most important enum in Rust for handling values that can be absent.
 *   This is how Rust avoids `null` pointer errors. It has two variants: `Some(T)` and `None`.
 * - **`Result<T, E>`:** The second most important enum, used for error handling. It has
 *   two variants: `Ok(T)` for success and `Err(E)` for failure. (Lesson 9 will dive deeper).
 * - **`if let`:** A concise way to handle a single pattern from a `match`.
 * - **The `_` Placeholder:** A catch-all pattern for a `match` arm.
 *
 * ### How to Run This Program:
 * - `cargo run`
 */

// --- 1. A Simple Enum ---
// Here we define a `Direction`. An instance of `Direction` can only be one of these four things.
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// --- 2. An Enum with Data ---
// Each variant can hold different types and amounts of data.
// This single `Message` enum can encode several different kinds of events.
#[derive(Debug)] // So we can print it for debugging
enum Message {
    Quit,                    // Has no data associated with it.
    Move { x: i32, y: i32 }, // Has named fields, like a struct.
    Write(String),           // Includes a single String.
    ChangeColor(u8, u8, u8), // Includes three u8 values.
}

// A function that processes a `Message` enum using a `match` statement.
fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and y direction {}.", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}.",
                r, g, b
            );
        }
    }
}

// --- 3. The `Option<T>` Enum ---
// `Option` is defined by the standard library like this:
// enum Option<T> {
//   Some(T), // Contains a value of type T
//   None,    // Does not contain a value
// }
// It is so common it's brought into scope automatically.

// This function safely adds one to a number that might not exist.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,           // If there's no value, we can't add one, so we return None.
        Some(i) => Some(i + 1), // If there is a value `i`, we wrap the result in a new `Some`.
    }
}

fn main() {
    println!("--- Lesson 7: Enums and Pattern Matching ---\n");

    println!("--- 1. Using a `match` with a simple enum ---");
    let player_direction = Direction::Up;

    // The `match` statement will compare the value of `player_direction`
    // against the pattern in each "arm". It's exhaustive; you MUST cover all variants.
    match player_direction {
        Direction::Up => println!("Player is moving up!"),
        Direction::Down => println!("Player is moving down!"),
        Direction::Left => println!("Player is moving left!"),
        Direction::Right => println!("Player is moving right!"),
    }

    println!("\n--- 2. Using an enum with data ---");
    let messages = vec![
        Message::Write(String::from("hello")),
        Message::Move { x: 10, y: -5 },
        Message::ChangeColor(255, 0, 128),
        Message::Quit,
    ];
    for msg in messages {
        println!("Processing message: {:?}", msg);
        process_message(msg);
    }

    println!("\n--- 3. Using the `Option<T>` enum ---");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("The value of `five` is: {:?}", five);
    println!("`plus_one(five)` returns: {:?}", six);
    println!("`plus_one(None)` returns: {:?}", none);

    println!("\n--- 4. `if let` for concise matching ---");
    let some_value = Some(3u8);

    // Imagine you want to do something if `some_value` is `Some(3)`, but nothing otherwise.
    // You could write a `match` like this:
    match some_value {
        Some(3) => println!("The value is three!"),
        _ => (), // The `_` is a catch-all for any other pattern. `()` is the "unit type", meaning "do nothing".
    }

    // But `if let` is much more concise for this use case.
    // It's syntax sugar for a `match` that runs code on one pattern while ignoring the rest.
    if let Some(3) = some_value {
        println!("(Using if let) The value is three!");
    }

    println!("\n--- End of Lesson 7 ---");
}
