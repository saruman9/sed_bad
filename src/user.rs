//! User implementation.
//!
//! TODO Write documentation.

use md5;

use errors::AuthResult;


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

    pub fn name(&self) -> &str {
        self.name.as_ref()
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
