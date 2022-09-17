use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidPassword,
    InvalidEmail,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        let msg = match self {
            InvalidPassword => "An invalid password was provided.",
            InvalidEmail => "An invalid email address was provided.",
        };

        write!(f, "{}", msg)
    }
}
