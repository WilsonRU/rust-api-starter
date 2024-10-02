use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::adapters::database::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub name: String,
}
