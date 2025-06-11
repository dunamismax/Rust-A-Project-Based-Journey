# Rust: A Project-Based Journey

[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/dunamismax/Rust-A-Project-Based-Journey/blob/main/LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](https://github.com/dunamismax/Rust-A-Project-Based-Journey/issues)
[![Stars](https://img.shields.io/github/stars/dunamismax/Rust-A-Project-Based-Journey?style=social)](https://github.com/dunamismax/Rust-A-Project-Based-Journey/stargazers)

Welcome to your ultimate journey to mastering Rust! This open-source curriculum is designed to guide you from your very first line of code to building a complete, database-backed web API.

> Learning Rust is like learning to forge a masterwork sword. It demands precision and a deep understanding of the material, but the result is unmatched in its reliability, performance, and safety. This course is your apprenticeship.

---

## ✨ Why This Journey?

This isn't just a collection of code snippets. It's a structured path designed for a deep, practical understanding of modern, idiomatic Rust.

*   **Learn by Doing:** Every lesson is a self-contained, runnable, and heavily commented Cargo project.
*   **Befriend the Compiler:** Tackle Ownership and the Borrow Checker head-on and learn *why* they are your most powerful tools.
*   **Zero to Web Developer:** Follow 23 carefully ordered lessons that take you from core syntax to a complete REST API.
*   **Master the Ecosystem:** Gain hands-on experience with `cargo` and essential crates like `serde`, `tokio`, `sqlx`, and `axum`.

---

## 📚 The Curriculum

The journey is divided into four parts, each building on the last to take you from core concepts to professional-level skills.

### Part 1: The Beginner Path - Core Syntax & The Ownership Model
| Lesson | Key Concepts | Description |
| :--- | :--- | :--- |
| `1_HelloWorld` | `cargo`, `fn main()`, `println!` | Your first Rust program. |
| `2_VariablesAndPrimitives` | `let`, `mut`, scalar types, tuples | Storing and managing data. |
| `3_FunctionsAndControlFlow`| `fn`, `if-else`, loops | Giving your program logic. |
| `4_Ownership` | **Core Concept:** move, clone, `String` | Understand Rust's unique memory model. |
| `5_BorrowingAndSlices` | **Core Concept:** `&`, `&mut` | Access data without taking ownership. |
| `6_Structs` | `struct`, methods, `impl` | Create your own custom data types. |

### Part 2: The Intermediate Path - Building Blocks of Idiomatic Rust
| Lesson | Key Concepts | Description |
| :--- | :--- | :--- |
| `7_EnumsAndPatternMatching` | `enum`, `Option<T>`, `match` | Master robust data modeling. |
| `8_Collections` | `Vec<T>`, `HashMap<K, V>` | Manage dynamic lists and key-value pairs. |
| `9_ErrorHandling` | **Core Concept:** `Result`, the `?` operator | Write resilient, professional code. |
| `10_Traits` | `trait`, generics (`<T>`), `impl Trait` | Define shared behavior. |
| `11_Lifetimes` | **Core Concept:** `'a`, lifetime elision | Ensure references are always valid. |
| `12_ModulesAndCrates` | `mod`, `use`, `crates.io` | Organize large projects and use libraries. |

### Part 3: The Advanced Path - The Modern Rust Ecosystem
| Lesson | Key Concepts | Description |
| :--- | :--- | :--- |
| `13_Testing` | `#[test]`, `assert!`, `should_panic` | Learn the discipline of testing. |
| `14_FileIO` | `std::fs`, `Read`, `Write` | Persist data by reading and writing files. |
| `15_ClosuresAndIterators` | closures, `.iter()`, `.map()`, `.filter()` | Embrace functional Rust for data analysis. |
| `16_SmartPointers` | `Box<T>`, `Rc<T>`, `RefCell<T>` | Advanced single-threaded ownership. |
| `17_WorkingWithJSON` | **Project:** `serde`, `serde_json` | Parse JSON into Rust structs and back. |

### Part 4: The Expert Path - Concurrency, Async & The Web
| Lesson | Key Concepts | Description |
| :--- | :--- | :--- |
| `18_BasicConcurrency` | `thread::spawn`, `move` closures | Do multiple things at once, safely. |
| `19_SharedStateConcurrency`| `Mutex`, `Arc` | Share data between threads without fear. |
| `20_AsyncProgramming` | `async`/`.await`, `tokio` | Write high-performance network services. |
| `21_DatabaseWithSqlx` | **Project:** `sqlx`, migrations | Build a compile-time checked data layer. |
| `22_SimpleWebAPI` | **Final Capstone:** `axum` | Build a complete REST API. |
| `23_ExploringTheFuture` | Guide to next steps | Where to go from here (Wasm, embedded, etc). |

---

## 🚀 How to Use

All you need is the Rust toolchain, installed via [rustup.rs](https://rustup.rs/).

1.  **Clone the repo:**
    ```sh
    git clone https://github.com/dunamismax/Rust-A-Project-Based-Journey.git
    cd Rust-A-Project-Based-Journey
    ```

2.  **Navigate and Run:** Each lesson is a standalone Cargo project.
    ```sh
    cd 1_HelloWorld
    cargo run
    ```
3.  **Advanced Setup:** Later lessons (like 21 and 22) require extra setup steps (e.g., `cargo install sqlx-cli`). A detailed setup guide is included in those lesson directories.

---

## 💬 Connect & Contribute

This project is for the community. Contributions are welcome! Feel free to open an issue or submit a pull request.

Connect with the author, **dunamismax**, on:

*   **Twitter:** [@dunamismax](https://twitter.com/dunamismax)
*   **Bluesky:** [@dunamismax.bsky.social](https://bsky.app/profile/dunamismax.bsky.social)
*   **Reddit:** [u/dunamismax](https://www.reddit.com/user/dunamismax)
*   **Discord:** `dunamismax`

## 📜 License

This project is licensed under the **MIT License**. See the `LICENSE` file for details.