use std::cell::RefCell;
/**
 * @file 16_SmartPointers/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Lesson 16: Advanced ownership patterns with smart pointers.
 *
 * ## Beyond Basic Ownership: Smart Pointers
 *
 * Smart pointers are data structures that act like pointers but also have additional
 * metadata and capabilities. They are a key feature in Rust for managing memory beyond
 * the basic ownership rules we've learned so far. They allow for patterns like multiple
 * owners of data or mutating data even when it appears to be immutable.
 *
 * All the smart pointers we'll cover here are for use within a single thread.
 *
 * ### Key Concepts in this Lesson:
 * - **`Box<T>`:** The simplest smart pointer. It allows you to store data on the heap
 *   instead of the stack. Its primary use case is for types whose size can't be known
 *   at compile time (like recursive types).
 * - **`Rc<T>` (Reference Counted):** A smart pointer that enables *multiple ownership*.
 *   It keeps a count of how many owners there are. When the count drops to zero, the
 *   data is cleaned up. This allows data to live until the last reference to it is gone.
 * - **`RefCell<T>` (Interior Mutability):** A smart pointer that enforces the borrowing
 *   rules at *runtime* instead of compile time. This allows you to mutate data even
 *   when there are immutable references to it. If the rules are broken at runtime,
 *   the program will panic.
 *
 * ### How to Run This Program:
 * - `cargo run`
 */
// We need to bring Rc and RefCell into scope. Box is so common it's pre-imported.
use std::rc::Rc;

// --- 1. `Box<T>` for Heap Allocation ---
// This is a "cons list", a classic functional data structure.
// It's defined recursively: a List is either a value plus another List, or nothing.
//
// enum ThisWontWork {
//     Cons(i32, ThisWontWork), // Error: recursive type has infinite size
//     Nil,
// }
//
// The definition above fails because the compiler can't figure out how much memory
// to allocate for a `ThisWontWork`. `Box` solves this. A `Box` is a pointer, and a
// pointer has a known, fixed size.
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// --- 2. `Rc<T>` for Multiple Ownership ---
// Here we modify our List to use `Rc` instead of `Box`.
// This allows multiple lists to share ownership of another list.
#[derive(Debug)]
enum SharedList {
    Cons(i32, Rc<SharedList>),
    Nil,
}

// --- 3. `RefCell<T>` for Interior Mutability ---
// A common use case is in mock objects for testing.
// This trait defines a behavior we want to test.
pub trait Messenger {
    fn send(&self, msg: &str);
}

// Our mock object will implement the trait.
// We want to be able to inspect the messages that were "sent" after the fact.
pub struct MockMessenger {
    // We use `Rc<RefCell<...>>` to allow multiple owners (if needed) to share
    // and mutate the same vector of messages.
    sent_messages: Rc<RefCell<Vec<String>>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: Rc::new(RefCell::new(vec![])),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        // `borrow_mut()` gets a mutable reference to the Vec inside the RefCell.
        // If another mutable borrow already existed, this line would panic!
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

fn main() {
    println!("--- Lesson 16: Smart Pointers ---\n");

    println!("--- 1. Using `Box<T>` to create a recursive List ---");
    // We create a list: 5 -> 10 -> Nil
    // The inner `List::Cons` is wrapped in a `Box` to give it a known size.
    let list = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
    println!("My recursive list: {:?}", list);

    println!("\n--- 2. Using `Rc<T>` for shared ownership ---");
    // `a` is a list: 5 -> 10 -> Nil
    let a = Rc::new(SharedList::Cons(
        5,
        Rc::new(SharedList::Cons(10, Rc::new(SharedList::Nil))),
    ));
    println!("Created list 'a' with ref count: {}", Rc::strong_count(&a));

    // `Rc::clone(&a)` DOES NOT do a deep copy. It just increments the reference count
    // and gives us a new pointer to the same data. This is very fast.
    let b = SharedList::Cons(3, Rc::clone(&a)); // `b` now points to `a`.
    let c = SharedList::Cons(4, Rc::clone(&a)); // `c` also points to `a`.

    println!(
        "After creating 'b' and 'c' pointing to 'a', ref count for 'a' is: {}",
        Rc::strong_count(&a)
    );
    println!("List b: {:?}", b);
    println!("List c: {:?}", c);

    // When `b` and `c` go out of scope, the count will decrease. When `a` goes out
    // of scope, the count will drop to 0 and the list data will be deallocated.

    println!("\n--- 3. Using `RefCell<T>` for a mock object test ---");
    let mock_messenger = MockMessenger::new();

    // We can call `send` on an immutable reference (`&mock_messenger`),
    // yet the internal data (`sent_messages`) is being changed.
    // This is the power of "interior mutability".
    mock_messenger.send("First message");
    mock_messenger.send("Second message");

    // We can check our work. `borrow()` gets an immutable reference to the inner data.
    let messages = mock_messenger.sent_messages.borrow();
    println!("Mock messenger has {} messages sent.", messages.len());
    assert_eq!(messages.len(), 2);
    println!("Messages sent: {:?}", messages);

    println!("\n--- End of Lesson 16 ---");
    println!("Summary: Use `Box` for simple heap data, `Rc` for multiple owners, and `RefCell` when you need to mutate data that appears immutable.");
}
