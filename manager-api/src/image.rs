use crate::web_result::{WebError, WebResult};
use dockworker::image::SummaryImage;
use dockworker::Docker;

pub fn list() -> WebResult<Vec<SummaryImage>> {
    let docker = Docker::connect_with_defaults().unwrap();
    match docker.images(true) {
        Ok(images) => WebResult::Ok(images),
        Err(error) => WebResult::Err(WebError::new(
            500,
            format!("unable to list images: {:?}", error),
        )),
    }
}
