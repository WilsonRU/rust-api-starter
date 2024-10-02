use rocket::State;
use rocket::{post, http::Status};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::adapters::database::db::DbPool;
use crate::core::repository::find_user_by_username;
use crate::core::auth::auth::generate_token;
use crate::adapters::http::error_handling::ApiError;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub account: UserPresenter
}

#[derive(Serialize)]
pub struct UserPresenter {
    pub email: String,
    pub name: String,
}

#[post("/login", data = "<login>")]
pub fn action(login: Json<LoginRequest>, db: &State<DbPool>) -> Result<Json<LoginResponse>, ApiError> {
    let mut conn = db.get().expect("Failed to get a connection from the pool");

    if let Some(user) = find_user_by_username(&mut conn, &login.username) {
        if bcrypt::verify(&login.password, &user.password_hash).unwrap() {
            let token = generate_token(user.id);

            let user_presenter = UserPresenter {
                email: user.username,
                name: user.name
            };

            return Ok(Json(LoginResponse { token, account: user_presenter }));
        }
    }

    Err(ApiError {
        status: Status::BadRequest,
        message: "Usuário ou senha inválidos".to_string(),
    })  
}
