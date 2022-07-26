extern crate dockworker;

use crate::container::list_containers;
use crate::web_result::WebResult;
use dockworker::container::Container;

mod container;
mod web_result;

#[rocket::get("/list", format = "json")]
fn list() -> WebResult<Vec<Container>> {
    list_containers()
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![list])
        .launch()
        .await;
}
