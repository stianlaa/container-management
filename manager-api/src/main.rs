extern crate dockworker;

use crate::container::ContainerDescription;
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
fn start(data: Json<ContainerDescription>) -> WebResult<()> {
    container::start(data.0)
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/container/", rocket::routes![list, start])
        .launch()
        .await;
}
