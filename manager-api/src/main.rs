extern crate dockworker;

use crate::container::{StartArgs, StopArgs};
use crate::web_result::WebResult;
use dockworker::container::Container;
use dockworker::image::SummaryImage;
use rocket::serde::json::Json;

mod container;
mod image;
mod web_result;

//container
#[rocket::get("/list", format = "application/json")]
fn list_containers() -> WebResult<Vec<Container>> {
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
fn list_images() -> WebResult<Vec<SummaryImage>> {
    image::list()
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/container/",
            rocket::routes![list_containers, start_containers, stop_containers],
        )
        .mount("/image/", rocket::routes![list_images])
        .launch()
        .await;
}
