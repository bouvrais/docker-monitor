use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use config::Config;
use actix_files as fs;
use serde_json::json;
use reqwest::Client;
use chrono::{DateTime, Utc};
use bollard::models::Port;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Container {
    id: String,
    name: String,
    image: String,
    status: String,
    created: DateTime<Utc>,
    ports: Vec<Port>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RemoteMachine {
    name: String,
    address: String,
    #[serde(default)]
    containers: Vec<Container>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AppState {
    machines: Vec<RemoteMachine>,
}

#[get("/api/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    let config = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap();

    let machines: Vec<RemoteMachine> = config.get("machines").unwrap();

    let app_state = web::Data::new(Arc::new(Mutex::new(AppState { machines })));
    HttpServer::new(move || {
        App::new()
        .app_data(app_state.clone())
        .service(hello)
        .service(get_machines)
        .service(get_machine_containers)
        .service(get_machine_details_page)
        .service(fs::Files::new("/", "../web/dist").index_file("index.html"))

    })
    .bind(("localhost", 6780))?
    .run()
    .await
}

#[get("/api/machines")]
pub async fn get_machines(state: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let app_state = state.lock().await;
    let machines = &app_state.machines;
    let client = Client::new();
    let mut updated_machines = Vec::new();

    for machine in machines {
        let url = format!("http://{}/api/containers", machine.address);
        match client.get(&url).send().await {
            Ok(response) => {
                if let Ok(containers) = response.json::<Vec<Container>>().await {
                    let updated_machine = RemoteMachine {
                        name: machine.name.clone(),
                        address: machine.address.clone(),
                        containers,
                    };
                    updated_machines.push(updated_machine);
                } else {
                    println!("Failed to parse containers for machine: {}", machine.name);
                    updated_machines.push(machine.clone());
                }
            }
            Err(e) => {
                println!("Failed to fetch containers for machine {}: {}", machine.name, e);
                updated_machines.push(machine.clone());
            }
        }
    }

    HttpResponse::Ok().json(json!({ "machines": updated_machines }))
}

#[get("/api/machines/{machine_id}/containers")]
async fn get_machine_containers(path: web::Path<String>, state: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let machine_id = path.into_inner();
    let app_state = state.lock().await;
    let machines = &app_state.machines;
    let machine = machines.iter().find(|m| m.name == machine_id);
    if let Some(machine) = machine {
        let client = Client::new();
        let url = format!("http://{}/api/containers", machine.address);
        match client.get(&url).send().await {
            Ok(response) => {
                if let Ok(containers) = response.json::<Vec<Container>>().await {
                    HttpResponse::Ok().json(json!({ "containers": containers }))
                } else {
                    println!("Failed to parse containers for machine: {}", machine.name);
                    HttpResponse::InternalServerError().json(json!({ "error": "Failed to parse containers" }))
                }   
            }
            Err(e) => {
                println!("Failed to fetch containers for machine {}: {}", machine.name, e);
                HttpResponse::InternalServerError().json(json!({ "error": format!("Failed to fetch containers: {}", e) }))
            }
        }
    } else {
        HttpResponse::NotFound().json(json!({ "error": format!("Machine {} not found", machine_id) }))
    }
}

#[get("/machines/{machine_id}")]
async fn get_machine_details_page() -> impl Responder {
    //let machine_id = path.into_inner();
    // Return the content of "../web/dist/machines/index.html"
    let file_path = format!("../web/dist/machines/index.html");
    fs::NamedFile::open(file_path)
}

