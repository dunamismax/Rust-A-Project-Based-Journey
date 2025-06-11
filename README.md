# Rust: A Project-Based Journey

![alt text](https://img.shields.io/badge/Language-Rust-orange.svg)

![alt text](https://img.shields.io/badge/License-MIT-yellow.svg)

![alt text](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)

<!-- FIXME: Replace YOUR_USERNAME and YOUR_REPO_NAME with your actual GitHub details -->
![alt text](https://img.shields.io/github/stars/YOUR_USERNAME/YOUR_REPO_NAME?style=social)

Welcome to your ultimate journey to mastering Rust! This open-source curriculum is designed to guide you from your very first line of code to building a complete, database-connected web API, all while embracing the principles that make Rust unique.

Learning Rust is like learning to forge a masterwork sword. It demands precision and a deep understanding of the material, but the result is unmatched in its reliability, performance, and safety. This course is your apprenticeship.
✨ Why This Journey?
This isn't just a collection of code snippets. It's a structured path designed for a deep, practical understanding of modern, idiomatic Rust.

🧠 Befriend the Compiler, Master Ownership: We tackle Ownership, Borrowing, and the Borrow Checker head-on. You'll learn not just the rules, but why they exist, turning the compiler from an adversary into your most trusted pair programmer.
🚀 Zero to Web Developer: 23 carefully ordered lessons guide you from core syntax and Cargo to building a high-performance, asynchronous REST API with a database backend in the final capstone project.
🛠️ Build a Real-World Portfolio: You won't just learn concepts; you'll apply them immediately by building practical tools, a JSON data processor, and a complete web service.
💪 Master the Rust Ecosystem: We embrace the tools of modern Rust development. You will master Cargo, use popular crates like serde, tokio, and axum, and understand how to navigate crates.io.
🌍 Open Source & Community Driven: This curriculum is for the community, by the community. Contributions, suggestions, and corrections are always welcome.
💡 Project Philosophy
Learn by Doing: We believe the best way to learn is by building. Every concept is tied to a practical, runnable example.
Code as Documentation: Every line of code, especially in the early lessons, is heavily documented to explain not just what it does, but why it's done that way in idiomatic Rust.
Incremental Complexity: The curriculum is carefully designed to introduce one major concept at a time, building on previous lessons to avoid overwhelming the learner.
🚀 Getting Started
All you need to begin your journey is the Rust toolchain and a desire to build fast, reliable software.

Prerequisites
The Rust Toolchain, installed via rustup. This includes rustc (the compiler) and cargo (the package manager and build tool).
A text editor or IDE (like VS Code with the rust-analyzer extension, or a JetBrains IDE with the Rust plugin).
A command-line terminal.
How to Use This Repository
Clone the repository:
git clone https://github.com/YOUR_USERNAME/YOUR_REPO_NAME.git
cd YOUR_REPO_NAME
content_copy
download
Use code with caution.
Sh
Start with Lesson 1: Each lesson is a self-contained Cargo project. cd into the first lesson's directory.
cd 1_HelloWorld
content_copy
download
Use code with caution.
Sh
Open src/main.rs and read through the heavily documented code.
Compile, Run, or Test: Use Cargo to interact with each lesson's project.
# Check the code for errors without compiling
cargo check

# Compile and run the project (for most lessons)
cargo run

# Run the tests (for Lesson 13 and others with test modules)
cargo test
content_copy
download
Use code with caution.
Sh
Proceed to the next lesson and enjoy the journey!
⚠️ Special Setup for Advanced Lessons
Starting in Part 3, some lessons require additional setup beyond a simple cargo run. Detailed instructions are provided inside each respective lesson's directory. This is to introduce you to real-world project management.

Lesson 13 (Testing): This lesson is focused on writing tests. You will primarily use the cargo test command to see the results.
Lessons 21 (Database) & 22 (Web API): These final projects require you to set up a local database environment. This involves:
Installing sqlx-cli (cargo install sqlx-cli).
Creating a .env file for your database URL.
Creating and running database migrations with sqlx migrate.
Don't worry, a step-by-step guide is provided to walk you through this process when you get there!

📚 The Journey
The curriculum is divided into four distinct paths, each building on the last to take you from core concepts to professional-level skills. Key lessons are marked as Core Concepts or Projects to highlight their importance.

Part 1: The Beginner Path - Core Syntax & The Ownership Model
Directory	Key Concepts	Description
1_HelloWorld	cargo, fn main(), println!, modules	Your first Rust program: using Cargo to create and run a project.
2_VariablesAndPrimitives	let, mut, shadowing, scalar types, tuples	Learn to store, manage, and display information with Rust's primitive types.
3_FunctionsAndControlFlow	fn, parameters, expressions vs. statements, if-else, loops	Give your program logic to perform tasks and make decisions.
4_Ownership	Core Concept: stack vs. heap, moving, cloning, String	The Ownership Leap: Understand Rust's unique memory management model.
5_BorrowingAndSlices	Core Concept: &, &mut, references, string slices	The solution to Ownership: Learn how to access data without taking ownership.
6_Structs	struct, methods, associated functions	Create your first custom data types to model your application's domain.
Part 2: The Intermediate Path - Building Blocks of Idiomatic Rust
Directory	Key Concepts	Description
7_EnumsAndPatternMatching	enum, Option<T>, Result<T, E>, match	Master one of Rust's most powerful features for robust data modeling and control flow.
8_Collections	Vec<T>, HashMap<K, V>, ownership in collections	Move beyond basic types to manage dynamic lists and key-value pairs.
9_ErrorHandling	Core Concept: panic!, Result, the ? operator	Write resilient code that can gracefully handle recoverable errors.
10_Traits	trait, generics (<T>), derive macros, impl Trait	Define shared behavior and create powerful, generic functions.
11_Lifetimes	Core Concept: 'a, lifetime elision, struct lifetimes	Ensure references are always valid in more complex scenarios.
12_ModulesAndCrates	mod, use, workspaces, crates.io	Learn to organize a large project and use third-party libraries.
Part 3: The Advanced Path - The Modern Rust Ecosystem
Directory	Key Concepts	Description
13_Testing	#[test], assert!, assert_eq!, integration tests	Learn the professional discipline of writing tests to ensure your code is correct.
14_FileIO	std::fs, Read, Write, Result-based I/O	Persist data by reading from and writing to files.
15_ClosuresAndIterators	|x| x*x, .iter(), .map(), .filter(), .collect()	Embrace Functional Rust: Use iterators to perform complex data analysis cleanly.
16_SmartPointers	Box<T>, Rc<T>, RefCell<T>	Understand heap allocation and shared ownership patterns.
17_WorkingWithJSON	Project: serde, serde_json, #[derive(Serialize, Deserialize)]	Parse real-world data from a JSON file into your Rust structs and back.
Part 4: The Expert Path - Concurrency, Async & The Web
Directory	Key Concepts	Description
18_BasicConcurrency	thread::spawn, move closures, JoinHandle	Learn the fundamentals of making your program do multiple things at once, safely.
19_SharedStateConcurrency	Mutex, Arc (Atomically Reference Counted)	Prevent race conditions and share data between threads the "fearless" Rust way.
20_AsyncProgramming	async/.await, Future, tokio runtime	Unlock Performance: Write non-blocking code for high-throughput network services.
21_DatabaseWithSqlx	Project: sqlx, database migrations, compile-time checked queries	Connect to a Database: Build a data layer that performs CRUD operations.
22_SimpleWebAPI	Final Capstone Project: axum, handlers, routing, state sharing	Become a Backend Dev: Build a simple REST API that serves data over HTTP.
23_ExploringTheFuture	A guide to next steps (WebAssembly, embedded, etc.)	A final document pointing you towards future learning.
⭐ Show Your Support
If this journey helps you on your path to mastering Rust, please give this repository a star! It helps the project reach more learners and encourages us to keep creating great content.

🤝 Contributing
Contributions are the lifeblood of the open-source community. Any contributions you make are greatly appreciated.

Reporting Bugs: Find a bug in the code or a typo in the comments? Please open an issue.
Suggesting Enhancements: Have an idea for a new lesson or a better way to explain a concept? Feel free to open an issue to discuss it.
Pull Requests: If you want to contribute directly, please fork the repo and create a pull request.
📜 License
This project is licensed under the MIT License. See the LICENSE file for more details.
