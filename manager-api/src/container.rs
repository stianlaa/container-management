use crate::web_result::{WebError, WebResult};
use dockworker::container::{Container, ContainerFilters};
use dockworker::{ContainerCreateOptions, Docker};

pub fn list() -> WebResult<Vec<Container>> {
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

pub fn start(image: &str) -> WebResult<()> {
    let docker = Docker::connect_with_defaults().unwrap();
    let mut create = ContainerCreateOptions::new(image);
    create.tty(true);

    match docker.create_container(Some("testing"), &create) {
        Ok(container) => match docker.start_container(&container.id) {
            Ok(_) => WebResult::Ok(()),
            Err(error) => WebResult::Err(WebError::new(
                500,
                format!("unable to start container: {:?}", error),
            )),
        },
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable to create container: {:?}", error),
        )),
    }
}
