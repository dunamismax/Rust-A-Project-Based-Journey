# Setup Guide for Lesson 21: Database with sqlx

Welcome to the setup guide for your first major project lesson! Before you can run the Rust code for 21_DatabaseWithSqlx, you need to prepare your local environment. This process involves installing a command-line tool, creating a database, and defining its structure through migrations.

Follow these steps carefully. They are essential for sqlx to perform its compile-time query checking, which is one of its most powerful features.

Prerequisites
You have a working Rust environment with cargo installed.
You are comfortable using your command-line terminal.
The Steps
Step 1: Install sqlx-cli
sqlx comes with a dedicated command-line tool, sqlx-cli, which you will use to manage your database migrations and to prepare your project for compile-time validation.

In your terminal, run the following command to install it globally via cargo:

cargo install sqlx-cli
content_copy
download
Use code with caution.
Sh
Note: If the sqlx command isn't found after installation, you may need to ensure that ~/.cargo/bin is in your system's PATH environment variable.
Step 2: Create the Environment File
It is a best practice to keep configuration, like database connection strings, out of your source code. We will use a .env file to store our database URL.

Inside the 21_DatabaseWithSqlx project directory, create a new file named .env. Add the following content to it:

# This line tells sqlx where to find our database file.
# We are using SQLite, so the database will be a simple file
# named `database.db` created in the project root.
DATABASE_URL=sqlite:database.db
content_copy
download
Use code with caution.
Env
Step 3: Create the Database Migration
Migrations are version-controlled scripts that define and alter your database's structure over time. This is how you create and modify tables.

Make sure your terminal's current directory is 21_DatabaseWithSqlx.
Use sqlx-cli to create your first migration file:
sqlx migrate add create_users_table
content_copy
download
Use code with caution.
Sh
This command creates a new migrations directory containing a single SQL file with a name like 20250611203000_create_users_table.sql (the timestamp will be different for you).
Open this newly created SQL file and paste the following code into it to define the schema for our users table:
-- migrations/YYYYMMDDHHMMSS_create_users_table.sql

-- This SQL command creates the `users` table if it doesn't already exist.
-- It defines the columns, their data types, and other constraints.
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
content_copy
download
Use code with caution.
SQL
Step 4: Prepare for Compile-Time Checks
This is the magic step for sqlx. The prepare command does two things:

It connects to your database using the DATABASE_URL.
It runs any pending migrations (which will create the database.db file and the users table inside it).
It saves a description of your database schema into a file named sqlx-data.json.
The Rust compiler uses this sqlx-data.json file to validate your SQL queries as it compiles your code.

In your terminal, run:

sqlx prepare
content_copy
download
Use code with caution.
Sh
You should see output indicating that the migration has run and that the query data has been saved.

Step 5: Build and Run the Project
You are now ready to compile and run the Rust program!

When you run cargo run, Cargo will:

Download the dependencies listed in Cargo.toml (like sqlx, tokio, etc.).
Compile your main.rs file, checking your SQL queries against sqlx-data.json.
Execute the final program.
cargo run
content_copy
download
Use code with caution.
Sh
Setup Complete!
If you followed all the steps, your program should run successfully, connecting to the database, running the CRUD demo, and printing the results to your console.

Troubleshooting Quick-Check:

Error: DATABASE_URL must be set? Make sure you created the .env file in the 21_DatabaseWithSqlx directory and that it is named correctly.
Error: no such table: users? Make sure your migration file contains the correct CREATE TABLE SQL and that you successfully ran sqlx prepare.
Compile Error on a query? You likely need to re-run sqlx prepare after changing an SQL query in your .rs file or after changing a migration file.
