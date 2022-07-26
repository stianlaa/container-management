extern crate dockworker;

use crate::web_result::WebResult;
use dockworker::container::Container;

mod container;
mod web_result;

#[rocket::get("/list", format = "json")]
fn list() -> WebResult<Vec<Container>> {
    container::list()
}

#[rocket::get("/start?<image>", format = "json")]
fn start(image: &str) -> WebResult<()> {
    container::start(image)
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![list, start])
        .launch()
        .await;
}
