//use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use rand::Rng;
use config::Config;
use actix_files as fs;
use serde_json::json;


#[derive(Debug, Serialize, Deserialize)]
struct Container {
    name: String,
    image: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

fn add_fake_containers(machine: &mut RemoteMachine) {
    let mut rng = rand::thread_rng();
    let fake_container_count = rng.gen_range(1..=5);
    let mut fake_containers = Vec::new();

    for _ in 0..fake_container_count {
        let fake_name = format!("container_{}", rng.gen_range(1000..9999));
        let fake_container = Container {
            name: fake_name,
            image: "fake_image".to_string(),
            status: "running".to_string(),
        };
        fake_containers.push(fake_container);
    }
    
    println!("Adding fake containers to machine {}: {:?}", machine.name, fake_containers);
    
    machine.containers = fake_containers;
    // Here you can add logic to associate these fake containers with the machine if needed
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

    let mut machines: Vec<RemoteMachine> = config.get("machines").unwrap();
    for machine in &mut machines {
        add_fake_containers(machine);
    }
    let app_state = web::Data::new(Arc::new(Mutex::new(AppState { machines })));
    HttpServer::new(move || App::new()
    .app_data(app_state.clone())
    .service(hello)
    .service(get_machines)
    .service(fs::Files::new("/", "./frontend/public").index_file("index.html")))
    .bind(("localhost", 6780))?
    .run()
    .await

    // HttpServer::new(move || {
    //     App::new()
    //         .app_data(app_state.clone())
    //         .service(fs::Files::new("/", "./frontend/public").index_file("index.html"))
    //         .service(get_machines)
    //         .service(web::resource("/api/update").to(update_containers))
    // })
    // .bind("127.0.0.1:6780")?
    // .run()
    // .await
}

#[get("/api/machines")]
pub async fn get_machines(state: web::Data<Arc<Mutex<AppState>>>) -> impl Responder {
    let app_state = state.lock().await;
    let machines = &app_state.machines;
    println!("List of machines in get_machines: {:?}", machines);
    HttpResponse::Ok().json(json!({ "machines": machines }))
}
