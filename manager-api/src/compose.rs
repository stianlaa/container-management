use crate::web_result::WebError;
use crate::WebResult;
use docker_compose_types::Compose;

pub fn get_docker_compose() -> WebResult<Compose> {
    // TODO: should be replaced with docker-compose file path
    let file_payload = std::fs::read_to_string("../docker-compose.yml").unwrap();
    match serde_yaml::from_str::<Compose>(&file_payload) {
        Ok(compose) => WebResult::Ok(compose),
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable read docker-compose file: {:?}", error),
        )),
    }
}
