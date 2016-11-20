//! Manipulate with DB.
//!
//! TODO Write documentation.

use std::env;
use std::fs;
use std::path::PathBuf;

use rusqlite;

use errors::DbResult;

pub struct Db {
    conn: rusqlite::Connection,
}

impl Db {
    pub fn new() -> DbResult<Self> {
        let db_file: PathBuf;
        if let Ok(home_dir) = env::var("HOME") {
            let db_dir = PathBuf::from(&home_dir).join(".config/sed_bad");
            fs::create_dir_all(db_dir)?;
            db_file = PathBuf::from(home_dir).join(".config/sed_bad/db.sqlite3");
        } else {
            db_file = env::current_dir().unwrap().join("sed_bad.sqlite3");
        }
        rusqlite::Connection::open(&db_file)
            .map_err(|err| From::from(err))
            .and_then(|conn| Db::init(conn))
    }

    fn init(conn: rusqlite::Connection) -> DbResult<Self> {
        conn.execute_batch("
BEGIN;
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY ASC,
    name TEXT NOT NULL UNIQUE,
    pass TEXT NOT NULL,
    pass_hash TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY ASC,
    name TEXT NOT NULL UNIQUE
);
CREATE TABLE IF NOT EXISTS metadata (
    id INTEGER PRIMARY KEY ASC,
    c_time INTEGER NOT NULL,
    m_time INTEGER NOT NULL,
    author_id NOT NULL REFERENCES users(id) ON UPDATE CASCADE ON DELETE CASCADE,
    category_id NOT NULL REFERENCES categories(id) ON UPDATE CASCADE ON DELETE CASCADE,
    status INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS docs (
    id INTEGER PRIMARY KEY ASC,
    name TEXT NOT NULL,
    metadata NOT NULL REFERENCES metadata(id) ON UPDATE CASCADE ON DELETE CASCADE,
    permission INTEGER NOT NULL,
    data BLOB,
    responsible INTEGER,
    changelog BLOB
);
CREATE TABLE IF NOT EXISTS comments (
    id INTEGER PRIMARY KEY ASC,
    author_id NOT NULL REFERENCES users(id) ON UPDATE CASCADE ON DELETE CASCADE,
    text TEXT NOT NULL,
    c_time INTEGET NOT NULL,
    doc_id NOT NULL REFERENCES docs(id) ON UPDATE CASCADE ON DELETE CASCADE
);
COMMIT;")?;
        Ok(Db { conn: conn })
    }

    pub fn conn(&self) -> &rusqlite::Connection {
        &self.conn
    }
}
