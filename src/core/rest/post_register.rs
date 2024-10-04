use rocket::State;
use rocket::{post, http::Status};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use bcrypt::{hash, DEFAULT_COST};

use crate::adapters::database::db::DbPool;
use crate::core::repository::UserRepository;
use crate::adapters::http::error_handling::ApiError;
use crate::core::user::NewUser;

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

    if let Some(_) = UserRepository::find_by_username(&mut conn, &register.username) {
        return Err(ApiError {
            status: Status::BadRequest,
            message: "Email informado existe.".to_string()
        })
    }

    let hashed_password = hash(register.password.to_string(), DEFAULT_COST).expect("Erro to hash password");

    let new_user = NewUser {
        username: register.username.to_string(),
        password_hash: hashed_password,
        name: register.name.to_string()
    };

    if UserRepository::store(&mut conn, &new_user).is_ok() {
        return Ok(Json(RegisterResponse {
            message: "Conta Criada com sucesso".to_string()
        }))
    }

    Err(ApiError {
        status: Status::Conflict,
        message: "Ocorreu um problema".to_string(),
    })  
}