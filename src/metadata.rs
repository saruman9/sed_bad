//! Metadata implementation.
//!
//! TODO Write documentation.

use chrono::{DateTime, UTC};

use user::User;
use category::Category;

pub struct Metadata {
    c_time: DateTime<UTC>,
    m_time: Option<DateTime<UTC>>,
    author: User, // TODO Reference?
    category: Category,
    status: Status,
}

enum Status {
    Beginning,
    InProgress,
    Complete,
}

impl Metadata {
    pub fn new(author: User, category: Category) -> Self {
        Metadata {
            c_time: UTC::now(),
            m_time: None,
            author: author, // TODO Reference?
            category: category,
            status: Status::Beginning,
        }
    }
}
