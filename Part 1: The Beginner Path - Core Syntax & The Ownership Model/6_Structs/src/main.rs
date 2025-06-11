/**
 * @file 6_Structs/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 6: Creating your own custom data types with `struct`.
 *
 * ## Modeling Your Domain: Structs
 *
 * So far, we've worked with Rust's built-in types. But to build complex applications,
 * we need to create our own custom types that represent the concepts in our program's
 * domain (e.g., a `User`, a `Product`, a `BlogPost`).
 *
 * Structs (short for "structures") are how we do this. They allow you to group related
 * data together and give the pieces meaningful names.
 *
 * ### Key Concepts in this Lesson:
 * - **Defining Structs:** How to define a "classic" struct with named fields.
 * - **Instantiating Structs:** How to create an instance of a struct.
 * - **Dot Notation:** How to access and modify the data in a struct's fields.
 * - **Methods & `impl`:** How to attach behavior (functions) to your structs. We'll
 *   learn the difference between a method (takes `&self`) and an associated function
 *   (like a static method or constructor).
 * - **Tuple Structs:** A concise version of a struct for when field names are redundant.
 * - **`#[derive(Debug)]`:** A handy "attribute" that lets us easily print a struct for
 *   debugging purposes.
 *
 * ### How to Run This Program:
 * - `cargo run`
 */

// We can add this "attribute" to a struct to allow it to be printed for debugging.
// We'll see this in action in the `main` function.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// To add BEHAVIOR to a struct, we use an `impl` block (short for "implementation").
// All the functions defined within this block are "associated" with the `Rectangle` struct.
impl Rectangle {
    // This is a METHOD.
    // Methods always have `&self`, `&mut self`, or `self` as their first parameter.
    // `&self` is a shorthand for `self: &Self`, where `Self` is the type the `impl`
    // block is for (in this case, `Rectangle`). It's an immutable borrow.
    fn area(&self) -> u32 {
        // We access the fields of the struct instance using dot notation.
        self.width * self.height
    }

    // A method can have other parameters too.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // This is an ASSOCIATED FUNCTION, not a method, because it does not take `self`.
    // These are often used as "constructors" that create a new instance of the struct.
    // They are called using `::` syntax (e.g., `Rectangle::square(30)`).
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// A "classic" C-style struct with named fields.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// A "tuple struct". Useful when the field names would be redundant.
// It behaves like a tuple but is its own distinct type.
struct Color(u8, u8, u8); // (R, G, B)
struct Point(i32, i32, i32); // (x, y, z)

fn main() {
    println!("--- Lesson 6: Structs ---\n");

    // --- 1. Instantiating a Classic Struct ---
    println!("--- 1. Creating Struct Instances ---");
    let mut user1 = User {
        email: String::from("user1@example.com"),
        username: String::from("userone"),
        active: true,
        sign_in_count: 1,
    };

    // Access fields using dot notation.
    println!("User '{}' has email: {}", user1.username, user1.email);

    // If the instance is mutable, you can change its fields.
    user1.sign_in_count = 2;
    println!("User sign-in count updated to: {}", user1.sign_in_count);

    // --- 2. Tuple Structs ---
    println!("\n--- 2. Using Tuple Structs ---");
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Access fields using dot notation and the index.
    println!("The color black has Red value: {}", black.0);
    println!("The origin point has y-coordinate: {}", origin.1);

    // --- 3. Methods and Associated Functions ---
    println!("\n--- 3. Methods and Associated Functions ---");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    // Use the `Debug` trait to print the struct instance.
    // `:#?` is "pretty-print" debug format.
    println!("rect1 is:\n{:#?}", rect1);

    // Call the `area` METHOD on our `rect1` instance.
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // Call the `can_hold` METHOD.
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));

    // Call the `square` ASSOCIATED FUNCTION using `::` syntax.
    let sq = Rectangle::square(25);
    println!(
        "\nCreated a square using an associated function:\n{:#?}",
        sq
    );
    println!("The area of the square is {}.", sq.area());

    println!("\n--- End of Lesson 6 ---");
}
