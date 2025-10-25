use axum::{
    Router,
    extract::Path,
    http::{HeaderValue, Method},
    response::Json,
    routing::{get, post},
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::SocketAddr;
use std::sync::Mutex;
use tower_http::cors::CorsLayer;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: T,
}

/* --------------------------------------------------------------
In‑memory “database”.  It lives for the whole program, but
every access is guarded by a Mutex, so no `unsafe` is needed.
-------------------------------------------------------------- */
lazy_static! {
    static ref USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());
}

#[tokio::main]
async fn main() {
    // Initialize with some sample data
    {
        let mut users = USERS.lock().unwrap();
        *users = vec![
            User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
            },
        ];
    }

    // CORS configuration to allow WASM client
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(root))
        .route("/api/users", get(get_users))
        .route("/api/users", post(create_user))
        .route("/api/users/:id", get(get_user_by_id))
        .route("/api/health", get(health_check))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Backend server running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

// Health check endpoint
async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: "Server is healthy!".to_string(),
    })
}

// Root endpoint
async fn root() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: "Rust Backend API is running!".to_string(),
    })
}

// Get all users
async fn get_users() -> Json<ApiResponse<Vec<User>>> {
    // Clone the vector while the lock is held, then release the lock
    let users = USERS.lock().unwrap().clone();
    Json(ApiResponse {
        success: true,
        data: users,
    })
}

// Get user by ID
async fn get_user_by_id(Path(id): Path<u32>) -> Json<ApiResponse<Option<User>>> {
    let users = USERS.lock().unwrap();
    let user = users.iter().find(|it| it.id == id).cloned();
    Json(ApiResponse {
        success: user.is_some(),
        data: user,
    })
}

// Create new user
async fn create_user(Json(payload): Json<CreateUserRequest>) -> Json<ApiResponse<User>> {
    let mut users = USERS.lock().unwrap();
    let new_id = users.len() as u32 + 1;
    let new_user = User {
        id: new_id,
        name: payload.name,
        email: payload.email,
    };
    users.push(new_user.clone());
    Json(ApiResponse {
        success: true,
        data: new_user,
    })
}
