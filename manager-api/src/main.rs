extern crate dockworker;

use crate::container::{Container, ContainerId, ContainerInfo};
use crate::container_creation::{ContainerCreateOptions, CreateContainerResponse};
use crate::container_logs::{Log, LogFilter};
use crate::web_result::WebResult;
use docker_compose_types::Compose;
use dockworker::image::SummaryImage;
use rocket::serde::json::Json;
use rocket_cors::AllowedHeaders;
use std::collections::BTreeMap;

mod compose;
mod container;
mod container_creation;
mod container_logs;
mod image;
mod web_result;

//container
#[rocket::get("/list", format = "application/json")]
fn get_container_list() -> WebResult<BTreeMap<String, Container>> {
    container::list()
}

#[rocket::get("/info?<container_id>", format = "application/json")]
fn get_container_info(container_id: String) -> WebResult<Option<ContainerInfo>> {
    container::get_info(container_id)
}

#[rocket::get("/info_by_name?<container_name>", format = "application/json")]
fn get_container_info_by_name(container_name: String) -> WebResult<Option<ContainerInfo>> {
    container::get_info_by_name(container_name)
}

#[rocket::get("/logs?<container_id>&<since>&<tail>", format = "application/json")]
fn get_container_logs(
    container_id: String,
    since: Option<i64>,
    tail: Option<i64>,
) -> WebResult<Log> {
    container_logs::get_container_logs(container_id, LogFilter { since, tail })
}

#[rocket::put("/start", format = "application/json", data = "<data>")]
fn start_container(data: Json<ContainerId>) -> WebResult<()> {
    container::start(data.0)
}

#[rocket::put("/stop", format = "application/json", data = "<data>")]
fn stop_container(data: Json<ContainerId>) -> WebResult<()> {
    container::stop(data.0)
}

#[rocket::put("/restart", format = "application/json", data = "<data>")]
fn restart_container(data: Json<ContainerId>) -> WebResult<()> {
    container::restart(data.0)
}

#[rocket::put("/create", format = "application/json", data = "<data>")]
fn create_container(data: Json<ContainerCreateOptions>) -> WebResult<CreateContainerResponse> {
    container_creation::create(data.0)
}

#[rocket::put("/remove", format = "application/json", data = "<data>")]
fn remove_container(data: Json<ContainerId>) -> WebResult<()> {
    container::remove(data.0)
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

#[rocket::get("/default_config?<container_name>", format = "application/json")]
fn get_default_creation_options(container_name: String) -> WebResult<ContainerCreateOptions> {
    compose::get_default_creation_options(container_name)
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
            rocket::routes![
                get_container_list,
                get_container_info,
                get_container_info_by_name,
                get_container_logs,
                start_container,
                stop_container,
                restart_container,
                create_container,
                remove_container,
            ],
        )
        .mount("/image/", rocket::routes![get_image_list])
        .mount(
            "/compose/",
            rocket::routes![get_docker_compose, get_default_creation_options],
        )
        .attach(cors)
        .launch()
        .await;
}
