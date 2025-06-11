/**
 * @file 14_FileIO/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 14: Persisting data by reading from and writing to files.
 *
 * ## Interacting with the World: File I/O
 *
 * Most applications need to persist data between runs. The most fundamental way to do
 * this is by reading from and writing to files on the computer's filesystem. Rust's
 * standard library provides excellent, safe, and robust tools for this.
 *
 * Since file operations can fail for many reasons (file not found, no permissions, disk
 * full, etc.), all file I/O functions in Rust return a `Result<T, E>`. This forces us
 * to handle potential errors gracefully.
 *
 * ### Key Concepts in this Lesson:
 * - **`std::fs` Module:** The primary module for filesystem operations.
 * - **Convenience Functions:** Using `fs::write` and `fs::read_to_string` for simple,
 *   one-shot file operations. This is the most common and idiomatic approach.
 * - **`std::io::{Read, Write}` Traits:** The underlying traits that provide methods
 *   for reading and writing streams of byte data.
 * - **`File` Struct:** The struct representing a file handle, used for more complex
 *   or manual file operations.
 * - **`Result`-based I/O:** Seeing `Result` and the `?` operator in a very practical,
 *   real-world scenario.
 *
 * ### How to Run This Program:
 * - `cargo run`
 *   When you run this program, it will create a file named `log.txt` in the current
 *   directory, write to it, read from it, print the contents, and then delete it.
 */
// We bring the `fs` module into scope, as well as the `Read` and `Write` traits,
// which provide useful methods on the `File` struct.
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
    println!("--- Lesson 14: File I/O ---\n");
    let filename = "log.txt";

    // --- 1. The Easy Way: `fs::write` ---
    // This is the most convenient way to write an entire buffer to a file.
    // It handles opening/creating the file, writing the data, and closing it.
    println!("--- 1. Writing to a file using `fs::write` ---");
    let content_to_write = "Hello, File System!\nThis is line two.\nAnd a final line.";

    match fs::write(filename, content_to_write) {
        Ok(_) => println!("Successfully wrote content to '{}'", filename),
        Err(e) => println!("Error writing to file: {}", e),
    }

    // --- 2. The Easy Way: `fs::read_to_string` ---
    // This is the most convenient way to read the entire contents of a file into a String.
    println!("\n--- 2. Reading from a file using `fs::read_to_string` ---");
    match fs::read_to_string(filename) {
        Ok(content) => {
            println!("Successfully read content from '{}':", filename);
            println!("--- FILE CONTENT ---");
            println!("{}", content);
            println!("--- END CONTENT ---");
        }
        Err(e) => println!("Error reading from file: {}", e),
    }

    // --- 3. The Manual (but more flexible) Way ---
    // The `run_manual_io` function demonstrates how you would do this with `File` handles.
    // This gives you more control, like keeping a file open for multiple operations.
    println!("\n--- 3. Demonstrating manual I/O (see function below) ---");
    if let Err(e) = run_manual_io("manual_log.txt") {
        println!("Manual I/O function failed with error: {}", e);
    }

    // --- 4. Cleaning Up ---
    println!("\n--- 4. Cleaning up created files ---");
    match fs::remove_file(filename) {
        Ok(_) => println!("Successfully deleted '{}'", filename),
        Err(e) => println!("Error deleting file: {}", e),
    }
    match fs::remove_file("manual_log.txt") {
        Ok(_) => println!("Successfully deleted 'manual_log.txt'"),
        Err(e) => println!("Error deleting file: {}", e),
    }

    println!("\n--- End of Lesson 14 ---");
}

/**
 * @brief Demonstrates the more manual approach to file I/O.
 * This function encapsulates the process of creating, writing to, opening,
 * and reading from a file using `File` handles directly.
 * It returns a Result, so the caller can handle any I/O errors.
 */
fn run_manual_io(filename: &str) -> io::Result<()> {
    // A) Writing to a file manually
    {
        // Use a block to ensure the file is closed after writing.
        // `File::create` opens a file in write-mode. If the file exists, its
        // contents are overwritten. If it doesn't exist, it is created.
        let mut file = File::create(filename)?; // The `?` propagates errors.

        // `write_all` takes a byte slice (`&[u8]`).
        file.write_all(b"Manual write, line 1.\n")?;
        file.write_all(b"Manual write, line 2.\n")?;
        println!("  -> Manual write to '{}' successful.", filename);
    } // `file` goes out of scope here, and the file handle is automatically closed.

    // B) Reading from a file manually
    // `File::open` opens an existing file in read-only mode.
    let mut file = File::open(filename)?;

    // Create an empty string to hold the file's contents.
    let mut contents = String::new();

    // `read_to_string` reads all bytes from the file and appends them to our string.
    file.read_to_string(&mut contents)?;

    println!(
        "  -> Manual read from '{}' successful. Content length: {} bytes.",
        filename,
        contents.len()
    );
    println!("     (Content: '{}')", contents.trim()); // trim to remove trailing newline for printing

    // If we reach here, all operations succeeded. Return the Ok variant.
    Ok(())
}
