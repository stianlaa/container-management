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

#[derive(Debug, Deserialize)]
pub struct CreateContainerArgs {
    pub image_name: String,
    pub container_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateContainerResponse {
    pub id: String,
    pub warnings: Option<Vec<String>>,
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

#[derive(Debug, Deserialize)]
pub struct LogFilter {
    pub since: Option<i64>,
    pub until: Option<i64>,
    pub tail: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct Log {
    /// [(timestamp, message),...]
    pub messages: Vec<(String, String)>,
}

impl From<Vec<String>> for Log {
    fn from(output: Vec<String>) -> Self {
        Log {
            messages: output
                .into_iter()
                .map(|entry| {
                    // TODO fragile and perhaps inefficient, improve
                    let mut parts = entry.splitn(2, ' ').map(String::from);
                    let timestamp = parts.next().unwrap();
                    let message = parts.next().unwrap();
                    (timestamp, message)
                })
                .collect(),
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

pub fn get_info_by_name(container_name: String) -> WebResult<ContainerInfo> {
    let docker = Docker::connect_with_defaults().unwrap();
    match docker.list_containers(Some(true), None, None, ContainerFilters::new()) {
        Ok(container_list) => {
            match container_list
                .into_iter()
                .find(|container| container.Names.contains(&container_name))
            {
                Some(container) => get_info(container.Id),
                None => WebResult::Err(WebError::new(
                    500,
                    format!("unable to find container with name: {container_name:?}"),
                )),
            }
        }
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable to list containers: {:?}", error),
        )),
    }
}

pub fn get_info(container_id: String) -> WebResult<ContainerInfo> {
    let docker = Docker::connect_with_defaults().unwrap();
    match docker.container_info(&container_id) {
        Ok(container_info) => WebResult::Ok(container_info.into()),
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable to get container info: {:?}", error),
        )),
    }
}

pub fn get_container_logs(container_id: String, log_filter: LogFilter) -> WebResult<Log> {
    let docker = Docker::connect_with_defaults().unwrap();
    let log_options = dockworker::ContainerLogOptions {
        stdout: true,
        stderr: true,
        since: log_filter.since,
        until: log_filter.until,
        timestamps: Some(true),
        tail: log_filter.tail,
        follow: false,
    };
    match docker.log_container(container_id.as_str(), &log_options) {
        Ok(mut log_response) => match log_response.output() {
            Ok(output) => WebResult::Ok(Log::from(
                output.lines().map(String::from).collect::<Vec<String>>(),
            )),
            Err(error) => WebResult::Err(WebError::new(
                500,
                format!("unable to get log output: {:?}", error),
            )),
        },
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable to get container logs: {:?}", error),
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

pub fn create(create_args: CreateContainerArgs) -> WebResult<CreateContainerResponse> {
    let docker = Docker::connect_with_defaults().unwrap();
    let mut create = ContainerCreateOptions::new(create_args.image_name.as_str());
    create.tty(true);

    match docker.create_container(Some(create_args.container_name.as_str()), &create) {
        Ok(creation_response) => WebResult::Ok(CreateContainerResponse {
            id: creation_response.id,
            warnings: creation_response.warnings,
        }),
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
