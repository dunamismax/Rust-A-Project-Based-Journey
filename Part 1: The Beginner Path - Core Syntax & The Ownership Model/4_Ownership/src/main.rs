/**
 * @file 4_Ownership/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 4: The Ownership Leap! Understanding Rust's unique memory management.
 *
 * ## The Heart of Rust: Ownership
 *
 * Welcome to the most central feature of Rust. Ownership is how Rust achieves its primary
 * goal: memory safety *without* needing a garbage collector (like in Java or Python) or
 * manual memory management (like in C/C++).
 *
 * It might feel different from what you're used to, but once it clicks, you'll see how it
 * enables you to write incredibly performant and safe code.
 *
 * ### The Three Rules of Ownership:
 * 1. Each value in Rust has a variable that’s called its *owner*.
 * 2. There can only be one owner at a time.
 * 3. When the owner goes out of scope, the value will be *dropped* (memory is freed).
 *
 * ### Key Concepts in this Lesson:
 * - **Stack vs. Heap:** We'll briefly review these two memory locations to understand *why*
 *   ownership is necessary for some types of data.
 * - **The `Move` Semantic:** See what happens when ownership is transferred. This is the
 *   default behavior for complex types like `String`.
 * - **The `Clone` Trait:** Learn how to explicitly create a deep copy of data when you
 *   need more than one variable to own its own data.
 * - **Ownership in Functions:** Understand how passing values to functions and returning
 *   values from functions interacts with the ownership system.
 *
 * ### How to Run This Program:
 * - `cargo run`
 * - Try uncommenting the lines that cause errors to see the compiler's messages firsthand!
 */

// This function takes ownership of a String.
// `some_string` comes into scope, and because it is of type `String` (a heap-allocated
// type), it takes ownership of the value passed to it.
fn takes_ownership(some_string: String) {
    println!("Inside `takes_ownership`, I now own: '{}'", some_string);
} // `some_string` goes out of scope here, `drop` is called, and the memory is freed.

// This function "makes a copy" of an integer.
// `some_integer` comes into scope, but because `i32` is a simple type of known, fixed
// size, it implements the `Copy` trait. The value is copied from the stack.
fn makes_copy(some_integer: i32) {
    println!("Inside `makes_copy`, I have a copy of: {}", some_integer);
} // `some_integer` goes out of scope, but nothing special happens to the original value.

fn main() {
    println!("--- Lesson 4: Ownership ---\n");

    // --- 1. Scope and The Stack ---
    println!("--- 1. Scope and The Stack ---");
    // `x` comes into scope. It's a simple `i32`, so it's placed on the stack.
    let x = 10;
    makes_copy(x); // A copy of the value of `x` (10) is passed to the function.
    println!("`x` is still valid after calling `makes_copy`: x = {}", x);
    // `x` is still valid here because `i32` is a `Copy` type. We can continue using it.

    // --- 2. The `String` Type and The Heap ---
    println!("\n--- 2. `String` and the Heap: The `Move` Semantic ---");

    // `s1` comes into scope. `String` is more complex than `i32`.
    // It's made of three parts stored on the stack:
    // 1. A pointer to the memory that holds the contents of the string ("hello").
    // 2. A length (how much memory in bytes the contents are using).
    // 3. A capacity (how much memory the OS has allocated for it).
    // The actual text "hello" is stored on the heap.
    let s1 = String::from("hello");
    println!("s1 has been created: '{}'", s1);

    // Now, we "assign" s1 to s2. What happens here is NOT a copy.
    // Rust *moves* ownership from s1 to s2.
    // The pointer, length, and capacity on the stack are copied for s2.
    // But s1 is now considered INVALIDATED. This prevents a "double free" error, where
    // both s1 and s2 might try to free the same memory when they go out of scope.
    let s2 = s1;
    println!("Ownership was *moved* to s2: '{}'", s2);

    // If you uncomment the line below, the program will NOT compile.
    // `s1` is no longer a valid owner. The compiler enforces this rule for us.
    // println!("Trying to use s1 fails: {}", s1); // error[E0382]: borrow of moved value: `s1`
    println!("'s1' is no longer valid. The borrow checker would prevent its use.");

    // --- 3. The `Clone` Trait: Making a Deep Copy ---
    println!("\n--- 3. `Clone`: Making a True Copy ---");

    let s3 = String::from("world");
    println!("s3 has been created: '{}'", s3);

    // If we want to make a full "deep copy" of the heap data, we use the `clone` method.
    // This is a more expensive operation as it involves allocating new memory on the
    // heap and copying the original data.
    let s4 = s3.clone();
    println!("s4 was cloned from s3: '{}'", s4);

    // Now, both s3 and s4 are valid because they both own their own, separate data.
    println!("s3 is still valid after the clone: '{}'", s3);

    // --- 4. Ownership and Functions ---
    println!("\n--- 4. Ownership and Functions ---");

    let s5 = String::from("I will be moved");
    println!("s5 created: '{}'", s5);

    // We pass `s5` to the `takes_ownership` function.
    // Ownership of the string is MOVED into the function.
    takes_ownership(s5);

    // Just like before, `s5` is no longer valid here because it no longer owns the data.
    // The `takes_ownership` function now owns it, and it was dropped when that function ended.
    // Uncommenting this line will cause a compiler error.
    // println!("Trying to use s5 after move fails: {}", s5); // error[E0382]: borrow of moved value: `s5`
    println!("'s5' is no longer valid as ownership was moved into the function.");

    println!("\n--- End of Lesson 4 ---");
    // Ownership seems restrictive, but it's the key to safety. In the next lesson,
    // we will learn about "Borrowing", which lets us *use* data without taking ownership.
} // Scope ends. `x`, `s2`, `s3`, `s4` go out of scope and are dropped.
