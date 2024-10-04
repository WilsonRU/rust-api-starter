use rocket::{http::Status, serde::json::{Json, Value}, State};
use serde::Deserialize;
use rocket::put;
use serde_json::json;

use crate::{adapters::{database::db::DbPool, http::error_handling::ApiError}, core::{auth::auth::Claims, repository::UserRepository}};


#[derive(Deserialize)]
pub struct UpdateRequest {
    pub name: String
}

#[put("/", data = "<request>")]
pub fn action(request: Json<UpdateRequest>, claims: Claims, db: &State<DbPool>) -> Result<Json<Value>, ApiError> {
    let mut conn = db.get().expect("Failed to get a connection from the pool");

    if let Some(mut user) = UserRepository::find_by_id(&mut conn, &claims.sub) {
        user.name = request.name.to_string();

        if UserRepository::update(&mut conn, &user).is_ok() {
            return Ok(Json(json!({
                "message": "Nome de usuario atualizado com sucesso"
            })));
        }
    }


    Err(ApiError {
        status: Status::BadRequest,
        message: "Usuario não encontrado".to_string()
    })
}