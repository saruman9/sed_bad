//! Metadata implementation.
//!
//! TODO Write documentation.

use chrono::{Date, DateTime, UTC, Datelike};

use user::User;
use category::Category;

#[derive(Debug)]
pub struct Metadata {
    c_time: DateTime<UTC>,
    m_time: DateTime<UTC>,
    author: User, // TODO Reference?
    category: Category,
    status: Status,
    date_expired: Date<UTC>,
}

#[derive(Debug)]
enum Status {
    Beginning,
    InProgress,
    Complete,
}

impl Metadata {
    pub fn new(author: &User, category: Category, date_expired: (u32, u32, u32)) -> Self {
        Metadata {
            c_time: UTC::now(),
            m_time: UTC::now(),
            author: author.clone(), // TODO Reference?
            category: category,
            status: Status::Beginning,
            date_expired: UTC::today()
                .with_year(date_expired.0 as i32)
                .unwrap()
                .with_month(date_expired.1)
                .unwrap()
                .with_day(date_expired.2)
                .unwrap(),
        }
    }
}
