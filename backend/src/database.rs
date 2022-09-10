use crate::user::{User, UUID};
use tokio_postgres::{connect, NoTls};

const WORKOUT_TABLE_CREATION_STR: &str = "CREATE TABLE IF NOT EXISTS workouts(\
name TEXT,\
owner_id TEXT,\
id TEXT\
);";

const EXERCISE_TABLE_CREATION_STR: &str = "CREATE TABLE IF NOT EXISTS exercises (\
workout_id TEXT,\
idx SMALLINT,\
name TEXT,\
muscles_trained TEXT\
);";

const USER_ACCOUNT_TABLE_CREATION_STR: &str = "CREATE TABLE IF NOT EXISTS user_accounts(\
email TEXT,\
password TEXT,\
user_id TEXT\
);";

const USER_PROFILE_TABLE_CREATION_STR: &str = "CREATE TABLE IF NOT EXISTS user_profiles(\
name TEXT,\
username TEXT,\
user_id TEXT\
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

    pub async fn get_user(&self, uuid: UUID) -> User {
        unimplemented!()
    }

    pub fn inner(&self) -> &tokio_postgres::Client {
        &self.inner
    }
}
