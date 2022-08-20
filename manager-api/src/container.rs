use crate::web_result::{WebError, WebResult};
use dockworker::container::ContainerFilters;
use dockworker::Docker;
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
            state: container.State.into(),
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

#[derive(Debug, Serialize)]
pub struct ContainerInfo {
    pub app_armor_profile: String,
    pub args: Vec<String>,
    //pub config: Config, // TODO add or remove
    pub created: String,
    pub driver: String,
    pub hostname_path: String,
    pub hosts_path: String,
    pub id: String,
    pub image: String,
    pub log_path: String,
    pub mount_label: String,
    // pub mounts: Vec<Mount>,
    pub name: String,
    // pub networksettings: NetworkSettings,
    pub path: String,
    pub process_label: String,
    pub resolvconf_path: String,
    pub restart_count: u64,
    pub state: State,
}

impl From<dockworker::container::ContainerInfo> for ContainerInfo {
    fn from(container: dockworker::container::ContainerInfo) -> Self {
        ContainerInfo {
            app_armor_profile: container.AppArmorProfile,
            args: container.Args,
            // config: container.Config,
            created: container.Created,
            driver: container.Driver,
            hostname_path: container.HostnamePath,
            hosts_path: container.HostsPath,
            id: container.Id,
            image: container.Image,
            log_path: container.LogPath,
            mount_label: container.MountLabel,
            // mounts: container.Mounts,
            name: container.Name,
            // network_settings: container.NetworkSettings,
            path: container.Path,
            process_label: container.ProcessLabel,
            resolvconf_path: container.ResolvConfPath,
            restart_count: container.RestartCount,
            // TODO rename struct and use to status, better name
            state: container.State.Status.into(), // TODO map entire object? consider value of State? Rename current State to Status
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

pub fn get_info_by_name(container_name: String) -> WebResult<Option<ContainerInfo>> {
    let docker = Docker::connect_with_defaults().unwrap();
    match docker.list_containers(Some(true), None, None, ContainerFilters::new()) {
        Ok(container_list) => {
            match container_list.into_iter().find(|container| {
                container
                    .Names
                    .iter()
                    .map(|name| name.replace('/', ""))
                    .any(|name| name == container_name)
            }) {
                Some(container) => get_info(container.Id),
                None => WebResult::Ok(None),
            }
        }
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable to list containers: {:?}", error),
        )),
    }
}

pub fn get_info(container_id: String) -> WebResult<Option<ContainerInfo>> {
    let docker = Docker::connect_with_defaults().unwrap();
    match docker.container_info(&container_id) {
        Ok(container_info) => WebResult::Ok(Some(container_info.into())),
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable to get container info: {:?}", error),
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

pub fn restart(start_args: ContainerId) -> WebResult<()> {
    let docker = Docker::connect_with_defaults().unwrap();
    match docker.restart_container(&start_args.container_id, Duration::from_secs(1)) {
        Ok(_) => WebResult::Ok(()),
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable to restart container: {:?}", error),
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
