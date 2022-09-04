use crate::web_result::WebError;
use crate::WebResult;
use dockworker::Docker;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateContainerResponse {
    pub id: String,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerCreateOptions {
    pub container_name: String,
    pub hostname: String,
    pub domainname: String,
    pub user: String,
    pub attach_stdin: bool,
    pub attach_stdout: bool,
    pub attach_stderr: bool,
    // pub exposed_ports: HashMap<String, Any>, not sure the type that this would need to be
    pub tty: bool,
    pub open_stdin: bool,
    pub stdin_once: bool,
    pub env: Vec<String>,
    pub cmd: Vec<String>,
    pub entrypoint: Vec<String>,
    pub image: String,
    pub labels: HashMap<String, String>,
    // pub volumes: HashMap<String, Any>, not sure the type that this would need to be.
    // pub healthcheck: Not sure the type that this would be
    pub working_dir: PathBuf,
    pub network_disabled: bool,
    pub mac_address: String,
    pub on_build: Vec<String>,
    pub stop_signal: String,
    pub stop_timeout: Duration,
    // pub host_config: Option<ContainerHostConfig>,
    // pub networking_config: Option<NetworkingConfig>,
}

impl From<ContainerCreateOptions> for dockworker::ContainerCreateOptions {
    fn from(options: ContainerCreateOptions) -> Self {
        let mut creation_opt = dockworker::ContainerCreateOptions::new(options.image.as_str())
            .hostname(options.hostname)
            .domainname(options.domainname)
            .user(options.user)
            .attach_stdin(options.attach_stdin)
            .attach_stdout(options.attach_stdout)
            .attach_stderr(options.attach_stderr)
            .tty(options.tty)
            .open_stdin(options.open_stdin)
            .stdin_once(options.stdin_once)
            .entrypoint(options.entrypoint)
            .working_dir(options.working_dir)
            .network_disabled(options.network_disabled)
            .mac_address(options.mac_address)
            .on_build(options.on_build)
            .stop_signal(options.stop_signal)
            .stop_timeout(options.stop_timeout)
            .clone();
        options.cmd.into_iter().for_each(|s| {
            creation_opt.cmd(s);
        });
        options.labels.into_iter().for_each(|(s, t)| {
            creation_opt.label(s, t);
        });
        creation_opt
    }
}

pub fn create(creation_options: ContainerCreateOptions) -> WebResult<CreateContainerResponse> {
    let docker = Docker::connect_with_defaults().unwrap();
    let container_name = creation_options.container_name.clone();
    let options = dockworker::ContainerCreateOptions::from(creation_options);

    match docker.create_container(Some(container_name.as_str()), &options) {
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
