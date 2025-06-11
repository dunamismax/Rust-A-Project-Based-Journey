/**
 * @file 10_Traits/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 10: Abstracting behavior with traits for generic programming.
 *
 * ## Defining Shared Behavior: Traits
 *
 * A trait tells the Rust compiler about functionality a type must provide. It's a way to
 * define a shared interface across different structs or enums. If you're familiar with
 * interfaces in languages like Java or C#, traits will feel similar, but they have some
 * key differences and additional power.
 *
 * By using traits, we can write functions that operate on any type that implements that
 * trait, allowing for flexible and reusable code.
 *
 * ### Key Concepts in this Lesson:
 * - **`trait`:** How to define a new trait with a set of method signatures.
 * - **`impl Trait for Type`:** How to implement a trait for one of our custom types.
 * - **Default Implementations:** Providing default behavior for a trait's methods that
 *   implementing types can use or override.
 * - **Generic Functions (`impl Trait`):** Writing functions that accept any type that
 *   implements a specific trait, enabling polymorphism.
 * - **Trait Bounds (`<T: Trait>`):** The full, more explicit syntax for generics.
 * - **`derive` Macros:** Revisiting `#[derive(Debug)]` and understanding it as a way the
 *   compiler automatically implements a trait for us.
 *
 * ### How to Run This Program:
 * - `cargo run`
 */

// --- 1. Defining a Trait ---
// We define a trait `Summary` which describes the behavior of providing a summary.
// Any type that wants to be "summarizable" must implement this trait.
pub trait Summary {
    // This is a method signature. It says there must be a method named `summarize`
    // that takes an immutable reference to self and returns a String.
    fn summarize(&self) -> String;

    // We can also provide a DEFAULT implementation.
    // Types can choose to use this implementation or provide their own (override it).
    fn summarize_author(&self) -> String {
        String::from("(Author information not available)")
    }
}

// --- 2. Our Custom Types ---
// Two different structs that we want to be able to summarize.
pub struct Article {
    pub headline: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub is_reply: bool,
}

// --- 3. Implementing the Trait on Our Types ---
// Now we provide the concrete behavior for `Article`.
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("'{}', by {}.", self.headline, self.author)
    }
    // We don't define `summarize_author` here, so `Article` will use the default implementation.
}

// And a different implementation for `Tweet`.
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    // We choose to OVERRIDE the default implementation for `summarize_author`.
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// --- 4. Using Traits in Function Parameters ---
// This function can accept ANY type that implements the `Summary` trait.
// `item: &impl Summary` is concise syntax sugar for a "trait bound".
// This allows for polymorphism.
pub fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize());
}

// The line above is sugar for this more verbose generic function definition,
// which is called a "trait bound":
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking News! {}", item.summarize());
// }
// We'll stick to the `impl Trait` syntax when possible as it's cleaner.

fn main() {
    println!("--- Lesson 10: Traits ---\n");

    let article = Article {
        headline: String::from("Rust is Safe and Fast"),
        author: String::from("Jane Doe"),
        content: String::from("Rust provides memory safety without a garbage collector..."),
    };

    let tweet = Tweet {
        username: String::from("rustacean_dev"),
        content: String::from("I'm loving traits in Rust! #rustlang"),
        is_reply: false,
    };

    println!("--- Calling trait methods directly ---");
    println!("Article Summary: {}", article.summarize());
    println!("Tweet Summary: {}", tweet.summarize());

    println!("\n--- Using default vs. overridden methods ---");
    println!("Article Author: {}", article.summarize_author()); // Uses the default
    println!("Tweet Author: {}", tweet.summarize_author()); // Uses the overridden version

    println!("\n--- Using traits for polymorphism ---");
    println!("Notifying about the article:");
    notify(&article);
    println!("Notifying about the tweet:");
    notify(&tweet);

    println!("\n--- `derive` is just automatic trait implementation ---");
    // Remember `#[derive(Debug)]` from the Structs lesson?
    // That's a macro that automatically generates an `impl std::fmt::Debug for YourType { ... }` block.
    // Traits are absolutely everywhere in Rust!

    println!("\n--- End of Lesson 10 ---");
}
