use crate::web_result::WebError;
use crate::{ContainerCreateOptions, WebResult};
use docker_compose_types::{Command, Compose, Entrypoint, Environment, Service};
use indexmap::IndexMap;
use serde_yaml::Error;

pub fn read_compose() -> Result<Compose, Error> {
    // TODO: should be replaced with docker-compose file path
    let file_payload = std::fs::read_to_string("../docker-compose.yml").unwrap();
    let nu_ver = std::env::var("NU_VER").expect("NU_VER env var is required");
    let file_payload = file_payload.replace("${NU_VER}", nu_ver.as_str());
    serde_yaml::from_str::<Compose>(&file_payload)
}

// TODO instead of returning compose file directly, consider mapping to own structure
pub fn get_docker_compose() -> WebResult<Compose> {
    match read_compose() {
        Ok(compose) => WebResult::Ok(compose),
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable read docker-compose file: {:?}", error),
        )),
    }
}

pub fn get_default_creation_options(container_name: String) -> WebResult<ContainerCreateOptions> {
    match read_compose() {
        // TODO Simplify
        Ok(compose) => match &compose.services {
            Some(services) => {
                let index_map: IndexMap<String, Option<Service>> = services.0.clone();
                let service_opt: &Option<Service> = index_map.get(&container_name).unwrap();
                match service_opt {
                    Some(service_ref) => {
                        let service = service_ref.clone();

                        let opt = ContainerCreateOptions {
                            container_name: service.container_name.unwrap_or_default(),
                            hostname: service.hostname.unwrap_or_default(),
                            domainname: String::default(),
                            user: service.user.unwrap_or_default(),
                            attach_stdin: false,
                            attach_stdout: false,
                            attach_stderr: false,
                            tty: true,
                            open_stdin: service.stdin_open,
                            stdin_once: false,
                            env: match service
                                .environment
                                .unwrap_or_else(|| Environment::List(vec![String::default()]))
                            {
                                Environment::List(elements) => elements,
                                Environment::KvPair(_) => vec![], // TODO implement
                            },
                            cmd: match service
                                .command
                                .unwrap_or_else(|| Command::Simple(String::default()))
                            {
                                Command::Simple(cmd) => vec![cmd],
                                Command::Args(cmds) => cmds,
                            },
                            entrypoint: match service
                                .entrypoint
                                .unwrap_or_else(|| Entrypoint::Simple(String::default()))
                            {
                                Entrypoint::List(elements) => elements,
                                Entrypoint::Simple(entrypoint) => vec![entrypoint],
                            },
                            image: service.image.unwrap_or_default(),
                            labels: Default::default(), // TODO implement
                            working_dir: Default::default(), // TODO implement
                            network_disabled: false,
                            mac_address: String::default(),
                            on_build: vec![],
                            stop_signal: service.stop_signal.unwrap_or_default(),
                            stop_timeout: Default::default(),
                        };
                        WebResult::Ok(opt)
                    }
                    None => {
                        WebResult::Err(WebError::new(500, format!("missing: {container_name}")))
                    }
                }
            }
            None => WebResult::Err(WebError::new(500, String::from("missing services"))),
        },
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable read docker-compose file: {:?}", error),
        )),
    }
}
