/**
 * @file 2_VariablesAndPrimitives/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 2: Storing data with variables and understanding Rust's basic types.
 *
 * ## Building Blocks: Variables and Primitive Types
 *
 * In this lesson, we explore the absolute fundamentals of any programming language: how to
 * store and manage information. We'll see how Rust handles variables and introduces its
 * strict but helpful rules.
 *
 * ### Key Concepts in this Lesson:
 * - **`let` and Immutability:** Learn why variables in Rust are immutable (unchangeable)
 *   by default and how this helps write safer code.
 * - **`mut` for Mutability:** Understand how to explicitly make a variable mutable
 *   (changeable) when you need to.
 * - **Shadowing:** Discover a powerful feature where you can declare a new variable with
 *   the same name as a previous one, even changing its type.
 * - **Scalar Types (the "primitives"):** The single-value types.
 *   - **Integers:** Whole numbers (e.g., `i32`, `u64`).
 *   - **Floating-Point:** Numbers with a decimal point (e.g., `f64`).
 *   - **Booleans:** `true` or `false`.
 *   - **Characters:** A single Unicode character (e.g., 'a', '🚀').
 * - **Compound Types:** Types that group multiple values.
 *   - **Tuples:** A fixed-size collection of values of varying types.
 *
 * ### How to Run This Program:
 * 1. Navigate to the `2_VariablesAndPrimitives` directory in your terminal.
 * 2. Run the command: `cargo run`
 *
 * All the output is organized with `println!` macros. Read the code and comments,
 * then run the program to see the results.
 */

fn main() {
    // This is the main function where our program's execution begins.

    println!("--- Lesson 2: Variables and Primitives ---\n");

    // --- 1. Variables and Mutability ---
    println!("--- 1. Variables and Mutability ---");

    // In Rust, variables are IMMUTABLE by default. This means once a value is bound
    // to a name, you cannot change that value. This is a core safety feature.
    let x = 5;
    println!("The value of immutable x is: {}", x);

    // If you uncomment the next line and run `cargo check`, the compiler will give an error!
    // x = 6; // error[E0384]: cannot assign twice to immutable variable `x`
    // This is great! The compiler prevents us from changing values we didn't intend to.

    // To make a variable MUTABLE, you must use the `mut` keyword.
    let mut y = 10;
    println!("The initial value of mutable y is: {}", y);
    y = 20; // This is allowed because `y` was declared with `mut`.
    println!("The new value of mutable y is: {}", y);

    println!("\n--- 2. Shadowing ---");

    // Rust allows you to "shadow" a variable. This is different from marking a
    // variable as `mut`. Shadowing lets you declare a NEW variable with the same
    // name. The old variable is "shadowed" by the new one.
    let z = 5;
    println!("The initial value of z is: {}", z);

    // We can shadow `z` by using `let` again.
    let z = z + 1; // The new `z` is bound to the result of the old `z` + 1.
    println!("The shadowed value of z is: {}", z);

    // A key advantage of shadowing is that you can change the TYPE of the variable,
    // something you CANNOT do with a `mut` variable.
    let spaces = "   "; // `spaces` is a string type
    let spaces = spaces.len(); // `spaces` is now shadowed by a number type (usize)
    println!("The number of spaces is: {}", spaces);
    // You couldn't do this with `mut`:
    // let mut mut_spaces = "   ";
    // mut_spaces = mut_spaces.len(); // Error: expected `&str`, found `usize`

    println!("\n--- 3. Scalar (Primitive) Types ---");

    // Scalar types represent a single value.

    // INTEGER: A number without a fractional component.
    // `i32` is the default integer type (`i` for signed, `32` for 32-bit).
    let my_integer: i32 = -500; // Explicit type annotation
    let my_unsigned_integer: u64 = 10_000; // `u` for unsigned (can't be negative). `_` is a visual separator.
    println!(
        "Integer: {}, Unsigned Integer: {}",
        my_integer, my_unsigned_integer
    );

    // FLOATING-POINT: A number with a decimal.
    // `f64` is the default. `f32` is also available.
    let my_float: f64 = 3.14159;
    println!("Float: {}", my_float);

    // BOOLEAN: `true` or `false`.
    let is_learning_rust: bool = true;
    println!("Am I learning Rust? {}", is_learning_rust);

    // CHARACTER: A single Unicode scalar value, denoted by single quotes.
    // This means it can represent much more than just ASCII.
    let my_char: char = 'Z';
    let my_emoji: char = '🦀'; // Yes, emojis are chars!
    println!("Character: {}, Emoji Character: {}", my_char, my_emoji);

    println!("\n--- 4. Compound Types: Tuples ---");

    // Tuples have a fixed length and can hold multiple values of DIFFERENT types.
    // They are a simple way to group data together.
    let my_tuple: (i32, f64, char) = (42, 6.28, '✅');
    println!("My entire tuple: {:?}", my_tuple); // Use `:?` to "debug print" the whole tuple.

    // You can "destructure" a tuple to get the values out into separate variables.
    let (answer, pi, status) = my_tuple;
    println!(
        "Destructured from tuple -> Answer: {}, Pi: {}, Status: {}",
        answer, pi, status
    );

    // You can also access tuple elements directly by their index, starting from 0.
    let answer_by_index = my_tuple.0;
    let pi_by_index = my_tuple.1;
    let status_by_index = my_tuple.2;
    println!(
        "Accessed by index -> Answer: {}, Pi: {}, Status: {}",
        answer_by_index, pi_by_index, status_by_index
    );

    println!("\n--- End of Lesson 2 ---");
    // Feel free to change the values, try to re-assign immutable variables,
    // and see what happens when you run `cargo run`!
}
