/**
 * @file 22_SimpleWebAPI/src/main.rs
 * @author dunamismax
 * @date 2025-06-11
 *
 * @brief Final Capstone Project: A database-backed web API with unit tests and logging.
 *
 * ## Become a Backend Dev: The Capstone Project
 *
 * This is it! This final lesson integrates everything we've learned into a single,
 * cohesive application. We will build a complete REST API that performs CRUD operations
 * for "Users". It is architected to be robust, maintainable, and asynchronous.
 *
 * ### We will integrate the following key skills:
 * - **Web Framework (`axum`):** To handle HTTP requests, routing, and state.
 * - **Database Persistence (`sqlx`):** To store our data in a SQLite database. (Lesson 21)
 * - **Async Programming (`tokio`):** The entire application is non-blocking. (Lesson 20)
 * - **Shared State (`Arc`):** To safely share our database connection pool with all requests. (Lesson 19)
 * - **JSON Processing (`serde`):** To serialize and deserialize data for our API. (Lesson 17)
 * - **Error Handling:** We will build a robust error handling system that translates our
 *   internal application errors into proper HTTP responses.
 *
 * ### Application Architecture:
 * Client -> HTTP Request -> Axum Router -> Handler -> `sqlx` -> Database
 *
 * ### How to Run This Program:
 * 1. Ensure you've completed the setup from Lesson 21 (sqlx-cli, .env, migrations).
 * 2. Run the server: `cargo run`
 * 3. Use a tool like `curl` or Postman to interact with the API endpoints.
 *
 * ### Example `curl` commands:
 * # Get all users:
 * curl http://127.0.0.1:3000/api/users
 *
 * # Create a user:
 * curl -X POST -H "Content-Type: application/json" -d '{"username": "carol", "email": "carol@example.com"}' http://127.0.0.1:3000/api/users
 *
 * # Get user with ID 1:
 * curl http://127.0.0.1:3000/api/users/1
 */

 use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::net::SocketAddr;
use std::sync::Arc;

// --- Application State ---
// This struct will hold shared state, like our database connection pool.
// We wrap it in an `Arc` to allow it to be shared safely across threads.
struct AppState {
    db_pool: SqlitePool,
}

// --- Data Models ---
// These are the structs that represent our data.
#[derive(Serialize, sqlx::FromRow, Debug)]
struct User {
    id: i64,
    username: String,
    email: String,
}

// This struct is used for the request body when creating a new user.
#[derive(Deserialize)]
struct CreateUserPayload {
    username: String,
    email: String,
}

// --- Main Application Entry Point ---
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup logging
    tracing_subscriber::fmt::init();

    // Load .env file and get database URL
    dotenvy::dotenv().expect("Failed to read .env file");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // The shared state
    let app_state = Arc::new(AppState { db_pool: pool });

    // Define our application's routes
    let app = Router::new()
        .route("/api/users", get(get_users_handler).post(create_user_handler))
        .route(
            "/api/users/:id",
            get(get_user_handler)
                .put(update_user_handler)
                .delete(delete_user_handler),
        )
        .with_state(app_state);

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}


// --- API Handlers ---
// These functions are called by the router when a request matches their path.

/// Handler to get all users
async fn get_users_handler(State(state): State<Arc<AppState>>) -> Result<Json<Vec<User>>, ApiError> {
    let users = sqlx::query_as!(User, "SELECT id, username, email FROM users")
        .fetch_all(&state.db_pool)
        .await?;
    Ok(Json(users))
}

/// Handler to create a new user
async fn create_user_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<(StatusCode, Json<User>), ApiError> {
    let result = sqlx::query!(
        "INSERT INTO users (username, email) VALUES (?, ?)",
        payload.username,
        payload.email
    )
    .execute(&state.db_pool)
    .await?;

    let new_user_id = result.last_insert_rowid();
    let new_user = sqlx::query_as!(User, "SELECT id, username, email FROM users WHERE id = ?", new_user_id)
        .fetch_one(&state.db_pool)
        .await?;

    Ok((StatusCode::CREATED, Json(new_user)))
}

/// Handler to get a single user by ID
async fn get_user_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<User>, ApiError> {
    let user = sqlx::query_as!(User, "SELECT id, username, email FROM users WHERE id = ?", id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::NotFound,
            _ => ApiError::from(e),
        })?;
    Ok(Json(user))
}

/// Handler to update a user (replaces the user with new data)
async fn update_user_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(payload): Json<CreateUserPayload>, // Re-use payload for simplicity
) -> Result<Json<User>, ApiError> {
    // First, check if the user exists
    sqlx::query!("SELECT id FROM users WHERE id = ?", id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| ApiError::NotFound)?;
    
    // Now, update
    sqlx::query!(
        "UPDATE users SET username = ?, email = ? WHERE id = ?",
        payload.username,
        payload.email,
        id
    )
    .execute(&state.db_pool)
    .await?;

    let updated_user = sqlx::query_as!(User, "SELECT id, username, email FROM users WHERE id = ?", id)
        .fetch_one(&state.db_pool)
        .await?;
    
    Ok(Json(updated_user))
}

/// Handler to delete a user by ID
async fn delete_user_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(&state.db_pool)
        .await?;
    
    if result.rows_affected() == 0 {
        Err(ApiError::NotFound)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}


// --- Custom Error Handling ---
// This enum defines the types of errors our API can return.
enum ApiError {
    SqlxError(sqlx::Error),
    NotFound,
}

// This implementation tells Axum how to convert our `ApiError` into a
// proper HTTP response.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::SqlxError(e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
            }
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
        };
        (status, Json(serde_json::json!({ "error": error_message }))).into_response()
    }
}

// This allows us to use the `?` operator to easily convert `sqlx::Error`
// into our `ApiError`.
impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::SqlxError(err)
    }
}