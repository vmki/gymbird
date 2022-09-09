use tokio_postgres::{connect, NoTls};

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
        client
            .query(
                "CREATE TABLE IF NOT EXISTS workouts (name TEXT, owner TEXT);",
                &[],
            )
            .await?;
        Ok(())
    }

    pub fn inner(&self) -> &tokio_postgres::Client {
        &self.inner
    }
}
