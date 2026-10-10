mod models;
mod state;
mod routes;
mod handlers;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

// use axum::{
//     extract::{Path, Query},
//     http::StatusCode,
//     routing::get, 
//     Router,
//     Json
// };




// #[derive(Serialize, Clone)]
// struct Task
// {
//     id: u32,
//     title: String,
//     done: bool
// }


// #[derive(Deserialize)]
// struct  CreateTask 
// {
//     title:String
// }


#[tokio::main]
async fn main() {
    
    // let app = Router::new()
    // .route("/", get(root))
    // .route("/health", get(health))
    // .route("/tasks", get(list_tasks).post(create_tasks))
    // .route("/tasks/{id}", get(get_task));

    let state = AppState::new();

    let app = routes::app(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await.unwrap();

    println!("task is running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

// this return plain text 
// async fn root() -> & 'static str
// {
//     "task is alive"
// }


// async fn list_tasks(Query(params): Query<HashMap<String, String>>) -> String
// { 
//     // "List of task here".to_string()
//     let filter = params
//     .get("done")
//     .map_or("".to_string(), |v| format!("done={}", v));

//     format!("list tasks, filter: {}", filter)
// }



// async fn list_tasks() -> Json<Vec<Task>>
// { 
//     Json(seed_tasks())
// }



// async fn get_task(Path(id):Path<u32>) -> Result<Json<Task>, StatusCode>
// {
//     // "pick a task".to_string()
//     let task = seed_tasks();

//     task
//     .into_iter()
//     .find(|t| t.id == id)
//     .map(Json)
//     .ok_or(StatusCode::NOT_FOUND)
// }

// async fn create_tasks() -> & 'static str
// {
//     "Creating task here"
// }

// async fn create_tasks(Json(payload): Json<CreateTask>) -> (StatusCode, Json<Task>)
// {
//     let task = Task
//     {
//         id:99,
//         title: payload.title,
//         done:false
//     };

//     (StatusCode::CREATED, Json(task))
// }


// async fn health() -> & 'static str
// {
//     "Ok"
// }



// fn seed_tasks() -> Vec<Task>
// {
//     vec![

//         Task { id: 1, title: "Learn Axum".to_string(), done:false},
//         Task { id: 2, title: "Write a script".to_string(), done:false},
//         Task { id: 3, title: "Record a video".to_string(), done:false}
//     ]

// }