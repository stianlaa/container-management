use crate::web_result::{WebError, WebResult};
use dockworker::container::ContainerFilters;
use dockworker::{ContainerCreateOptions, Docker};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize)]
pub enum State {
    Created,
    Running,
    Restarting,
    Exited,
    Paused,
    Dead,
    Unknown,
}

impl From<String> for State {
    fn from(state: String) -> Self {
        match state.as_str() {
            "created" => State::Created,
            "running" => State::Running,
            "restarting" => State::Restarting,
            "exited" => State::Exited,
            "paused" => State::Paused,
            "dead" => State::Dead,
            _ => State::Unknown,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct StartArgs {
    pub image_name: String,
    pub container_name: String,
}

#[derive(Debug, Deserialize)]
pub struct StopArgs {
    pub container_id: String,
}

#[derive(Debug, Serialize)]
pub struct Container {
    pub id: String,
    pub image: String,
    pub command: String,
    pub state: State,
    pub names: Vec<String>,
}

impl From<dockworker::container::Container> for Container {
    fn from(container: dockworker::container::Container) -> Self {
        Container {
            id: container.Id,
            image: container.Image,
            command: container.Command,
            state: State::from(container.State),
            names: container.Names,
        }
    }
}

pub fn list() -> WebResult<Vec<Container>> {
    let docker = Docker::connect_with_defaults().unwrap();
    let filter = ContainerFilters::new();
    match docker.list_containers(None, None, None, filter) {
        Ok(container_list) => {
            let mapped_containers: Vec<Container> =
                container_list.into_iter().map(|c| c.into()).collect();
            WebResult::Ok(mapped_containers)
        }
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
