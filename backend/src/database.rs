use crate::models::RegistrationParameters;
use crate::user::*;
use anyhow::Context;
use std::error::Error;
use tokio_postgres::{connect, NoTls};

const WORKOUT_TABLE_CREATION_STR: &str = "CREATE TABLE IF NOT EXISTS workouts(\
name TEXT NOT NULL,\
owner_id TEXT NOT NULL,\
id TEXT PRIMARY KEY\
);";

const EXERCISE_TABLE_CREATION_STR: &str = "CREATE TABLE IF NOT EXISTS exercises (\
workout_id TEXT NOT NULL,\
idx SMALLINT NOT NULL,\
name TEXT NOT NULL,\
muscles_trained TEXT NOT NULL\
);";

const USER_TABLE_CREATION_STR: &str = "CREATE TABLE IF NOT EXISTS users(\
email TEXT UNIQUE NOT NULL,\
password TEXT NOT NULL,\
name TEXT NOT NULL,\
username TEXT UNIQUE NOT NULL,\
id TEXT PRIMARY KEY\
);";

#[derive(Debug, Clone)]
pub struct Credentials<'a> {
    pub username: &'a str,
    pub db_name: &'a str,
    pub host: &'a str,
}

pub struct Database {
    inner: tokio_postgres::Client,
}

impl Database {
    pub async fn connect(credentials: Credentials<'_>) -> anyhow::Result<Self> {
        let credential_string = format!(
            "dbname={} user={} host={}",
            credentials.db_name, credentials.username, credentials.host
        );

        let (client, connection) = connect(&credential_string, NoTls).await?;

        // Start the postgres connection.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("database connection error: {}", e);
            }
        });

        // TODO: Check if proper tables exist
        Self::check_and_initialize_tables(&client).await?;

        Ok(Self { inner: client })
    }

    async fn check_and_initialize_tables(client: &tokio_postgres::Client) -> anyhow::Result<()> {
        // Execute table creation strings to check if the required SQL tables exist
        client.execute(WORKOUT_TABLE_CREATION_STR, &[]).await?;
        client.execute(EXERCISE_TABLE_CREATION_STR, &[]).await?;
        client.execute(USER_TABLE_CREATION_STR, &[]).await?;
        Ok(())
    }

    pub async fn get_user_by_email(&self, email: String) -> anyhow::Result<User> {
        let rows = self
            .inner
            .query("SELECT * FROM users WHERE email = $1", &[&email])
            .await?;

        let row = rows.get(0).context("User not found.")?;

        Ok(User::from(row))
    }

    pub async fn create_user(&self, data: RegistrationParameters) -> anyhow::Result<User> {
        self.inner
            .query(
                "INSERT INTO users (email, password, name, username, user_id) VALUES($1, $2, $3, $4, $5)",
                &[
                    &data.email,
                    &data.password,
                    &data.name,
                    &data.username,
                    &"246",
                ],
            )
            .await?;

        Ok(self.get_user_by_email(data.email).await?)
    }

    pub fn inner(&self) -> &tokio_postgres::Client {
        &self.inner
    }
}
