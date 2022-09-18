use super::ID;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub username: String,
    pub id: ID,
    pub email: String,
    pub password: String,
}

impl From<&Row> for User {
    fn from(data: &Row) -> Self {
        Self {
            name: data.get("name"),
            username: data.get("username"),
            id: data.get("user_id"),
            email: data.get("email"),
            password: data.get("password"),
        }
    }
}
