use std::error::Error as StdError;
use std::fmt;
use std::result::Result as SResult;
use warp::reject::Reject;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    InvalidPassword,
    InvalidEmail,
    InvalidSessionToken,
    InvalidID,
    Argon2(argon2::Error),
    Database(#[from] tokio_postgres::Error),
}

impl Reject for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        let msg = match self {
            InvalidPassword => {
                "An invalid password was provided.".into()
            }
            InvalidEmail => {
                "An invalid email address was provided.".into()
            }
            InvalidSessionToken => {
                "An invalid session token was provided.".into()
            }
            InvalidID => "An invalid ID was provided.".into(),
            Argon2(e) => e.to_string(),
            Database(e) => e.to_string(),
        };

        write!(f, "{}", msg)
    }
}
