use rocket::serde::json::{Json, Value};
use serde_json::json;

use crate::core::auth::auth::Claims;

#[get("/me")]
pub fn action(_claims: Claims) -> Json<Value> {
    let user_id = _claims.sub;
    Json(json!({
        "account" : ["A", user_id]
    }))
}