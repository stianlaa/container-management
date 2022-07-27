use crate::web_result::{WebError, WebResult};
use dockworker::container::{Container, ContainerFilters};
use dockworker::{ContainerCreateOptions, Docker};
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct StartArgs {
    pub image_name: String,
    pub container_name: String,
}

#[derive(Debug, Deserialize)]
pub struct StopArgs {
    pub container_id: String,
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

pub fn start(start_args: StartArgs) -> WebResult<()> {
    let docker = Docker::connect_with_defaults().unwrap();
    let mut create = ContainerCreateOptions::new(start_args.image_name.as_str());
    create.tty(true);

    match docker.create_container(Some(start_args.container_name.as_str()), &create) {
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

pub fn stop(stop_args: StopArgs) -> WebResult<()> {
    let docker = Docker::connect_with_defaults().unwrap();
    // TODO receive and deserialize duration from request
    match docker.stop_container(stop_args.container_id.as_str(), Duration::from_secs(1)) {
        Ok(_) => WebResult::Ok(()),
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable to stop container: {:?}", error),
        )),
    }
}
