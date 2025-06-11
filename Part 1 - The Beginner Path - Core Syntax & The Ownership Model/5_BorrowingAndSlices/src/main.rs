/**
 * @file 5_BorrowingAndSlices/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 5: Accessing data without taking ownership via Borrowing and Slices.
 *
 * ## The Solution to Ownership: Borrowing
 *
 * In the last lesson, we saw that passing a `String` to a function moves its ownership,
 * making the original variable invalid. This is often not what we want. What if we just
 * want a function to *use* a value but not own it?
 *
 * The answer is **Borrowing**. We can create *references* to a value, which allows us to
 * access the data without taking ownership.
 *
 * ### The Rules of References:
 * 1. At any given time, you can have EITHER *one* mutable reference OR *any number* of
 *    immutable references.
 * 2. References must *always* be valid (the compiler prevents "dangling references").
 *
 * ### Key Concepts in this Lesson:
 * - **Immutable References (`&T`):** How to "borrow" a value for read-only access.
 * - **Mutable References (`&mut T`):** How to borrow a value to modify it.
 * - **The Borrow Checker:** How Rust's compiler enforces the rules of references at
 *   compile time.
 * - **Slices (`&[T]` and `&str`):** A special kind of reference that lets you refer to a
 *   contiguous sequence of elements in a collection, like a portion of a `String` or an array.
 *
 * ### How to Run This Program:
 * - `cargo run`
 * - As always, try uncommenting the error lines to see the compiler's helpful messages.
 */

fn main() {
    println!("--- Lesson 5: Borrowing and Slices ---\n");

    // --- 1. Immutable Borrows / References (`&`) ---
    println!("--- 1. Immutable References (&) ---");

    let s1 = String::from("hello");

    // Instead of passing `s1` (which would move it), we pass `&s1`.
    // This creates a reference to `s1` but does not move ownership.
    let len = calculate_length(&s1);

    println!("The string is '{}' and its length is {}.", s1, len);
    // Notice `s1` is still perfectly valid here! We can still use it because we
    // never moved ownership away from it.

    // --- 2. Mutable Borrows / References (`&mut`) ---
    println!("\n--- 2. Mutable References (&mut) ---");

    // To borrow mutably, the variable itself must be mutable.
    let mut s2 = String::from("world");
    println!("The original string is: '{}'", s2);

    // We pass a mutable reference `&mut s2` to the function.
    change_string(&mut s2);

    // The original string has been changed by the function.
    println!("The modified string is: '{}'", s2);

    // THE "ONE MUTABLE REFERENCE" RULE:
    // You can only have one mutable reference to a particular piece of data in a scope.
    let r1 = &mut s2;
    // let r2 = &mut s2; // ERROR! Uncommenting this line fails to compile.
    // error[E0499]: cannot borrow `s2` as mutable more than once at a time
    println!("The first mutable borrow holds: '{}'", r1);
    // This rule prevents "data races" at compile time. Data races can happen when:
    // - Two or more pointers access the same data at the same time.
    // - At least one of the pointers is being used to write to the data.
    // - There’s no mechanism being used to synchronize access to the data.

    // --- 3. Mixing Mutable and Immutable References ---
    println!("\n--- 3. Mixing Reference Types ---");
    let mut s3 = String::from("mixing is tricky");

    let r_immut1 = &s3; // An immutable reference.
    let r_immut2 = &s3; // Totally fine, we can have many immutable references.
    println!("Immutable borrows: '{}' and '{}'", r_immut1, r_immut2);

    // Now, let's try to make a mutable reference while immutable ones exist.
    // let r_mut = &mut s3; // ERROR! Uncommenting this fails.
    // error[E0502]: cannot borrow `s3` as mutable because it is also borrowed as immutable
    // You can't have a mutable reference while immutable ones exist because the
    // holders of the immutable references don't expect the value to change.

    // The scopes of the immutable references `r_immut1` and `r_immut2` end after the
    // `println!` where they are last used. After that, we are free to borrow mutably again.
    let r_mut2 = &mut s3;
    r_mut2.push_str("!");
    println!(
        "After immutable borrows ended, we could make a mutable one: '{}'",
        r_mut2
    );

    // --- 4. Slices: References to a Part of a Collection ---
    println!("\n--- 4. Slices ---");

    let sentence = String::from("hello beautiful world");
    println!("Original sentence: '{}'", sentence);

    // `first_word` takes a reference to a String and returns a `&str` (a string slice).
    let word = first_word(&sentence);

    println!("The first word is: '{}'", word);

    // Slices work for other collections too, like arrays.
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let number_slice: &[i32] = &numbers[1..4]; // A slice of elements at index 1, 2, and 3.
    println!("The full array is: {:?}", numbers);
    println!("The slice of the array is: {:?}", number_slice); // `[2, 3, 4]`

    println!("\n--- End of Lesson 5 ---");
}

// This function takes a REFERENCE to a String, so it doesn't take ownership.
fn calculate_length(s: &String) -> usize {
    s.len()
} // `s` goes out of scope, but because it does not have ownership, nothing happens.

// This function takes a MUTABLE REFERENCE to a String.
fn change_string(s: &mut String) {
    s.push_str(", changed"); // `push_str` appends a literal to a String.
}

/**
 * @brief Finds the first word in a string.
 * @param s A reference to a String.
 * @return A string slice (`&str`) containing the first word.
 *
 * A string slice is a reference to part of a String.
 */
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // Convert String to an array of bytes.

    // `iter().enumerate()` gives us both the index and the element.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // If we find a space...
            return &s[0..i]; // ...return a slice from the start to the space.
        }
    }

    &s[..] // If no space is found, the whole string is one word. Return a slice of the full string.
}
