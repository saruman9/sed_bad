//! Comment implementation.
//!
//! TODO Write documentation.

use chrono::{DateTime, UTC};

use user::User;
use db::Db;
use errors::DbResult;

#[derive(Debug)]
pub struct Comment {
    id: i64,
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

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn author(&self) -> &User {
        &self.author
    }

    pub fn text(&self) -> &str {
        self.text.as_ref()
    }

    pub fn c_time(&self) -> i64 {
        self.c_time.num_seconds_from_unix_epoch()
    }

    pub fn save_to_db(&mut self, db: &Db, doc_id: i64) -> DbResult<i64> {
        let mut stmt = db.conn()
            .prepare("
INSERT INTO comments VALUES (NULL, (SELECT id FROM users WHERE name = ?), ?, ?, ?);
")?;
        self.id = stmt.insert(&[&self.author().name(), &self.text(), &self.c_time(), &doc_id])?;
        Ok(self.id())
    }
}
