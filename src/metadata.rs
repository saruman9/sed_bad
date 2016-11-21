//! Metadata implementation.
//!
//! TODO Write documentation.

use chrono::{DateTime, UTC, TimeZone};

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
    date_expired: DateTime<UTC>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Beginning,
    InProgress,
    Complete,
}

impl Status {
    pub fn get_num(&self) -> i64 {
        match *self {
            Status::Beginning => 0,
            Status::InProgress => 1,
            Status::Complete => 2,
        }
    }

    pub fn from_num(num: i64) -> Status {
        match num {
            0 => Status::Beginning,
            1 => Status::InProgress,
            2 => Status::Complete,
            _ => Status::Beginning,
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
            date_expired: UTC.ymd(date_expired.0 as i32, date_expired.1, date_expired.2).and_hms(0, 0, 0),
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn c_time(&self) -> DateTime<UTC> {
        self.c_time
    }

    pub fn m_time(&self) -> DateTime<UTC> {
        self.m_time
    }

    pub fn set_m_time(&mut self, m_time: DateTime<UTC>) {
        self.m_time = m_time;
    }

    pub fn author(&self) -> &User {
        &self.author
    }

    pub fn set_author(&mut self, author: User) {
        self.author = author;
        self.set_m_time(UTC::now());
    }

    pub fn category(&self) -> &Category {
        &self.category
    }

    pub fn set_category(&mut self, category: Category) {
        self.category = category;
        self.set_m_time(UTC::now());
    }

    pub fn status(&self) -> Status {
        self.status.clone()
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
        self.set_m_time(UTC::now());
    }

    pub fn date_expired(&self) -> DateTime<UTC> {
        self.date_expired
    }

    pub fn set_date_expired(&mut self, date: DateTime<UTC>) {
        self.date_expired = date;
        self.set_m_time(UTC::now());
    }

    pub fn update(&self, db:&Db) -> DbResult<i32> {
        db.conn()
            .execute("UPDATE metadata SET m_time = ?, author_id = ?, category_id = ?, status = ?, date_expired = ? WHERE id = ?;",
                     &[&self.m_time(),
                       &self.author().id(),
                       &self.category().id(),
                       &self.status().get_num(),
                       &self.date_expired(),
                       &self.id()])
            .map_err(From::from)
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
                      &self.date_expired])?;
        Ok(self.id())
    }

    pub fn get_by_id(db: &Db, id: i64) -> DbResult<Metadata> {
        db.conn().query_row_and_then("SELECT * FROM metadata WHERE id = ?;", &[&id], |row| {
            Ok(Metadata {
                id: row.get_checked(0)?,
                c_time: row.get_checked(1)?,
                m_time: row.get_checked(2)?,
                author: User::get_by_id(db, row.get_checked(3)?)?,
                category: Category::get_by_id(db, row.get_checked(4)?)?,
                status: Status::from_num(row.get_checked(5)?),
                date_expired: row.get_checked(6)?,
            })
        })

    }
}
