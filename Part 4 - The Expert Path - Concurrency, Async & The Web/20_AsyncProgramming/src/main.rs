/**
 * @file 20_AsyncProgramming/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 20: Writing high-performance, non-blocking code with `async`/`.await`.
 *
 * ## Unlock Performance: Asynchronous Programming
 *
 * Traditional threads are great for CPU-bound work, but what about tasks that spend most
 * of their time waiting for something, like a network request or a database query?
 * A blocking thread just sits there, consuming resources. Asynchronous programming is the
 * solution.
 *
 * With `async`, a single thread can manage hundreds or thousands of tasks. When one task
 * needs to wait, it yields control, and the thread moves on to run another task. This
is incredibly efficient for I/O-bound applications like web servers.
 *
 * ### Key Concepts in this Lesson:
 * - **`async fn`:** A function that is asynchronous. Instead of blocking, it returns a
 *   `Future`. A `Future` is a placeholder for a value that will be ready later.
 * - **`.await`:** The keyword used to pause an `async fn` until its `Future` has resolved.
 *   Crucially, this pause is non-blocking; the thread is free to run other tasks.
 * - **Runtime (`tokio`):** `async` code doesn't do anything by itself. It needs an
 *   "executor" or "runtime" to run the `Future`s, poll them, and drive them to
 *   completion. `tokio` is the most popular async runtime in Rust.
 * - **`#[tokio::main]`:** A convenient macro that sets up the `tokio` runtime and turns
 *   our `main` function into an async entry point.
 * - **`tokio::join!`:** A macro that lets us run multiple `Future`s concurrently and
 *   wait for them all to finish.
 * - **Non-blocking I/O:** Using `tokio::time::sleep` instead of `std::thread::sleep` to
 *   simulate waiting without blocking the thread.
 *
 * ### How to Run This Program:
 * - `cargo run`
 */

 use tokio::time::{self, Duration, Instant};

 // This is an async function. The `async` keyword transforms it.
 // Instead of returning `()`, it now returns a `Future<Output = ()>`.
 async fn learn_song() {
     println!("Learning the song lyrics...");
     // `tokio::time::sleep` is an async version of `thread::sleep`.
     // It pauses this task but allows the runtime to work on other tasks.
     time::sleep(Duration::from_millis(500)).await;
     println!("-> Finished learning song!");
 }
 
 async fn sing_song() {
     println!("Singing the song...");
     time::sleep(Duration::from_millis(600)).await;
     println!("-> Finished singing song!");
 }
 
 async fn dance() {
     println!("Dancing...");
     time::sleep(Duration::from_millis(700)).await;
     println!("-> Finished dancing!");
 }
 
 // The `#[tokio::main]` macro sets up the Tokio runtime for us.
 // It allows our `main` function to be `async`.
 #[tokio::main]
 async fn main() {
     println!("--- Lesson 20: Async Programming with Tokio ---\n");
     let start_time = Instant::now();
 
     // --- 1. Running async functions sequentially ---
     println!("--- 1. Sequential Execution ---");
     // When we `.await` one after another, the program behaves like synchronous code.
     // Each function must complete before the next one starts.
     learn_song().await;
     sing_song().await;
     dance().await;
     println!("Sequential execution took: {:?}\n", start_time.elapsed());
     // Total time should be roughly 500ms + 600ms + 700ms = 1800ms.
 
     
     // --- 2. Running async functions concurrently ---
     println!("--- 2. Concurrent Execution ---");
     let start_time_concurrent = Instant::now();
     
     // The `tokio::join!` macro takes multiple futures and runs them concurrently.
     // It will wait until all of them have completed.
     // The tasks are all started, and the runtime switches between them when one
     // needs to wait (i.e., when it hits an `.await`).
     tokio::join!(
         learn_song(),
         sing_song(),
         dance()
     );
     
     println!("Concurrent execution took: {:?}", start_time_concurrent.elapsed());
     // Total time should be roughly the duration of the LONGEST task (~700ms),
     // because they are all running at the same time!
 
     println!("\n--- End of Lesson 20 ---");
     // `async` is a powerful tool for writing high-performance services.
     // In the next lessons, we'll use this knowledge to build a real database-backed web API.
 }