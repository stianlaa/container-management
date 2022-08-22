use crate::web_result::{WebError, WebResult};
use dockworker::container::ContainerFilters;
use dockworker::Docker;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::time::Duration;

const STOP_CONTAINER_TIMEOUT_MS: u64 = 1000;
const RESTART_CONTAINER_TIMEOUT_MS: u64 = 1000;

#[derive(Debug, Serialize)]
pub enum Status {
    Created,
    Running,
    Restarting,
    Exited,
    Paused,
    Dead,
    Unknown,
}

impl From<String> for Status {
    fn from(status: String) -> Self {
        match status.as_str() {
            "created" => Status::Created,
            "running" => Status::Running,
            "restarting" => Status::Restarting,
            "exited" => Status::Exited,
            "paused" => Status::Paused,
            "dead" => Status::Dead,
            _ => Status::Unknown,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ContainerId {
    pub container_id: String,
}

#[derive(Debug, Serialize)]
pub struct ContainerConfig {
    pub attach_stderr: bool,
    pub attach_stdin: bool,
    pub attach_stdout: bool,
    pub cmd: Vec<String>,
    pub domainname: String,
    pub entrypoint: Vec<String>,
    pub env: Vec<String>,
    pub exposed_ports: HashMap<String, HashMap<String, String>>,
    pub hostname: String,
    pub image: String,
    pub labels: HashMap<String, String>,
    pub on_build: Vec<String>,
    pub open_stdin: bool,
    pub stdin_once: bool,
    pub tty: bool,
    pub user: String,
    pub volumes: HashMap<String, HashMap<String, String>>,
    pub working_dir: String,
}

impl From<dockworker::container::Config> for ContainerConfig {
    fn from(config: dockworker::container::Config) -> Self {
        ContainerConfig {
            attach_stderr: config.AttachStderr,
            attach_stdin: config.AttachStdin,
            attach_stdout: config.AttachStdout,
            cmd: config.Cmd,
            domainname: config.Domainname,
            entrypoint: config.Entrypoint,
            env: config.Env,
            exposed_ports: config.ExposedPorts,
            hostname: config.Hostname,
            image: config.Image,
            labels: config.Labels,
            on_build: config.OnBuild,
            open_stdin: config.OpenStdin,
            stdin_once: config.StdinOnce,
            tty: config.Tty,
            user: config.User,
            volumes: config.Volumes,
            working_dir: config.WorkingDir,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ContainerInfo {
    pub app_armor_profile: String,
    pub args: Vec<String>,
    pub config: ContainerConfig,
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
    pub status: Status,
}

impl From<dockworker::container::ContainerInfo> for ContainerInfo {
    fn from(container: dockworker::container::ContainerInfo) -> Self {
        ContainerInfo {
            app_armor_profile: container.AppArmorProfile,
            args: container.Args,
            config: container.Config.into(),
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
            status: container.State.Status.into(),
        }
    }
}

pub fn list() -> WebResult<BTreeMap<String, Option<ContainerInfo>>> {
    let docker = Docker::connect_with_defaults().unwrap();
    let filter = ContainerFilters::new();
    match docker.list_containers(Some(true), None, None, filter) {
        Ok(containers) => {
            let container_info = containers
                .into_iter()
                .map(|container| {
                    let container_info_opt = match docker.container_info(&container.Id) {
                        Ok(info) => Some(ContainerInfo::from(info)),
                        Err(_) => None,
                    };
                    let name = container
                        .Names
                        .get(0)
                        .expect("containers should have at least one name")
                        .replace('/', "");
                    (name, container_info_opt)
                })
                .collect::<BTreeMap<String, Option<ContainerInfo>>>();

            WebResult::Ok(container_info)
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
    match docker.stop_container(
        stop_args.container_id.as_str(),
        Duration::from_millis(STOP_CONTAINER_TIMEOUT_MS),
    ) {
        Ok(_) => WebResult::Ok(()),
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable to stop container: {:?}", error),
        )),
    }
}

pub fn restart(start_args: ContainerId) -> WebResult<()> {
    let docker = Docker::connect_with_defaults().unwrap();
    match docker.restart_container(
        &start_args.container_id,
        Duration::from_secs(RESTART_CONTAINER_TIMEOUT_MS),
    ) {
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
