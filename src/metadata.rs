//! Metadata implementation.
//!
//! TODO Write documentation.

use chrono::{Date, DateTime, UTC, Datelike};

use user::User;
use category::Category;
use errors::DbResult;
use db::Db;

#[derive(Debug)]
pub struct Metadata {
    id: i64,
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

impl Status {
    fn get_num(&self) -> i64 {
        match *self {
            Status::Beginning => 0,
            Status::InProgress => 1,
            Status::Complete => 2,
        }
    }
}

impl Metadata {
    pub fn new(author: &User, category: Category, date_expired: (u32, u32, u32)) -> Self {
        Metadata {
            id: 0,
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

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn c_time(&self) -> i64 {
        self.c_time.num_seconds_from_unix_epoch()
    }

    pub fn m_time(&self) -> i64 {
        self.m_time.num_seconds_from_unix_epoch()
    }

    pub fn author(&self) -> &User {
        &self.author
    }

    pub fn category(&self) -> &Category {
        &self.category
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn date_expired(&self) -> i64 {
        self.date_expired.num_days_from_ce() as i64
    }

    pub fn save_to_db(&mut self, db: &Db) -> DbResult<i64> {
        let mut stmt = db.conn()
            .prepare("
INSERT INTO metadata VALUES (NULL, ?, ?, (SELECT id FROM users WHERE name  \
                      = ?), (SELECT id FROM categories WHERE name = ?), ?, ?);
")?;
        self.id = stmt.insert(&[&self.c_time(),
                      &self.m_time(),
                      &self.author().name(),
                      &self.category().name(),
                      &self.status().get_num(),
                      &self.date_expired()])?;
        Ok(self.id())
    }
}
