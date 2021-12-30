use axum::{extract::Path, response::Json};
use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize)]
struct Task {
    id: u64,
    title: String,
    completed: bool,
    description: String,
}
#[derive(Deserialize)]
pub struct CreateT {
    title: String,
    completed: bool,
    description: String,
}

static mut TASKS: Vec<Task> = vec![];

pub async fn get_tasks() -> Json<Value> {
    unsafe { Json(json!({ "data": TASKS, "message": "get all task" })) }
}

pub async fn get_task(Path(id): Path<u64>) -> Json<Value> {
    unsafe {
        let task = TASKS.iter().find(|t| t.id == id);
        match task {
            Some(task) => Json(json!({ "data": task, "message": "get task" })),
            None => Json(json!({ "data": "task not found", "message": "task not found" })),
        }
    }
}

pub async fn create_task(Json(payload): Json<CreateT>) -> impl IntoResponse {
    unsafe {
        let id = TASKS.len() as u64;
        let task = Task {
            id: id,
            title: payload.title,
            completed: payload.completed,
            description: payload.description,
        };
        TASKS.push(task);
        let task_find = TASKS.iter().find(|t| t.id == id);
        (
            StatusCode::CREATED,
            Json(json!({ "data": task_find, "message": "create task" })),
        )
    }
}


pub async fn update_task(Json(payload): Json<CreateT>, Path(id): Path<u64>) -> impl IntoResponse {
    unsafe {
        let task = TASKS.iter().find(|t| t.id == id);
        match task {
            Some(task) => {
                let refreshed_task = Task {
                    id: id,
                    title: payload.title,
                    completed: payload.completed,
                    description: payload.description,
                };
                TASKS[task.id as usize] = refreshed_task;
                let msg = json!({ "data": task, "message": "update task" });
                (StatusCode::OK, Json(msg))
            }
            None => (
                StatusCode::NOT_FOUND,
                Json(json!({ "data": "task not found", "message": "task not found" })),
            ),
        }
    }
}

pub async fn delete_task(Path(id): Path<u64>) -> impl IntoResponse {
    unsafe {
        let task = TASKS.iter().find(|t| t.id == id);
        match task {
            Some(task) => {
                TASKS.retain(|t| t.id != id);
                (
                    StatusCode::OK,
                    Json(json!({ "data": task, "message": "task delete" })),
                )
            }
            None => (
                StatusCode::NOT_FOUND,
                Json(json!({ "data": "task not found", "message": "task not found" })),
            ),
        }
    }
}
