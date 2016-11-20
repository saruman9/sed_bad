//! Comment implementation.
//!
//! TODO Write documentation.

use chrono::{DateTime, UTC};

use user::User;

#[derive(Debug)]
pub struct Comment {
    id: u32,
    author: User, // TODO Reference?
    text: String,
    c_time: DateTime<UTC>,
}

impl Comment {
    pub fn new(user: &User, text: String) -> Self {
        Comment {
            id: 0,
            author: user.clone(),
            text: text,
            c_time: UTC::now(),
        }
    }
}
