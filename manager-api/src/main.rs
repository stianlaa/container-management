extern crate dockworker;

use crate::container::{StartArgs, StopArgs};
use crate::web_result::WebResult;
use dockworker::container::Container;
use rocket::serde::json::Json;

mod container;
mod web_result;

#[rocket::get("/list", format = "application/json")]
fn list() -> WebResult<Vec<Container>> {
    container::list()
}

#[rocket::put("/start", format = "application/json", data = "<data>")]
fn start(data: Json<StartArgs>) -> WebResult<()> {
    container::start(data.0)
}

#[rocket::put("/stop", format = "application/json", data = "<data>")]
fn stop(data: Json<StopArgs>) -> WebResult<()> {
    container::stop(data.0)
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/container/", rocket::routes![list, start, stop])
        .launch()
        .await;
}
