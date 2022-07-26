use crate::web_result::{WebError, WebResult};
use dockworker::container::{Container, ContainerFilters};
use dockworker::Docker;

pub fn list_containers() -> WebResult<Vec<Container>> {
    let docker = Docker::connect_with_defaults().unwrap();
    let filter = ContainerFilters::new();
    match docker.list_containers(None, None, None, filter) {
        Ok(container_list) => WebResult::Ok(container_list),
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable to list containers: {:?}", error),
        )),
    }
}
