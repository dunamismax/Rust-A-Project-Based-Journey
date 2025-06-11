/**
 * @file 3_FunctionsAndControlFlow/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 3: Giving your program logic with functions and control flow.
 *
 * ## Giving Code Structure and Brains
 *
 * Now that we know how to store data, let's learn how to organize our code and make it
 * perform tasks and decisions. This lesson covers two essential concepts: functions for
 * reusable logic and control flow for making choices.
 *
 * ### Key Concepts in this Lesson:
 * - **`fn`:** How to declare functions, pass them data (parameters), and get data back
 *   (return values).
 * - **Statements vs. Expressions:** A crucial distinction in Rust. Expressions *evaluate*
 *   to a value, while statements *perform* an action. This has a big impact on how you
 *   write concise code.
 * - **`if-else` Conditionals:** The classic way to make your program choose a path based
 *   on a condition. We'll see how `if` is a powerful expression in Rust.
 * - **Loops (`loop`, `while`, `for`):** The three ways to make your program repeat actions,
 *   each with its own best use case.
 *
 * ### How to Run This Program:
 * 1. Navigate to the `3_FunctionsAndControlFlow` directory in your terminal.
 * 2. Run the command: `cargo run`
 */

fn main() {
    println!("--- Lesson 3: Functions and Control Flow ---\n");

    // --- 1. Calling Functions ---
    println!("--- 1. Calling Functions ---");

    // We can call other functions from `main` to keep our code organized.
    greet("Rustacean"); // "Rustacean" is the argument we pass to the function.

    // Functions can return values, which we can store in variables.
    let five = give_me_five();
    println!("The function `give_me_five` returned: {}", five);

    // We can pass the result of one function as an argument to another.
    let result = add_one(five);
    println!("The number {} plus one is: {}", five, result);

    // --- 2. Control Flow with if-else ---
    println!("\n--- 2. Control Flow with if-else ---");
    let number = 7;

    // An `if` expression checks a boolean condition.
    if number < 10 {
        println!("Condition was true: {} is less than 10.", number);
    } else {
        println!("Condition was false: {} is not less than 10.", number);
    }

    // Because `if` is an EXPRESSION, you can use it on the right side of a `let` statement.
    // The types in each block must be the same (here, both are `&str`).
    let a_message = if number % 2 == 0 {
        "The number is even."
    } else {
        "The number is odd."
    };
    println!("{}", a_message);

    // --- 3. Repetition with Loops ---
    println!("\n--- 3. Repetition with Loops ---");

    // A) The `loop` keyword creates an infinite loop. You must use `break` to exit.
    println!("A) `loop` with a `break`:");
    let mut counter = 0;
    loop {
        counter += 1;
        print!("{} ", counter); // `print!` prints without a newline.
        if counter >= 5 {
            break; // Exit the loop
        }
    }
    println!(); // Print a newline for clean formatting.

    // B) A `while` loop runs as long as its condition is true.
    println!("B) `while` loop:");
    let mut countdown = 3;
    while countdown != 0 {
        println!("{}...", countdown);
        countdown -= 1;
    }
    println!("LIFTOFF!!!");

    // C) A `for` loop is the most common and safest loop in Rust. It iterates over
    //    a collection of items. Here, `1..4` creates a range from 1 up to (but not including) 4.
    println!("C) `for` loop over a range:");
    for i in 1..4 {
        // The range is `1, 2, 3`
        println!("Iteration {}", i);
    }

    println!("\n--- End of Lesson 3 ---");
}

// --- Function Definitions ---

/**
 * @brief A simple function that takes one argument.
 * @param name This is a "string slice" (`&str`), a common way to pass text.
 *        We will cover `&str` in detail in the Ownership lessons.
 */
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

/**
 * @brief A function that returns a value.
 * The `-> i32` syntax declares the return type of the function.
 * In Rust, the last line of a function is automatically returned if it doesn't have
 * a semicolon. This is an "expression-based" return, very common in idiomatic Rust.
 * @return The integer `5`.
 */
fn give_me_five() -> i32 {
    5 // No semicolon means this value is returned.
}

/**
 * @brief A function that takes a number and returns that number plus one.
 * @param number The input `i32` integer.
 * @return The input number incremented by 1.
 */
fn add_one(number: i32) -> i32 {
    // We could also write `return number + 1;`. The `return` keyword is explicit.
    // However, for the final expression in a function, omitting `return` and the
    // semicolon is more idiomatic.
    number + 1
}
