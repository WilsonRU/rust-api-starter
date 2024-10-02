use rocket::request::{FromRequest, Outcome, Request};

use super::auth::{authenticate, Claims};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization")
            .and_then(|header| header.strip_prefix("Bearer "));

        if let Some(token) = token {
            match authenticate(token) {
                Ok(token_data) => {
                    return Outcome::Success(token_data);
                }
                Err(_) => {
                    return Outcome::Forward(rocket::http::Status::Unauthorized);
                }
            }
        }

        Outcome::Forward(rocket::http::Status::Unauthorized)
    }
}