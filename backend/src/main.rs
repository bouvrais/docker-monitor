//use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use config::Config;
use actix_files as fs;
use serde_json::json;
use reqwest::Client;
use actix_cors::Cors;
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Container {
    name: String,
    image: String,
    status: String,
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
        //let cors = Cors::default().allowed_origin("http://localhost:5174");
        let cors = Cors::default().allow_any_origin().send_wildcard();
        App::new()
        .wrap(cors)
        .app_data(app_state.clone())
        .service(hello)
        .service(get_machines)
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
