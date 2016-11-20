//! Document structure.
//!
//! TODO Write documentation.

use metadata::Metadata;
use permission::Permission;
use comment::Comment;
use user::User;
use category::Category;

#[derive(Debug)]
pub struct Document {
    id: i64,
    name: String,
    metadata: Metadata,
    access: Permission,
    data: Option<String>,
    comments: Vec<Comment>,
    responsible: User, // TODO Reference?
}

impl Document {
    pub fn new(name: String,
               author: &User,
               category: Category,
               responsible_user: User,
               expired_date: (u32, u32, u32),
               comment: Option<Comment>)
               -> Self {
        Document {
            id: 0,
            name: name.to_string(),
            metadata: Metadata::new(&author, category, expired_date),
            access: Permission::new(),
            data: None,
            comments: if let Some(comment) = comment {
                vec![comment]
            } else {
                vec![]
            },
            responsible: responsible_user,
        }
    }
}
