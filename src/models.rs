use super::schema::*;
use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Serialize, Deserialize};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub last_login: chrono::NaiveDateTime,
}

impl User {
    pub fn from_details<U: Into<String>, E: Into<String>, H: Into<String>>(
        username: U,
        email: E,
        pwd: H
    ) -> Self {
        Self {
            id: 0,
            username: username.into(),
            first_name: None,
            last_name: None,
            email: email.into(),
            hash: pwd.into(),
            created_at: chrono::Utc::now().naive_utc(),
            last_login: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub id: i32,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "invitations"]
pub struct Invitation {
    pub id: uuid::Uuid,
    pub email: String,
    pub expires_at: chrono::NaiveDateTime,
}

impl<T> From<T> for Invitation
where
    T: Into<String>,
{
    fn from(email: T) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            email: email.into(),
            expires_at: chrono::Utc::now().naive_utc() + chrono::Duration::days(1),
        }
    }
}