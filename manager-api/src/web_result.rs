use rocket::response::Response;
use serde::Serialize;

#[derive(Debug, serde::Serialize)]
pub struct ErrorContent {
    pub code: u16,
    reason: String,
    description: String,
}

#[derive(Debug, Serialize)]
pub struct WebError {
    pub error: ErrorContent,
}

pub enum WebResult<T: Serialize> {
    Ok(T),
    Err(WebError),
}

impl WebError {
    pub fn new(code: u16, description: String) -> WebError {
        let reason = match code {
            400 => "Bad Request".to_string(),
            401 => "Unauthorized".to_string(),
            404 => "Not found".to_string(),
            500 => "Internal server error".to_string(),
            _ => "Error".to_string(),
        };
        WebError {
            error: ErrorContent {
                code,
                reason,
                description,
            },
        }
    }
}

impl<'r> rocket::response::Responder<'r, 'static> for WebError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let body = serde_json::to_string(&self).unwrap();
        Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(rocket::http::ContentType::JSON)
            .status(rocket::http::Status::new(self.error.code))
            .ok()
    }
}

impl<'r, T: Serialize> rocket::response::Responder<'r, 'static> for WebResult<T> {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            WebResult::Ok(body) => {
                let json_body = serde_json::to_string(&body).unwrap();
                Response::build()
                    .sized_body(json_body.len(), std::io::Cursor::new(json_body))
                    .header(rocket::http::ContentType::JSON)
                    .status(rocket::http::Status::new(200))
                    .ok()
            }
            WebResult::Err(web_error) => {
                let json_body = serde_json::to_string(&web_error).unwrap();
                Response::build()
                    .sized_body(json_body.len(), std::io::Cursor::new(json_body))
                    .header(rocket::http::ContentType::JSON)
                    .status(rocket::http::Status::new(web_error.error.code))
                    .ok()
            }
        }
    }
}
