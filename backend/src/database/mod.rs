use crate::error::Error;
use crate::models::{FetchUser, LoginParameters, RegistrationParameters};
use anyhow::Context;
use tokio_postgres::{connect, NoTls};
use uuid::Uuid;
use crate::error::Result;
use self::exercise::Exercise;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub mod user;
use user::*;

pub mod exercise;

const WORKOUT_TABLE_CREATION_STR: &str = include_str!("sql/workout_table.sql");
const EXERCISE_TABLE_CREATION_STR: &str = include_str!("sql/exercise_table.sql");
const USER_TABLE_CREATION_STR: &str = include_str!("sql/user_table.sql");
const SESSION_TOKEN_TABLE_CREATION_STR: &str = include_str!("sql/session_token_table.sql");

pub type ID = String;
pub type SessionToken = String;

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

    pub async fn login(&self, params: LoginParameters) -> Result<SessionToken> {
        // Check if the email address in `params` is found in the database.
        let user = match self.get_user_by_email(params.email).await {
            Ok(u) => u,
            Err(_) => return Err(Error::InvalidEmail),
        };

        // Check if the password hash in the database matches the password inputted by the user.
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        match Argon2::default().verify_password(params.password.as_bytes(), &parsed_hash) {
            // If it matches, generate a new session token, insert it into the database and return it.
            Ok(_) => {
                let session_token = self.generate_session_token(&user.id).await;
                return Ok(session_token);
            }
            Err(_) => return Err(Error::InvalidPassword),
        }
    }

    async fn check_and_initialize_tables(client: &tokio_postgres::Client) -> anyhow::Result<()> {
        // Execute table creation strings to check if the required SQL tables exist
        client.execute(WORKOUT_TABLE_CREATION_STR, &[]).await?;
        client.execute(EXERCISE_TABLE_CREATION_STR, &[]).await?;
        client.execute(USER_TABLE_CREATION_STR, &[]).await?;
        client
            .execute(SESSION_TOKEN_TABLE_CREATION_STR, &[])
            .await?;
        Ok(())
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<User> {
        let rows = self
            .inner
            .query("SELECT * FROM users WHERE email = $1", &[&email])
            .await?;

        match rows.get(0) {
            Some(r) => Ok(User::from(r)),
            None => Err(Error::InvalidEmail),
        }

    }

    pub async fn get_all_exercises(&self) -> Vec<Exercise> {
        self.inner.query("SELECT * FROM exercises;", &[])
            .await.unwrap().iter().map(|row| Exercise::from(row)).collect()
    }

    pub async fn get_exercise(&self, id: ID) -> Result<Exercise> {
        match self.inner.query("SELECT * FROM exercises WHERE id = $1", &[&id]).await {
            Ok(rows) => Ok(Exercise::from(rows.first().unwrap().clone())),
            Err(e) => Err(Error::InvalidID),
        }
    }

    pub async fn create_user(&self, data: RegistrationParameters) -> anyhow::Result<SessionToken> {
        let id = Uuid::new_v4().to_string();

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(data.password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        self.inner
            .query(
                "INSERT INTO users (email, password, name, username, id) VALUES($1, $2, $3, $4, $5)",
                &[
                    &data.email,
                    &password_hash,
                    &data.name,
                    &data.username,
                    &id
                ],
            )
            .await?;

        let session_token = self.generate_session_token(id).await;

        Ok(session_token)
    }

    pub async fn fetch_user(&self, session_token: SessionToken) -> Result<FetchUser> {
        let user_id = match self.authorize(session_token).await {
            Ok(uid) => uid,
            Err(_) => return Err(Error::InvalidSessionToken),
        };

        Ok(FetchUser::from(
            &self
                .inner
                .query("SELECT * FROM users WHERE id = $1", &[&user_id])
                .await
                .unwrap()[0],
        ))
    }

    pub async fn log_out(&self, session_token: SessionToken) {
        if let Err(e) = self.inner.execute("DELETE FROM session_tokens WHERE token = $1", &[&session_token]).await {
            eprintln!("Error when trying to log out with session token {}: {}", session_token, e); 
        }
    }

    async fn authorize(&self, session_token: SessionToken) -> Result<String> {
        let rows = self
            .inner
            .query(
                "SELECT * FROM session_tokens WHERE token = $1",
                &[&session_token],
            )
            .await?;

        match rows.get(0) {
            Some(row) => Ok(row.get("user_id")),
            None => Err(Error::InvalidSessionToken),
        }
    }

    async fn generate_session_token<T>(&self, user_id: T) -> String 
        where T: AsRef<str>
    {
        let session_token = Uuid::new_v4().to_string();
        self.inner
            .execute(
                "INSERT INTO session_tokens (user_id, token) VALUES($1, $2);",
                &[&user_id.as_ref(), &session_token],
            )
            .await
            .unwrap();

        session_token
    }

    pub fn inner(&self) -> &tokio_postgres::Client {
        &self.inner
    }
}
