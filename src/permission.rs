//! Permission implementation.
//!
//! TODO Write documentation.

pub struct Permission {
    author: NaivePermission,
    responsible: NaivePermission,
    others: NaivePermission,
}

struct NaivePermission {
    read: bool,
    write: bool,
    comment: bool,
}

impl Permission {
    pub fn new() -> Self {
        Permission {
            author: NaivePermission::new(true, true, true),
            responsible: NaivePermission::new(false, false, false),
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
