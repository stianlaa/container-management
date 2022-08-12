extern crate dockworker;

use crate::container::{Container, ContainerId, CreateContainerArgs};
use crate::web_result::WebResult;
use docker_compose_types::Compose;
use dockworker::image::SummaryImage;
use rocket::serde::json::Json;
use rocket_cors::AllowedHeaders;
use std::collections::BTreeMap;

mod compose;
mod container;
mod image;
mod web_result;

//container
#[rocket::get("/list", format = "application/json")]
fn get_container_list() -> WebResult<BTreeMap<String, Container>> {
    container::list()
}

#[rocket::put("/start", format = "application/json", data = "<data>")]
fn start_containers(data: Json<StartArgs>) -> WebResult<()> {
    container::start(data.0)
}

#[rocket::put("/stop", format = "application/json", data = "<data>")]
fn stop_containers(data: Json<StopArgs>) -> WebResult<()> {
    container::stop(data.0)
}

//image
#[rocket::get("/list", format = "application/json")]
fn get_image_list() -> WebResult<Vec<SummaryImage>> {
    image::list()
}

//compose
#[rocket::get("/docker-compose", format = "application/json")]
fn get_docker_compose() -> WebResult<Compose> {
    compose::get_docker_compose()
}

#[rocket::main]
async fn main() {
    // Configure cors to allow cors requests regardless of origins, methods and headers
    let cors = rocket_cors::CorsOptions {
        allowed_origins: rocket_cors::AllowedOrigins::All,
        allowed_methods: vec![rocket::http::Method::Get, rocket::http::Method::Put]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Unable to create cors options");

    let _ = rocket::build()
        .mount(
            "/container/",
            rocket::routes![get_container_list, start_containers, stop_containers],
        )
        .mount("/image/", rocket::routes![get_image_list])
        .mount("/compose/", rocket::routes![get_docker_compose])
        .attach(cors)
        .launch()
        .await;
}
