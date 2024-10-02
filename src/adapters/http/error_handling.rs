use rocket::response::{self, Responder};
use rocket::serde::json::Json;
use rocket::Request;
use rocket::http::Status;
use serde::Serialize;
use std::io::Cursor;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub status: Status,
    pub message: String,
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let json_string = serde_json::to_string(&self).unwrap();
        
        response::Response::build()
            .status(self.status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(json_string.len(), Cursor::new(json_string))
            .ok()
    }
}

#[catch(409)]
pub fn businesslogic() -> Json<ApiError> {
    Json(ApiError {
        status: Status::Conflict,
        message: "[BUSINESS LOGIC]".to_string(),
    })
}

#[catch(500)]
pub fn internal_error() -> Json<ApiError> {
    Json(ApiError {
        status: Status::InternalServerError,
        message: "[INTERNAL SERVER ERROR]".to_string(),
    })
}
