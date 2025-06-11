/**
 * @file 21_DatabaseWithSqlx/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Project 21: A complete data layer with compile-time checked SQL.
 *
 * ## Connect to a Database with `sqlx`
 *
 * This is our first major capstone project. We will build a complete data access layer
 * that performs Create, Read, Update, and Delete (CRUD) operations against a database.
 * We will use `sqlx`, a modern, asynchronous, and safe SQL toolkit for Rust.
 *
 * `sqlx`'s killer feature is **compile-time checked queries**. It inspects your SQL
 * queries at compile time and compares them against your live database (or a saved
 * schema), catching errors long before your code ever runs.
 *
 * ### Key Concepts in this Lesson:
 * - **`sqlx-cli`:** The command-line tool for managing migrations and preparing the
 *   offline schema data for compile-time checks.
 * - **Database Migrations:** Version-controlling your database schema.
 * - **Connection Pool (`SqlitePool`):** An efficient way to manage multiple connections
 *   to the database, essential for concurrent applications like web servers.
 * - **CRUD Operations:** Implementing create, read, update, and delete logic.
 * - **`query_as!` macro:** The `sqlx` macro to execute a query and map the results
 *   directly into a Rust struct.
 * - **`#[derive(sqlx::FromRow)]`:** The derive macro that enables this mapping.
 *
 * ### How to Run This Program:
 * 1. Follow the setup steps (install sqlx-cli, create .env, create migration).
 * 2. **Prepare the query data:** `sqlx prepare`
 * 3. Run the application: `cargo run`
 */

 use sqlx::sqlite::{SqlitePool, SqliteRow};
 use sqlx::FromRow;
 use serde::{Deserialize, Serialize};
 use anyhow::Result;
 
 // Our User struct.
 // `#[derive(FromRow)]` allows `sqlx` to map a database row to this struct.
 // `Debug` lets us print it, `Serialize` will be useful for the web API.
 #[derive(Debug, FromRow, Serialize, Deserialize)]
 pub struct User {
     pub id: i64,
     pub username: String,
     pub email: String,
 }
 
 #[tokio::main]
 async fn main() -> Result<()> {
     println!("--- Lesson 21: Database with sqlx ---\n");
 
     // 1. Load environment variables from .env file
     dotenvy::dotenv().expect("Failed to read .env file");
     let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
 
     // 2. Create a connection pool
     let pool = SqlitePool::connect(&database_url).await?;
     println!("Successfully connected to the database.");
 
     // 3. Run migrations
     sqlx::migrate!("./migrations").run(&pool).await?;
     println!("Database migrations ran successfully.");
 
     // 4. Run our CRUD demo
     if let Err(e) = run_crud_demo(&pool).await {
         eprintln!("CRUD demo failed: {}", e);
     }
     
     Ok(())
 }
 
 async fn run_crud_demo(pool: &SqlitePool) -> Result<()> {
     println!("\n--- Running CRUD Demo ---");
 
     // CREATE
     let new_user_id = create_user(pool, "alice", "alice@example.com").await?;
     println!("Created user with ID: {}", new_user_id);
 
     let _ = create_user(pool, "bob", "bob@example.com").await?;
 
     // READ (All)
     let users = get_all_users(pool).await?;
     println!("\nCurrent users: {:#?}", users);
 
     // READ (One)
     let user = get_user_by_id(pool, new_user_id).await?;
     println!("\nFetched user by ID {}: {:#?}", new_user_id, user);
 
     // UPDATE
     let updated_rows = update_user_email(pool, new_user_id, "alice.new@example.com").await?;
     println!("\nUpdated {} user(s). New email for user ID {}.", updated_rows, new_user_id);
     let updated_user = get_user_by_id(pool, new_user_id).await?;
     println!("Verified updated user: {:#?}", updated_user);
 
     // DELETE
     let deleted_rows = delete_user(pool, new_user_id).await?;
     println!("\nDeleted {} user(s) with ID {}.", deleted_rows, new_user_id);
     
     let final_users = get_all_users(pool).await?;
     println!("\nFinal list of users: {:#?}", final_users);
     
     println!("\n--- CRUD Demo Finished ---");
     Ok(())
 }
 
 /// CREATE: Inserts a new user into the database.
 async fn create_user(pool: &SqlitePool, username: &str, email: &str) -> Result<i64> {
     let result = sqlx::query!("INSERT INTO users (username, email) VALUES (?, ?)", username, email)
         .execute(pool)
         .await?;
     Ok(result.last_insert_rowid())
 }
 
 /// READ: Fetches all users from the database.
 async fn get_all_users(pool: &SqlitePool) -> Result<Vec<User>> {
     let users = sqlx::query_as!(User, "SELECT id, username, email FROM users")
         .fetch_all(pool)
         .await?;
     Ok(users)
 }
 
 /// READ: Fetches a single user by their ID.
 async fn get_user_by_id(pool: &SqlitePool, id: i64) -> Result<User> {
     let user = sqlx::query_as!(User, "SELECT id, username, email FROM users WHERE id = ?", id)
         .fetch_one(pool)
         .await?;
     Ok(user)
 }
 
 /// UPDATE: Updates a user's email given their ID.
 async fn update_user_email(pool: &SqlitePool, id: i64, new_email: &str) -> Result<u64> {
     let result = sqlx::query!("UPDATE users SET email = ? WHERE id = ?", new_email, id)
         .execute(pool)
         .await?;
     Ok(result.rows_affected())
 }
 
 /// DELETE: Removes a user from the database by their ID.
 async fn delete_user(pool: &SqlitePool, id: i64) -> Result<u64> {
     let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
         .execute(pool)
         .await?;
     Ok(result.rows_affected())
 }