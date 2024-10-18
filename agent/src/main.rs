use actix_web::{web, App, HttpServer, HttpResponse, Responder, get};
use bollard::Docker;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Container {
    id: String,
    name: String,
    image: String,
    status: String,
    created: DateTime<Utc>,
    update_available: bool,
}

struct AppState {
    containers: Arc<RwLock<Vec<Container>>>,
}

async fn update_containers(state: web::Data<AppState>, docker: &Docker) {
    let mut containers = Vec::new();

    let list_containers = docker.list_containers::<String>(None).await.unwrap();
    for container in list_containers {
        let id = container.id.unwrap_or_default();
        let name = container.names.unwrap_or_default().join(", ");
        let image = container.image.unwrap_or_default();
        let status = container.state.unwrap_or_default();
        let created = DateTime::from_timestamp(container.created.unwrap_or_default(), 0).unwrap_or_default();

        // Check if an update is available for the image
        let update_available = check_image_update(&docker, &image).await;

        containers.push(Container {
            id,
            name,
            image,
            status,
            created,
            update_available,
        });
    }

    let mut state_containers = state.containers.write().await;
    *state_containers = containers;
}

async fn check_image_update(docker: &Docker, image: &str) -> bool {
    // This is a simplified check. In a real-world scenario, you'd need to handle
    // authentication, different registries, and parsing image names and tags.
    let inspect = docker.inspect_image(image).await;
    match inspect {
        Ok(img) => {
            if let Some(repo_tags) = img.repo_tags {
                if let Some(_tag) = repo_tags.first() {
                    // Here you would typically make an API call to the Docker registry
                    // to check if a newer version is available.
                    // For this example, we'll just return false.
                    false
                } else {
                    false
                }
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

#[get("/api/containers")]
async fn get_containers(state: web::Data<AppState>) -> impl Responder {
    let containers = state.containers.read().await;
    HttpResponse::Ok().json(&*containers)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let docker = Docker::connect_with_local_defaults().expect("Failed to connect to Docker");
    let state = web::Data::new(AppState {
        containers: Arc::new(RwLock::new(Vec::new())),
    });

    // Spawn a task to update containers periodically
    let state_clone = state.clone();
    let docker_clone = docker.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(60)); // Update every 60 seconds
        loop {
            interval.tick().await;
            update_containers(state_clone.clone(), &docker_clone).await;
        }
    });

    // Initial update
    update_containers(state.clone(), &docker).await;

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get_containers)
    })
    .bind("0.0.0.0:7001")?
    .run()
    .await
}
