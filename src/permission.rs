//! Permission implementation.
//!
//! TODO Write documentation.

#[derive(Debug)]
pub struct Permission {
    author: NaivePermission,
    responsible: NaivePermission,
    others: NaivePermission,
}

#[derive(Debug)]
struct NaivePermission {
    read: bool,
    write: bool,
    comment: bool,
}

impl Permission {
    pub fn new() -> Self {
        Permission {
            author: NaivePermission::new(true, true, true),
            responsible: NaivePermission::new(true, false, true),
            others: NaivePermission::new(false, false, false),
        }
    }
}

impl NaivePermission {
    pub fn new(read: bool, write: bool, comment: bool) -> Self {
        NaivePermission {
            read: read,
            write: write,
            comment: comment,
        }
    }
}
