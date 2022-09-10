use tokio_postgres::Row;
pub type UUID = String;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    name: String,
    username: String,
    user_id: UUID,
}

impl From<&Row> for UserProfile {
    fn from(data: &Row) -> Self {
        Self {
            name: data.get("name"),
            username: data.get("username"),
            user_id: data.get("user_id"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccount {
    email: String,
    password: String,
    user_id: UUID,
}

impl From<&Row> for UserAccount {
    fn from(data: &Row) -> Self {
        Self {
            email: data.get("email"),
            password: data.get("password"),
            user_id: data.get("user_id"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    user_profile: UserProfile,
    user_account: UserAccount,
    id: UUID,
}

impl From<(UserAccount, UserProfile)> for User {
    fn from(data: (UserAccount, UserProfile)) -> Self {
        Self {
            user_account: data.0,
            id: data.1.user_id.clone(),
            user_profile: data.1,
        }
    }
}
