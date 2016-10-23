//! Comment implementation.
//!
//! TODO Write documentation.

use chrono::{DateTime, UTC};

use user::User;

pub struct Comment {
    id: u32,
    author: User, // TODO Reference?
    text: String,
    c_time: DateTime<UTC>,
}
