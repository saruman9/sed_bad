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

    pub fn get_int(&self) -> i64 {
        let author_perm = self.author.get_int() * 100;
        let responsible_perm = self.responsible.get_int() * 10;
        let others_perm = self.others.get_int();
        author_perm + responsible_perm + others_perm
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

    pub fn get_int(&self) -> i64 {
        let mut res = 0;
        if self.read {
            res += 4;
        }
        if self.write {
            res += 2;
        }
        if self.comment {
            res += 1;
        }
        res
    }
}
