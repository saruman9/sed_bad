//! Implementing of some types of errors.
//!
//! TODO Write docs.

use rusqlite;

use std::io;
use std::fmt;
use std::error::Error;

pub type AuthResult<T> = Result<T, AuthError>;
pub type DbResult<T> = Result<T, DbError>;

#[derive(Debug)]
pub enum AuthError {
    IoError(io::Error),
    WrongSource,
}

#[derive(Debug)]
pub enum DbError {
    SqliteError(rusqlite::Error),
    IoError(io::Error),
}

impl From<io::Error> for AuthError {
    fn from(err: io::Error) -> Self {
        AuthError::IoError(err)
    }
}

impl From<rusqlite::Error> for DbError {
    fn from(err: rusqlite::Error) -> Self {
        DbError::SqliteError(err)
    }
}

impl From<io::Error> for DbError {
    fn from(err: io::Error) -> Self {
        DbError::IoError(err)
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AuthError::IoError(ref err) => err.fmt(f),
            AuthError::WrongSource => {
                write!(f, "Wrong source (file, DB, etc) of user's information.")
            }
        }
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DbError::SqliteError(ref err) => err.fmt(f),
            DbError::IoError(ref err) => err.fmt(f),
        }
    }
}

impl Error for AuthError {
    fn description(&self) -> &str {
        match *self {
            AuthError::IoError(ref err) => err.description(),
            AuthError::WrongSource => "wrong source of users",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            AuthError::IoError(ref err) => Some(err),
            AuthError::WrongSource => None,
        }
    }
}

impl Error for DbError {
    fn description(&self) -> &str {
        match *self {
            DbError::SqliteError(ref err) => err.description(),
            DbError::IoError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            DbError::SqliteError(ref err) => Some(err),
            DbError::IoError(ref err) => Some(err),
        }
    }
}
