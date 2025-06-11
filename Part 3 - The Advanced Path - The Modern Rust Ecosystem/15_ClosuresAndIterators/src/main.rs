/**
 * @file 15_ClosuresAndIterators/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 15: The functional side of Rust for elegant data processing.
 *
 * ## Embrace Functional Rust: Closures and Iterators
 *
 * Rust is heavily influenced by functional programming. Two core features that enable this
 * are closures and iterators. They allow you to write code that is concise, declarative,
 * and often more performant than a handwritten loop.
 *
 * ### Key Concepts in this Lesson:
 * - **Closures:** Anonymous functions that you can define on the fly, store in a
 *   variable, and pass as arguments to other functions. Crucially, they can "capture"
 *   variables from the scope in which they are defined.
 * - **The `Iterator` Trait:** A trait for types that can produce a sequence of values.
 *   The core method is `next()`, which yields one item at a time.
 * - **Iterator Adaptors:** Methods that transform an iterator into a new iterator with
 *   different behavior (e.g., `map`, `filter`). These are "lazy," meaning they don't do
 *   any work until you consume the iterator.
 * - **Consuming Adaptors:** Methods that consume the iterator and produce a final value
 *   (e.g., `collect`, `sum`).
 * - **Zero-Cost Abstraction:** A key Rust principle. Using iterators and their methods
 *   compiles down to machine code that is just as fast as a manual `for` loop, so you
 *   get high-level expressiveness with no runtime performance penalty.
 *
 * ### How to Run This Program:
 * - `cargo run`
 */

fn main() {
    println!("--- Lesson 15: Closures and Iterators ---\n");

    // --- 1. Closures: Anonymous, Capturing Functions ---
    println!("--- 1. Closures ---");

    let x = 4;
    // This is a closure. It takes one argument `y` and adds the `x` from its
    // environment to it. The type annotations `: i32` are often optional.
    let add_x = |y: i32| y + x;

    let result = add_x(10);
    println!("The closure `add_x(10)` returned: {}", result);

    // Closures can also mutate their environment.
    let mut count = 0;
    // This closure needs to be `mut` because it modifies `count`.
    // It takes no arguments (`||`).
    let mut increment_count = || {
        count += 1;
        println!("The count is now: {}", count);
    };

    increment_count();
    increment_count();

    // --- 2. Iterators: Processing a Sequence of Items ---
    println!("\n--- 2. Iterators ---");
    let numbers = vec![1, 2, 3];

    // `iter()` creates an iterator that yields immutable references (`&i32`).
    // The `for` loop is the most basic consumer of an iterator.
    println!("Iterating with .iter():");
    for num_ref in numbers.iter() {
        println!("  Got &i32: {}", num_ref);
    }

    // --- 3. The Power of Chaining: `map`, `filter`, and `collect` ---
    println!("\n--- 3. Chaining Iterator Methods ---");
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original data: {:?}", data);

    // This is where Rust's functional style shines. We chain methods to perform a
    // sequence of operations. This is often more readable than nested loops and ifs.
    let processed_data: Vec<i32> = data
        .iter() // 1. Create an iterator of `&i32`.
        .filter(|&&n| n % 2 == 0) // 2. Filter: keep only even numbers. `&&n` dereferences twice.
        .map(|&n| n * 3) // 3. Map: multiply each remaining number by 3. `&n` dereferences once.
        .rev() // 4. Another adaptor: reverse the iterator's direction.
        .collect(); // 5. Collect: consume the iterator and create a new `Vec<i32>`.

    println!(
        "Processed data (even numbers, tripled, reversed): {:?}",
        processed_data
    );
    // Expected output: [30, 24, 18, 12, 6]

    // Note on laziness: `filter` and `map` don't actually do any work by themselves.
    // They are "lazy" and only process elements when `collect` (or another consumer)
    // asks for the next item.

    println!("\n--- 4. Consuming Adaptors: `sum` ---");
    let sum_of_processed: i32 = processed_data.iter().sum();
    println!("The sum of the processed data is: {}", sum_of_processed);

    println!("\n--- End of Lesson 15 ---");
}
