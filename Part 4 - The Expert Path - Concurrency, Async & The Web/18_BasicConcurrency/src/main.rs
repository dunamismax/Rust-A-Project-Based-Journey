/**
 * @file 18_BasicConcurrency/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 18: Running code in parallel with threads.
 *
 * ## Fearless Concurrency: The Basics
 *
 * Concurrency is when different parts of a program execute independently. Parallelism is
 * when they execute at the same time (e.g., on different CPU cores). Rust provides
 * powerful tools for both, with a strong focus on safety. Many languages have bugs
 * related to how threads share data, but Rust's ownership system prevents entire classes
 * of these bugs at compile time. This is "fearless concurrency."
 *
 * In this lesson, we'll start with the most basic building block: creating a new thread
 * to run some code.
 *
 * ### Key Concepts in this Lesson:
 * - **`thread::spawn`:** The function used to create a new thread. It takes a "closure"
 *   containing the code that the new thread will run.
 * - **`JoinHandle`:** The return value from `thread::spawn`. It acts as a handle to the
 *   newly created thread, most importantly allowing us to wait for it to finish.
 * - **The `move` Keyword:** We'll see why the `move` keyword is essential for closures
 *   used in threads, as it gives the new thread ownership of the data it needs to work with.
 * - **Race Conditions:** We'll see what happens when the main thread finishes before a
 *   spawned thread, and how to prevent it.
 *
 * ### How to Run This Program:
 * - `cargo run`
 */
use std::thread;
use std::time::Duration;

fn main() {
    println!("--- Lesson 18: Basic Concurrency ---\n");

    // --- 1. Spawning a Thread ---
    println!("--- 1. Spawning a thread ---");
    println!("(Note: The main thread might finish before the spawned thread prints!)");

    // `thread::spawn` takes a closure as an argument.
    // This new thread will execute the code inside the closure.
    thread::spawn(|| {
        for i in 1..=5 {
            println!("  -> Spawned thread says: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // The main thread continues executing its own code.
    for i in 1..=3 {
        println!("Main thread says: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    // What happens here? When the `main` function ends, the program terminates,
    // even if the spawned thread hasn't finished its work yet!
    // The output is unpredictable. Try running it a few times.

    // --- 2. Waiting for Threads with a `JoinHandle` ---
    println!("\n--- 2. Using a JoinHandle to wait for a thread ---");

    // This time, we store the return value of `thread::spawn` in a variable.
    // This value is a `JoinHandle`.
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("  -> (Joined) Spawned thread says: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // We can do other work in the main thread while the spawned one runs...
    for i in 1..=3 {
        println!("(Joined) Main thread says: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Now, we call the `join()` method on the handle.
    // This blocks the main thread from continuing (and exiting) until the
    // spawned thread that `handle` represents has finished its execution.
    // `.unwrap()` is used because `join` returns a Result (it could fail if the thread panicked).
    handle.join().unwrap();
    println!("Spawned thread has finished. Main thread will now exit.");

    // --- 3. Using `move` Closures with Threads ---
    println!("\n--- 3. Using `move` to transfer ownership to a thread ---");

    let v = vec![1, 2, 3];

    // If we try to use `v` inside the thread's closure without `move`, Rust complains.
    // let handle_fail = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v); // ERROR! `v` does not live long enough.
    // });
    // The compiler doesn't know how long the spawned thread will run, so it can't be sure
    // that the reference to `v` will be valid.

    // The solution is the `move` keyword before the closure.
    // This forces the closure to take OWNERSHIP of the values it uses from the environment,
    // such as `v`. The vector data is literally "moved" into the new thread.
    let handle_move = thread::spawn(move || {
        println!("  -> (Moved) Spawned thread now owns the vector: {:?}", v);
    });

    // `v` is no longer valid in the main thread; ownership has been transferred.
    // println!("{:?}", v); // ERROR! borrow of moved value: `v`

    handle_move.join().unwrap();

    println!("\n--- End of Lesson 18 ---");
    // Next, we'll see how to safely SHARE data between threads, not just move it.
}
