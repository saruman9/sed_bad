//! Document structure.
//!
//! TODO Write documentation.

use metadata::Metadata;
use permission::Permission;
use comment::Comment;
use user::User;
use category::Category;

struct Document {
    name: String,
    metadata: Metadata,
    access: Permission,
    data: String,
    comments: Vec<Comment>,
    responsible: Option<User>, // TODO Reference?
    changelog: Vec<Change>,
}

// TODO.
struct Change {}

impl Document {
    pub fn new(name: &str, author: User, category: Category, data: &str) -> Self {
        Document {
            name: name.to_string(),
            metadata: Metadata::new(author, category),
            access: Permission::new(),
            data: data.to_string(),
            comments: Vec::new(),
            responsible: None,
            changelog: Vec::new(),
        }
    }
}
