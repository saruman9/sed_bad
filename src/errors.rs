#![warn(missing_docs)]
//! Implementing of some types of errors.

use std::io;
use std::fmt;
use std::error::Error;

pub type AuthResult<T> = Result<T, AuthError>;

#[derive(Debug)]
pub enum AuthError {
    Io(io::Error),
    WrongSource,
}

impl From<io::Error> for AuthError {
    fn from(err: io::Error) -> Self {
        AuthError::Io(err)
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AuthError::Io(ref err) => err.fmt(f),
            AuthError::WrongSource => {
                write!(f, "Wrong source (file, DB, etc) of user's information.")
            }
        }
    }
}

impl Error for AuthError {
    fn description(&self) -> &str {
        match *self {
            AuthError::Io(ref err) => err.description(),
            AuthError::WrongSource => "wrong source of users",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            AuthError::Io(ref err) => Some(err),
            AuthError::WrongSource => None,
        }
    }
}
