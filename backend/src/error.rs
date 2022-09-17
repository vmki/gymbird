use std::error::Error as StdError;
use std::fmt;
use warp::reject::Reject;

#[derive(Debug)]
pub enum Error {
    InvalidPassword,
    InvalidEmail,
    InvalidSessionToken,
    Argon2(argon2::Error),
}

impl Reject for Error {}

impl fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        use Error::*;

        let msg = match self {
            InvalidPassword => "An invalid password was provided.".into(),
            InvalidEmail => "An invalid email address was provided.".into(),
            InvalidSessionToken => "An invalid session token was provided.".into(),
            &Argon2(e) => e.to_string(),
        };

        write!(f, "{}", msg)
    }
}
