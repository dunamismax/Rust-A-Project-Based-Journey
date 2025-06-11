/**
 * @file 11_Lifetimes/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 11: Making promises to the compiler with lifetime annotations.
 *
 * ## Guaranteeing References are Valid: Lifetimes
 *
 * You've already been using lifetimes! The borrow checker analyzes the scopes (lifetimes)
 * of all variables to ensure references never point to deallocated memory (a "dangling
 * reference"). Most of the time, it can do this without any help from you. This is
 * called **Lifetime Elision**.
 *
 * However, sometimes the relationship between the lifetimes of different references is
 * ambiguous. In these cases, the compiler will ask for help. We provide this help by
 * adding **generic lifetime annotations** to our functions.
 *
 * IMPORTANT: Lifetime annotations DO NOT change how long any reference lives. They are
 * purely descriptive; they are a promise we make to the compiler so it can verify that
 * our references are safe.
 *
 * ### Key Concepts in this Lesson:
 * - **Dangling References:** The core problem lifetimes prevent.
 * - **Generic Lifetime Syntax (`'a`):** How to add annotations to function signatures.
 *   The tick `'` is the key character. `'a` is a conventional, but arbitrary, name.
 * - **Lifetimes in Function Signatures:** How to tell the compiler how the lifetimes of
 *   input references relate to the lifetime of an output reference.
 * - **Lifetimes in Struct Definitions:** How to define a struct that holds a reference,
 *   ensuring the struct instance can't outlive the data it refers to.
 * - **The `'static` Lifetime:** A special lifetime for data that lives for the entire
 *   program (e.g., string literals).
 *
 * ### How to Run This Program:
 * - `cargo run`
 */

// A struct that holds a reference. This means it needs a lifetime annotation.
// The `<'a>` after the struct name declares a generic lifetime parameter.
// This annotation means "an instance of `ImportantExcerpt` can't outlive the
// reference it holds in its `part` field."
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// This is the classic example function. It takes two string slices and returns the one that is longest.
// The `<'a>` declares a generic lifetime that we can use to describe the relationship
// between the references.
//
// `x: &'a str` -> x is a reference that lives for at least the duration of lifetime 'a.
// `y: &'a str` -> y is also a reference that lives for at least the duration of 'a.
// `-> &'a str` -> The returned reference is guaranteed to live as long as the SHORTER of 'x' or 'y'.
//
// This signature is our promise to the compiler: the returned reference will be valid
// for as long as both of the input references are valid.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("--- Lesson 11: Lifetimes ---\n");

    println!("--- 1. Lifetimes in Functions ---");

    let string1 = String::from("abcd");
    let string2 = "xyz"; // A string literal (`&'static str`)

    // We can call `longest` with references to these two strings.
    // The compiler sees that both `string1` and `string2` are valid for the
    // current scope, so the returned reference (`result`) will also be valid here.
    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);

    println!("\n--- 2. How Lifetimes Prevent Dangling References ---");

    // Let's try to break the rule we promised the compiler.
    let string3 = String::from("long string is long");
    let result2;

    {
        // An inner scope starts here.
        let string4 = String::from("short");
        // We call `longest` and try to assign the result to `result2`.
        // The lifetime `'a` for this call is the *shorter* of `string3` and `string4`.
        // So, `'a` is the lifetime of `string4`.
        result2 = longest(string3.as_str(), string4.as_str());
        println!("Inside inner scope, longest is: '{}'", result2);
    } // `string4` goes out of scope and is dropped HERE.

    // If the next line were allowed, `result2` would be a dangling reference, because
    // it would be pointing to memory that belonged to `string4`, which no longer exists.
    // The borrow checker uses our lifetime annotations to catch this at compile time!
    // UNCOMMENTING THE LINE BELOW WILL CAUSE A COMPILER ERROR:
    // `string4` does not live long enough
    // println!("The longest string is: {}", result2);
    println!("Cannot use `result2` here because the data it referred to might be gone.");

    println!("\n--- 3. Lifetimes in Structs ---");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a sentence.");

    // We create an instance of our struct. The lifetime of `i` is tied to the
    // lifetime of the data in `first_sentence`.
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // This is perfectly fine because `novel` (which `first_sentence` refers to) is
    // still in scope.
    println!("The important excerpt is: '{}'", i.part);

    println!("\n--- End of Lesson 11 ---");
}

// NOTE: A function that would fail to compile without lifetimes.
//
// fn dangling_reference() -> &String {
//     let s = String::from("hello");
//     &s // We are trying to return a reference to `s`.
// } // But `s` is dropped here, so the reference would be dangling.
//
// The compiler correctly rejects this with the error: "missing lifetime specifier".
// It tells us it doesn't know how to guarantee the returned reference will be valid.
