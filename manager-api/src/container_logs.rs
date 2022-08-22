use crate::web_result::{WebError, WebResult};
use dockworker::Docker;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LogFilter {
    pub since: Option<i64>,
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

pub fn get_container_logs(container_id: String, log_filter: LogFilter) -> WebResult<Log> {
    let docker = Docker::connect_with_defaults().unwrap();
    let log_options = dockworker::ContainerLogOptions {
        stdout: true,
        stderr: true,
        since: log_filter.since,
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
