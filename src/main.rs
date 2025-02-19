use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{
    delete, get, post, put, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
    ResponseError,
};

use serde::{Deserialize, Serialize};

use std::fmt::Display;
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
    tasks: Mutex<Vec<Task>>,
}

#[get("/tasks")]
async fn get_tasks(data: web::Data<AppState>) -> impl Responder {
    let tasks = data.tasks.lock().unwrap();

    let response = serde_json::to_string(&(*tasks)).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        tasks: Mutex::new(vec![Task {
            id: 1,
            content: "Add your first task!".to_string(),
            checked: false,
        }]),
    });

    HttpServer::new(move || App::new().app_data(app_state.clone()).service(get_tasks))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
