use dockworker::container::{Container, ContainerFilters};
use dockworker::Docker;
use rocket::Response;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ResponseWrapper<T: Serialize> {
    pub body: T,
}

impl<'r, T: Serialize> rocket::response::Responder<'r, 'static> for ResponseWrapper<T> {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let body = serde_json::to_string(&self.body).unwrap();
        Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(rocket::http::ContentType::JSON)
            .status(rocket::http::Status::new(200))
            .ok()
    }
}

pub fn list_containers() -> ResponseWrapper<Vec<Container>> {
    let docker = Docker::connect_with_defaults().unwrap();
    let filter = ContainerFilters::new();
    ResponseWrapper::<Vec<Container>> {
        body: docker.list_containers(None, None, None, filter).unwrap(),
    }
}
