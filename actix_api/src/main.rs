use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct CreateTodoInput {
    title: String,
}

#[derive(Deserialize)]
struct UpdateTodoInput {
    title: Option<String>,
    completed: Option<bool>,
}

use std::sync::{Arc, Mutex};

struct AppState {
    todo_list: Arc<Mutex<Vec<Todo>>>,
}

use actix_web::{HttpResponse, Responder, delete, get, post, put, web};

#[post("/todos")]
async fn create_todo(
    data: web::Data<AppState>,
    body: web::Json<CreateTodoInput>,
) -> impl Responder {
    let mut todos = data.todo_list.lock().unwrap();
    let new_id = todos.len() + 1;

    let new_todo = Todo {
        id: new_id,
        title: body.title.clone(),
        completed: false,
    };

    todos.push(new_todo.clone());
    HttpResponse::Created().json(new_todo)
}

#[get("/todos")]
async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let todos = data.todo_list.lock().unwrap();
    HttpResponse::Ok().json(&*todos)
}

#[put("/todos/{id}")]
async fn update_todo(
    data: web::Data<AppState>,
    path: web::Path<usize>,
    body: web::Json<UpdateTodoInput>,
) -> impl Responder {
    let id = path.into_inner();
    let mut todos = data.todo_list.lock().unwrap();

    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
        if let Some(title) = &body.title {
            todo.title = title.clone();
        }
        if let Some(completed) = body.completed {
            todo.completed = completed;
        }
        HttpResponse::Ok().json(todo)
    } else {
        HttpResponse::NotFound().body("Todo not found")
    }
}

#[delete("/todos/{id}")]
async fn delete_todo(data: web::Data<AppState>, path: web::Path<usize>) -> impl Responder {
    let id = path.into_inner();
    let mut todos = data.todo_list.lock().unwrap();

    let initial_len = todos.len();
    todos.retain(|t| t.id != id);

    if todos.len() < initial_len {
        HttpResponse::Ok().body("Todo deleted successfully")
    } else {
        HttpResponse::NotFound().body("Todo not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize our shared database simulation
    let app_state = web::Data::new(AppState {
        todo_list: Arc::new(Mutex::new(Vec::new())),
    });

    println!("🚀 Server starting on http://127.0.0.1:3000");

    use actix_web::{App, HttpServer};
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // Share state across threads
            .service(create_todo)
            .service(get_todos)
            .service(update_todo)
            .service(delete_todo)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
