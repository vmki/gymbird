use crate::database::{Credentials, Database};
use crate::database::ID;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::Row;

pub type State = Arc<Mutex<Database>>;

fn get_var(var: &str) -> String {
    env::var(var).expect(&format!("Couldn't find environment variable ${}.", var))
}

pub async fn new_state() -> anyhow::Result<State> {
    let credentials = Credentials {
        username: &get_var("DB_USERNAME"),
        db_name: &get_var("DB_NAME"),
        host: &get_var("DB_HOST"),
    };

    println!("{:#?}", credentials);

    Ok(Arc::new(Mutex::new(Database::connect(credentials).await?)))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginParameters {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationParameters {
    pub email: String,
    pub password: String,
    pub name: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchUser {
    pub name: String,
    pub username: String,
    pub id: ID,
    pub email: String,
}

impl From<&Row> for FetchUser {
    fn from(data: &Row) -> Self {
        Self {
            name: data.get("name"),
            username: data.get("username"),
            id: data.get("id"),
            email: data.get("email"),
        }
    }
}
