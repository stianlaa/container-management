extern crate dockworker;

use dockworker::Docker;

#[rocket::get("/list")]
fn list() -> String {
    let docker = Docker::connect_with_defaults().unwrap();
    println!("{:#?}", docker.system_info().unwrap());
    String::from("placeholder")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![list])
        .launch()
        .await;
}
