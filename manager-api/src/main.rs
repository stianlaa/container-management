extern crate dockworker;

use crate::container::{list_containers, ResponseWrapper};
use dockworker::container::Container;

mod container;

#[rocket::get("/list", format = "json")]
fn list() -> ResponseWrapper<Vec<Container>> {
    list_containers()
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![list])
        .launch()
        .await;
}
