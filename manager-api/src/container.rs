use crate::web_result::{WebError, WebResult};
use dockworker::container::ContainerFilters;
use dockworker::{ContainerCreateOptions, Docker};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
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

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateContainerArgs {
    pub image_name: String,
    pub container_name: String,
}

#[derive(Debug, Deserialize)]
pub struct ContainerId {
    pub container_id: String,
}

#[derive(Debug, Serialize)]
pub struct Container {
    pub id: String,
    pub image: String,
    pub command: String,
    pub state: State,
    pub name: String,
}

impl From<dockworker::container::Container> for Container {
    fn from(container: dockworker::container::Container) -> Self {
        Container {
            id: container.Id,
            image: container.Image,
            command: container.Command,
            state: State::from(container.State),
            name: container
                .Names
                .get(0)
                // TODO find better way to handle
                .get_or_insert(&String::from("missing_name"))
                .clone()
                .replace('/', ""),
        }
    }
}

pub fn list() -> WebResult<BTreeMap<String, Container>> {
    let docker = Docker::connect_with_defaults().unwrap();
    let filter = ContainerFilters::new();
    match docker.list_containers(Some(true), None, None, filter) {
        Ok(container_list) => {
            let mapped_containers: BTreeMap<String, Container> = container_list
                .into_iter()
                .map(Container::from)
                .map(|container| (container.name.clone(), container))
                .collect();
            WebResult::Ok(mapped_containers)
        }
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable to list containers: {:?}", error),
        )),
    }
}

pub fn start(start_args: ContainerId) -> WebResult<()> {
    let docker = Docker::connect_with_defaults().unwrap();
    match docker.start_container(&start_args.container_id) {
        Ok(_) => WebResult::Ok(()),
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable to start container: {:?}", error),
        )),
    }
}

pub fn stop(stop_args: ContainerId) -> WebResult<()> {
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

pub fn create(create_args: CreateContainerArgs) -> WebResult<()> {
    let docker = Docker::connect_with_defaults().unwrap();
    let mut create = ContainerCreateOptions::new(create_args.image_name.as_str());
    create.tty(true);

    match docker.create_container(Some(create_args.container_name.as_str()), &create) {
        Ok(_container) => WebResult::Ok(()),
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable to create container: {:?}", error),
        )),
    }
}

pub fn remove(remove_args: ContainerId) -> WebResult<()> {
    let docker = Docker::connect_with_defaults().unwrap();
    match docker.remove_container(remove_args.container_id.as_str(), None, Some(true), None) {
        Ok(_) => WebResult::Ok(()),
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable to remove container: {:?}", error),
        )),
    }
}
