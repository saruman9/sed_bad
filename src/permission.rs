//! Permission implementation.
//!
//! TODO Write documentation.

#[derive(Debug)]
pub struct Permission {
    author: NaivePermission,
    responsible: NaivePermission,
    others: NaivePermission,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct NaivePermission {
    pub read: bool,
    pub write: bool,
    pub comment: bool,
}

impl Permission {
    pub fn new() -> Self {
        Permission {
            author: NaivePermission::new(true, true, true),
            responsible: NaivePermission::new(true, false, true),
            others: NaivePermission::new(false, false, false),
        }
    }

    pub fn author(&self) -> NaivePermission {
        self.author
    }

    pub fn responsible(&self) -> NaivePermission {
        self.responsible
    }

    pub fn others(&self) -> NaivePermission {
        self.others
    }

    pub fn get_int(&self) -> i64 {
        let author_perm = self.author.get_int() * 100;
        let responsible_perm = self.responsible.get_int() * 10;
        let others_perm = self.others.get_int();
        author_perm + responsible_perm + others_perm
    }

    pub fn from_int(perm: i64) -> Permission {
        let author_perm = perm / 100;
        let responsible_perm = (perm - author_perm * 100) / 10;
        let others_perm = perm - author_perm * 100 - responsible_perm * 10;
        Permission {
            author: NaivePermission::from_int(author_perm),
            responsible: NaivePermission::from_int(responsible_perm),
            others: NaivePermission::from_int(others_perm),
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

    pub fn from_int(perm: i64) -> NaivePermission {
        let read = perm & 4 == 4;
        let write = perm & 2 == 2;
        let comment = perm & 1 == 1;
        NaivePermission {
            read: read,
            write: write,
            comment: comment,
        }
    }
}

#[test]
fn naive_permission_to_from_int() {
    let new_perm = NaivePermission {
        read: false,
        write: false,
        comment: false,
    };
    let new_perm_7 = NaivePermission {
        read: true,
        write: true,
        comment: true,
    };
    let new_perm_6 = NaivePermission {
        read: true,
        write: true,
        comment: false,
    };
    let new_perm_5 = NaivePermission {
        read: true,
        write: false,
        comment: true,
    };
    let new_perm_4 = NaivePermission {
        read: true,
        write: false,
        comment: false,
    };
    let new_perm_3 = NaivePermission {
        read: false,
        write: true,
        comment: true,
    };
    let new_perm_2 = NaivePermission {
        read: false,
        write: true,
        comment: false,
    };
    let new_perm_1 = NaivePermission {
        read: false,
        write: false,
        comment: true,
    };
    assert_eq!(new_perm.get_int(), 0);
    assert_eq!(new_perm_1.get_int(), 1);
    assert_eq!(new_perm_2.get_int(), 2);
    assert_eq!(new_perm_3.get_int(), 3);
    assert_eq!(new_perm_4.get_int(), 4);
    assert_eq!(new_perm_5.get_int(), 5);
    assert_eq!(new_perm_6.get_int(), 6);
    assert_eq!(new_perm_7.get_int(), 7);
    assert_eq!(new_perm, NaivePermission::from_int(0));
    assert_eq!(new_perm_1, NaivePermission::from_int(1));
    assert_eq!(new_perm_2, NaivePermission::from_int(2));
    assert_eq!(new_perm_3, NaivePermission::from_int(3));
    assert_eq!(new_perm_4, NaivePermission::from_int(4));
    assert_eq!(new_perm_5, NaivePermission::from_int(5));
    assert_eq!(new_perm_6, NaivePermission::from_int(6));
    assert_eq!(new_perm_7, NaivePermission::from_int(7));
}
