//! User implementation.
//!
//! TODO Write documentation.

use md5;

use errors::{AuthResult, DbResult};
use db::Db;

trait UserVec {
    fn is_auth(&self, name: &str, pass: &str) -> bool;
}

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    id: u32,
    name: String,
    pass: String,
    pass_hash: [u8; 16],
}

impl User {
    pub fn new(name: &str, pass: &str) -> Self {
        // TODO Change ID of users.
        User {
            id: 0,
            name: name.to_string(),
            pass: pass.to_string(),
            pass_hash: md5::compute(pass.as_bytes()),
        }
    }

    pub fn set(&mut self, user: User) {
        self.name = user.name;
        self.pass = user.pass;
        self.pass_hash = user.pass_hash;
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

impl Default for User {
    fn default() -> Self {
        User::new(&String::default(), &String::default())
    }
}

impl UserVec for Vec<User> {
    fn is_auth(&self, name: &str, pass: &str) -> bool {
        if let Some(user) = self.iter().find(|user| user.name() == name) {
            md5::compute(pass.as_bytes()) == user.pass_hash
        } else {
            false
        }
    }
}

impl Db {
    pub fn init_root(self) -> DbResult<Db> {
        self.conn()
            .execute("
INSERT OR REPLACE INTO users VALUES (
    'root',
    'toor',
    '7b24afc8bc80e548d66c4e7ff72171c5'
);",
                     &[])?;
        Ok(self)
    }
}

#[test]
fn new_user() {
    let new_user = User {
        id: 0,
        name: "Test".to_string(),
        pass: "qwerty".to_string(),
        pass_hash: [0xd8, 0x57, 0x8e, 0xdf, 0x84, 0x58, 0xce, 0x06, 0xfb, 0xc5, 0xbb, 0x76, 0xa5,
                    0x8c, 0x5c, 0xa4],
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
    db.conn().query_row("
SELECT * FROM users WHERE name = ?;
",
                        &[&"root"],
                        |row| {
        assert_eq!(row.get::<i32, String>(0), "root");
        assert_eq!(row.get::<i32, String>(1), "toor");
        assert_eq!(row.get::<i32, String>(2),
                   "7b24afc8bc80e548d66c4e7ff72171c5");
    }).unwrap();
}
