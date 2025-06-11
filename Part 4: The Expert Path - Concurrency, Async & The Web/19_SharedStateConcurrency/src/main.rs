/**
 * @file 19_SharedStateConcurrency/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 19: Safely sharing data between multiple threads.
 *
 * ## The "Fearless" Way: Sharing State
 *
 * In the last lesson, we *moved* ownership of data into a thread. But what if multiple
 * threads need to access the *same* piece of data? This is called "shared state"
 * concurrency, and it's notoriously difficult to get right. It's the source of "data
 * races," where the final result depends on the unpredictable order in which threads
 * access the data.
 *
 * Rust provides two key smart pointers to solve this problem safely.
 *
 * ### Key Concepts in this Lesson:
 * - **`Mutex<T>` (Mutual Exclusion):** A smart pointer that provides a "lock." At any
 *   given time, only one thread can hold the lock. To access the data `T` inside the
 *   `Mutex`, a thread must first acquire the lock. If another thread already has it,
 *   the current thread will wait until the lock is released.
 * - **`Arc<T>` (Atomically Reference Counted):** Remember `Rc<T>` from the smart
 *   pointers lesson? It lets us have multiple owners of data. `Arc<T>` is its
 *   thread-safe equivalent. The "A" stands for "atomic," meaning its internal reference
 *   count can be safely incremented and decremented by multiple threads at once.
 * - **The `Arc<Mutex<T>>` Pattern:** This is the most common and idiomatic way in Rust
 *   to share mutable state between threads. The `Arc` lets every thread have ownership
 *   of the `Mutex`, and the `Mutex` ensures that only one thread at a time can actually
 *   *access* the data inside.
 *
 * ### How to Run This Program:
 * - `cargo run`
 */
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("--- Lesson 19: Shared State Concurrency ---\n");

    // --- 1. Using a Mutex in a Single Thread ---
    println!("--- 1. Mutex basics (single thread) ---");
    // We create a Mutex that guards an i32.
    let m = Mutex::new(5);
    println!("Initial mutex state: {:?}", m);

    {
        // To access the data, we must call `.lock()`. This acquires the lock.
        // It returns a Result, which we unwrap for this example.
        // The `lock()` call would block if another thread held the lock.
        let mut num = m.lock().unwrap();
        // `num` is a smart pointer (`MutexGuard`) that dereferences to `i32`.
        *num = 6;
        println!("  -> Acquired lock, changed data to: {}", *num);
    } // The lock is automatically RELEASED here when `num` goes out of scope.

    println!("Mutex state after lock was released: {:?}", m);

    // --- 2. The `Arc<Mutex<T>>` Pattern for Multi-Threaded Sharing ---
    println!("\n--- 2. Sharing a Mutex between threads with Arc ---");

    // We wrap our Mutex in an Arc. The data we want to share is a counter.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    println!("Spawning 10 threads, each of which will increment the counter...");

    for i in 0..10 {
        // `Arc::clone` is cheap. It just increments the atomic reference count
        // and gives us a new pointer to the SAME allocation.
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // Each thread locks the mutex, modifies the data, and then the lock
            // is released as the `num` guard goes out of scope.
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("  -> Thread {} incremented counter to {}", i, *num);
        });
        handles.push(handle);
    }

    // Wait for all the threads to finish their work.
    for handle in handles {
        handle.join().unwrap();
    }

    // Now that all threads are done, we can lock the mutex one last time in the
    // main thread to see the final result.
    let final_count = *counter.lock().unwrap();
    println!("\nAll threads finished.");
    println!("Final counter value is: {}", final_count);
    assert_eq!(final_count, 10);

    println!("\n--- End of Lesson 19 ---");
    // This `Arc<Mutex<T>>` pattern is fundamental to traditional shared-state
    // concurrency in Rust. It guarantees that even though the threads run in an
    // unpredictable order, they can't corrupt the counter's state because only
    // one can access it at a time.
}
