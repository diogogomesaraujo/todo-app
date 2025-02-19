use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{
    delete, get, post, put, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct Task {
    id: u32,
    content: String,
    checked: bool,
}

impl Responder for Task {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}

struct AppState {
    id_count: Mutex<u32>,
    tasks: Mutex<Vec<Task>>,
}

// get all tasks
#[get("/tasks")]
async fn get_tasks(data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();

    let response = serde_json::to_string(&(*tasks)).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}

// add a task
#[post("/tasks")]
async fn add_task(req: web::Json<String>, data: web::Data<AppState>) -> impl Responder {
    let mut id_count = data.id_count.lock().unwrap();
    *id_count += 1;

    let new_task = Task {
        id: id_count.clone(),
        content: String::from(&req.into_inner()),
        checked: false,
    };

    let mut tasks = data.tasks.lock().unwrap();

    let response = serde_json::to_string(&new_task).unwrap();

    tasks.push(new_task);

    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(response)
}

//remove a task
#[delete("/tasks/{id}")]
async fn delete_task(id: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let task_id: u32 = *id;
    let mut tasks = data.tasks.lock().unwrap();

    let i: Option<usize> = tasks.iter().position(|t| task_id == t.id);

    match i {
        Some(i) => {
            tasks.remove(i);
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(format!("Deleted with success!"))
        }
        None => HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body(format!("Couldn't find the task with id {}", task_id)),
    }
}

//edit content of task
#[put("/tasks/{i}")]
async fn edit_task_content(
    id: web::Path<u32>,
    req: web::Json<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let task_id: u32 = *id;
    let mut tasks = data.tasks.lock().unwrap();

    let i: Option<usize> = tasks.iter().position(|t| task_id == t.id);

    match i {
        Some(i) => {
            tasks[i].content = req.into_inner();
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(format!("Edited with success!"))
        }
        None => HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body(format!("Couldn't find the task with id {}", task_id)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        id_count: Mutex::new(0),
        tasks: Mutex::new(vec![Task {
            id: 0,
            content: "Add your first task!".to_string(),
            checked: false,
        }]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_tasks)
            .service(add_task)
            .service(delete_task)
            .service(edit_task_content)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
