use crate::web_result::{WebError, WebResult};
use dockworker::container::{Container, ContainerFilters};
use dockworker::{ContainerCreateOptions, Docker};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ContainerDescription {
    pub image_name: String,
    pub container_name: String,
}

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

pub fn start(container_description: ContainerDescription) -> WebResult<()> {
    let docker = Docker::connect_with_defaults().unwrap();
    let mut create = ContainerCreateOptions::new(container_description.image_name.as_str());
    create.tty(true);

    match docker.create_container(Some(container_description.container_name.as_str()), &create) {
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
