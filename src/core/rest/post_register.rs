use rocket::State;
use rocket::{post, http::Status};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::adapters::database::db::DbPool;
use crate::core::repository::{find_user_by_username, store};
use crate::adapters::http::error_handling::ApiError;

#[derive(Deserialize)]
pub struct RegsiterRequest {
    pub username: String,
    pub password: String,
    pub name: String
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub message: String
}

#[post("/register", data = "<register>")]
pub fn action(register: Json<RegsiterRequest>, db: &State<DbPool>) -> Result<Json<RegisterResponse>, ApiError> {
    let mut conn = db.get().expect("Failed to get a connection from the pool");

    if let Some(_) = find_user_by_username(&mut conn, &register.username) {
        return Err(ApiError {
            status: Status::BadRequest,
            message: "Email informado existe.".to_string()
        })
    }

    if store(&mut conn, &register.username, &register.password, &register.name).is_ok() {
        return Ok(Json(RegisterResponse {
            message: "Conta Criada com sucesso".to_string()
        }))
    }

    Err(ApiError {
        status: Status::Conflict,
        message: "Ocorreu um problema".to_string(),
    })  
}