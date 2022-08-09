extern crate dockworker;

use crate::container::{Container, StartArgs, StopArgs};
use crate::web_result::WebResult;
use docker_compose_types::Compose;
use dockworker::image::SummaryImage;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::response::Response;
use rocket::serde::json::Json;
use rocket::Request;

mod compose;
mod container;
mod image;
mod web_result;
pub struct CORS;

// From: https://github.com/SergioBenitez/Rocket/issues/2142#issuecomment-1086660848
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new(
                "Access-Control-Allow-Headers",
                "content-type, authorization",
            ));
        }
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_header(Header::new("Vary", "Origin"));
    }
}

//container
#[rocket::get("/list", format = "application/json")]
fn get_container_list() -> WebResult<Vec<Container>> {
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
    let _ = rocket::build()
        .mount(
            "/container/",
            rocket::routes![get_container_list, start_containers, stop_containers],
        )
        .mount("/image/", rocket::routes![get_image_list])
        .mount("/compose/", rocket::routes![get_docker_compose])
        .attach(CORS)
        .launch()
        .await;
}
