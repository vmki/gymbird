use tokio_postgres::Row;
pub type UUID = String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    name: String,
    username: String,
    user_id: UUID,
    email: String,
    password: Option<String>,
}

impl From<&Row> for User {
    fn from(data: &Row) -> Self {
        Self {
            name: data.get("name"),
            username: data.get("username"),
            user_id: data.get("user_id"),
            email: data.get("email"),
            password: data.try_get("password").ok(),
        }
    }
}
