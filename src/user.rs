//! User implementation.
//!
//! TODO Write documentation.

use md5;

use errors::{AuthResult, DbResult};
use db::Db;

trait UserVec {
    fn is_auth(&self, name: &str, pass: &str) -> bool;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    id: i64,
    name: String,
    pass: String,
    pass_hash: String,
}

impl User {
    pub fn new<S: Into<String> + Clone>(name: S, pass: S) -> Self {
        // TODO Change ID of users.
        User {
            id: 0,
            name: name.into(),
            pass: pass.clone().into(),
            pass_hash: md5::compute(pass.into().as_bytes())
                .into_iter()
                .map(|c| format!("{:x}", c))
                .collect(),
        }
    }

    pub fn set(&mut self, user: User) {
        self.id = user.id;
        self.name = user.name;
        self.pass = user.pass;
        self.pass_hash = user.pass_hash;
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn pass(&self) -> &str {
        self.pass.as_ref()
    }

    pub fn pass_hash(&self) -> &str {
        self.pass_hash.as_ref()
    }

    pub fn is_root(&self) -> bool {
        self.name() == "root"
    }

    pub fn exists(&self, db: &Db) -> DbResult<bool> {
        let mut stmt = db.conn()
            .prepare("
SELECT * FROM users WHERE name = $1 AND pass = $2 AND pass_hash = $3;
")?;
        stmt.exists(&[&self.name(), &self.pass(), &self.pass_hash()])
            .map_err(|e| From::from(e))
    }

    pub fn save_to_db(&mut self, db: &Db) -> DbResult<i64> {
        let mut stmt = db.conn()
            .prepare("
INSERT INTO users VALUES (NULL, $1, $2, $3);
")?;
        self.id = stmt.insert(&[&self.name(), &self.pass(), &self.pass_hash()])?;
        Ok(self.id())
    }

    pub fn get_user(db: &Db, name: &str) -> DbResult<User> {
        db.conn().query_row_and_then("SELECT * FROM users WHERE name = ?", &[&name], |row| {
            Ok(User {
                id: row.get_checked(0)?,
                name: row.get_checked(1)?,
                pass: row.get_checked(2)?,
                pass_hash: row.get_checked(3)?,
            })
        })
    }

    pub fn get_users(db: &Db) -> DbResult<Vec<User>> {
        let mut users: Vec<User> = Vec::new();
        let mut stmt = db.conn()
            .prepare("
SELECT * FROM users;
")?;
        let mut rows = stmt.query(&[])?;
        while let Some(row) = rows.next() {
            let row = row?;
            users.push(User {
                id: row.get_checked(0)?,
                name: row.get_checked(1)?,
                pass: row.get_checked(2)?,
                pass_hash: row.get_checked(3)?,
            });
        }
        Ok(users)
    }

    pub fn delete_by_id(db: &Db, id: i64) -> DbResult<()> {
        let mut stmt = db.conn()
            .prepare("
DELETE FROM users WHERE ROWID = ?;
")?;
        stmt.execute(&[&id])?;
        Ok(())
    }

    pub fn update_by_id(db: &Db, id: i64, name: &str, pass: &str) -> DbResult<()> {
        let updated_user = User::new(name, pass);
        let mut stmt = db.conn()
            .prepare("
UPDATE users SET name = $1, pass = $2, pass_hash = $3 WHERE ROWID == $4;
")?;
        stmt.execute(&[&updated_user.name(), &updated_user.pass, &updated_user.pass_hash(), &id])?;
        Ok(())
    }
}

impl Default for User {
    fn default() -> Self {
        User::new("", "")
    }
}

impl UserVec for Vec<User> {
    fn is_auth(&self, name: &str, pass: &str) -> bool {
        if let Some(user) = self.iter().find(|user| user.name() == name) {
            md5::compute(pass.as_bytes())
                .into_iter()
                .map(|c| format!("{:x}", c))
                .collect::<String>() == user.pass_hash
        } else {
            false
        }
    }
}

impl Db {
    pub fn init_root(self) -> DbResult<Db> {
        let mut root_user: User = User::new("root", "toor");
        if !root_user.exists(&self)? {
            root_user.save_to_db(&self)?;
        }
        Ok(self)
    }
}

#[test]
fn new_user() {
    let new_user = User {
        id: 0,
        name: "Test".to_string(),
        pass: "qwerty".to_string(),
        pass_hash: "d8578edf8458ce6fbc5bb76a58c5ca4".to_string(),
    };
    assert_eq!(new_user, User::new("Test", "qwerty"));
}

#[test]
fn auth_user() {
    let users = vec![
        User::new("1", "1"),
        User::new("2", "2"),
        User::new("test", "pass"),
        User::new("right", "qwerty"),
    ];

    assert!(users.is_auth("right", "qwerty"));
    assert!(!users.is_auth("2", "qwerty"));
    assert!(!users.is_auth("wer", "qwef"));
}

#[test]
fn check_init_root() {
    let db = Db::new().and_then(|d| d.init_root()).unwrap();
    db.conn()
        .query_row("
SELECT * FROM users WHERE name = ?;
",
                   &[&"root"],
                   |row| {
                       assert_eq!(row.get::<i32, String>(1), "root");
                       assert_eq!(row.get::<i32, String>(2), "toor");
                       assert_eq!(row.get::<i32, String>(3),
                                  "7b24afc8bc80e548d66c4e7ff72171c5");
                   })
        .unwrap();
}
