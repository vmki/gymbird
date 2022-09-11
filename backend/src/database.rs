use crate::user::*;
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

const USER_ACCOUNT_TABLE_CREATION_STR: &str = "CREATE TABLE IF NOT EXISTS user_accounts(\
email TEXT UNIQUE NOT NULL,\
password TEXT NOT NULL,\
user_id TEXT PRIMARY KEY\
);";

const USER_PROFILE_TABLE_CREATION_STR: &str = "CREATE TABLE IF NOT EXISTS user_profiles(\
name TEXT NOT NULL,\
username TEXT UNIQUE NOT NULL,\
user_id TEXT PRIMARY KEY\
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
        client.query(WORKOUT_TABLE_CREATION_STR, &[]).await?;
        client.query(EXERCISE_TABLE_CREATION_STR, &[]).await?;
        client.query(USER_ACCOUNT_TABLE_CREATION_STR, &[]).await?;
        client.query(USER_PROFILE_TABLE_CREATION_STR, &[]).await?;
        Ok(())
    }

    pub async fn get_user(&self, uuid: UUID) -> Result<User, Box<dyn Error>> {
        let profile = UserProfile::from(
            &self
                .inner
                .query("SELECT * FROM user_profiles WHERE user_id = $1", &[&uuid])
                .await?[0],
        );

        let account = UserAccount::from(
            &self
                .inner
                .query("SELECT * FROM user_accounts WHERE user_id = $1", &[&uuid])
                .await?[0],
        );

        Ok(User::from((account, profile)))
    }

    pub async fn get_user_account(&self, email: String) -> Result<UserAccount, Box<dyn Error>> {
        Ok(UserAccount::from(
            &self
                .inner
                .query("SELECT * FROM user_accounts WHERE email = $1", &[&email])
                .await?[0],
        ))
    }

    pub fn inner(&self) -> &tokio_postgres::Client {
        &self.inner
    }
}
