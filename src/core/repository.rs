use diesel::prelude::*;

use crate::core::user::User;
use super::user::NewUser;
use crate::adapters::database::schema::users::dsl::*;


pub struct UserRepository;

impl UserRepository {

    pub fn find_by_id(conn: &mut PgConnection, user_id: &i32) -> Option<User> {
        users.filter(id.eq(user_id))
            .first::<User>(conn)
            .ok()
    }

    pub fn find_by_username(conn: &mut PgConnection, uname: &str) -> Option<User> {
        users.filter(username.eq(uname))
            .first::<User>(conn)
            .ok()
    }

    pub fn store(conn: &mut PgConnection, user: &NewUser) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users)
            .values(user)
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, user: &User) -> Result<User, diesel::result::Error> {
        diesel::update(users.filter(id.eq(user.id)))
        .set(user)
        .get_result(conn)
    }
}