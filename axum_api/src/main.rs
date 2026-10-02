use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Clone)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

#[derive(Deserialize)]
struct CreateTask {
    title: String,
}

#[derive(Deserialize)]
struct UpdateTask {
    title: Option<String>,
    done: Option<bool>,
}

#[derive(Clone)]
struct AppState {
    tasks: Arc<Mutex<Vec<Task>>>,
    next_id: Arc<Mutex<u32>>,
}

impl AppState {
    pub fn new() -> Self {
        let seed = vec![
            Task {
                id: 1,
                title: "Learn Axum".to_string(),
                done: false,
            },
            Task {
                id: 2,
                title: "Learn Tokio".to_string(),
                done: false,
            },
            Task {
                id: 3,
                title: "Learn Serde".to_string(),
                done: false,
            },
        ];

        Self {
            tasks: Arc::new(Mutex::new(seed)),
            next_id: Arc::new(Mutex::new(4)),
        }
    }
}

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/tasks", get(list_tasks).post(create_task))
        .route(
            "/tasks/{id}",
            get(get_task).delete(delete_task).patch(update_task),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "app is live"
}

async fn health() -> &'static str {
    "Ok"
}

async fn list_tasks(State(state): State<AppState>) -> Json<Vec<Task>> {
    let tasks = state.tasks.lock().unwrap();

    Json(tasks.clone())
}

async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Task>, StatusCode> {
    let tasks = state.tasks.lock().unwrap();

    tasks
        .iter()
        .find(|t| t.id == id)
        .map(|t| Json(t.clone()))
        .ok_or(StatusCode::NOT_FOUND)
}

async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTask>,
) -> (StatusCode, Json<Task>) {
    let mut tasks = state.tasks.lock().unwrap();
    let mut next_id = state.next_id.lock().unwrap();

    let task = Task {
        id: *next_id,
        title: payload.title,
        done: false,
    };

    *next_id += 1;
    tasks.push(task.clone());

    (StatusCode::CREATED, Json(task))
}

async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, StatusCode> {
    let mut tasks = state.tasks.lock().unwrap();

    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(title) = payload.title {
        task.title = title;
    }

    if let Some(done) = payload.done {
        task.done = done;
    }

    Ok(Json(task.clone()))
}

async fn delete_task(State(state): State<AppState>, Path(id): Path<u32>) -> StatusCode {
    let mut tasks = state.tasks.lock().unwrap();
    let before = tasks.len();

    tasks.retain(|t| t.id != id);

    if tasks.len() < before {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
