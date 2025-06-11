/**
 * @file 1_HelloWorld/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 1: Your First Rust Program.
 *
 * ## The Journey Begins: "Hello, World!"
 *
 * Welcome to the first step in your Rust journey! This lesson is the traditional starting
 * point for learning any new programming language. We will accomplish several fundamental
 * tasks.
 *
 * ### Key Concepts in this Lesson:
 * - **Cargo:** We will use Rust's built-in build tool and package manager, `cargo`, to
 *   create, build, and run our project. Cargo is one of Rust's greatest strengths.
 * - **`fn main()`:** We will define the `main` function, which is the mandatory entry
 *   point for every executable Rust program.
 * - **The `println!` Macro:** We will use the `println!` macro to print text to the
 *   console. You'll learn why it ends with a `!` and how it differs from a regular function.
 * - **Modules & Crates:** This simple program is itself a "crate". The `main.rs` file is the
 *   root file of this binary crate's module structure. We will explore this more later.
 *
 * ### How to Run This Program:
 * 1. Navigate to the `1_HelloWorld` directory in your terminal.
 * 2. Run the command: `cargo run`
 *
 * Cargo will first compile your program (if it has changed) and then execute the
 * resulting binary. You should see "Hello, Rustacean!" printed to your screen.
 */

// Every executable Rust program must have a `main` function.
// `fn` is the keyword used to declare a new function.
// When you run the program, the code inside the `main` function is the first code that runs.
// The empty parentheses `()` indicate that this function takes no arguments (or "parameters").
// The curly braces `{}` define the "body" or scope of the function. All the code for this
// function goes inside these braces.
fn main() {
    // This line does the printing. Let's break it down:
    //
    // - `println!` is a Rust "macro". A macro is a piece of code that writes other code.
    //   In Rust, you can identify a macro by the exclamation mark `!` at the end of its name.
    //   We use `println!` instead of a regular function because it provides compile-time
    //   format string checking and can accept a variable number of arguments.
    //
    // - `"Hello, Rustacean!"` is a "string literal" (specifically, a `&'static str`). It's a
    //   piece of text that is hard-coded into our program's binary. We pass it as the
    //   first argument to the `println!` macro.
    //
    // - The line ends with a semicolon `;`. In Rust, most lines of code ("statements")
    //   must end with a semicolon. This tells the compiler that this expression is finished.
    println!("Hello, Rustacean!");
}

// --- End of File ---
//
// Congratulations! You have just read through your first Rust program.
// Now, let's compile and run it. Open your terminal, navigate to this project's
// directory (e.g., `cd 1_HelloWorld`), and run the command:
//
// > cargo run
//
// You can also try these other useful Cargo commands:
//
// > cargo check
//   (This quickly checks your code for errors without creating an executable binary)
//
// > cargo build
//   (This compiles your code but doesn't run it. The output will be in the `target/debug/` directory)
//
// > cargo build --release
//   (This compiles your code with optimizations for performance. The output will be in `target/release/`)
