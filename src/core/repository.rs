use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;

use crate::core::user::User;
use crate::adapters::database::schema::users::dsl::*;

pub struct UserRepository;

impl UserRepository {
    pub fn find_user_by_username(conn: &mut PgConnection, uname: &str) -> Option<User> {
        users.filter(username.eq(uname))
            .first::<User>(conn)
            .ok()
    }

    pub fn store(conn: &mut PgConnection, email: &str, password: &str, uname: &str) -> Result<User, diesel::result::Error> {
        let hashed_password = hash(password, DEFAULT_COST).expect("Erro to hash password");

        let new_user = User {
            id: 0,
            username: email.to_string(),
            password_hash: hashed_password,
            name: uname.to_string()
        };

        diesel::insert_into(users)
            .values(&new_user)
            .get_result(conn)
    }
}