//! Document structure.
//!
//! TODO Write documentation.

use metadata::Metadata;
use permission::Permission;
use comment::Comment;
use user::User;
use category::Category;
use errors::DbResult;
use db::Db;

#[derive(Debug)]
pub struct Document {
    id: i64,
    name: String,
    metadata: Metadata,
    permission: Permission,
    data: Option<Vec<u8>>,
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
            permission: Permission::new(),
            data: None,
            comments: if let Some(comment) = comment {
                vec![comment]
            } else {
                vec![]
            },
            responsible: responsible_user,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    pub fn permission(&self) -> &Permission {
        &self.permission
    }

    pub fn data(&self) -> Option<Vec<u8>> {
        self.data.clone()
    }

    pub fn comments(&self) -> &Vec<Comment> {
        self.comments.as_ref()
    }

    pub fn comments_mut(&mut self) -> &mut Vec<Comment> {
        self.comments.as_mut()
    }

    pub fn responsible(&self) -> &User {
        &self.responsible
    }

    pub fn save_to_db(&mut self, db: &Db) -> DbResult<i64> {
        let metadata_id = self.metadata_mut().save_to_db(db)?;
        let mut stmt = db.conn()
            .prepare("
INSERT INTO docs VALUES (NULL, ?, ?, ?, ?, ?);
")?;
        self.id = stmt.insert(&[&self.name(),
                      &metadata_id,
                      &self.permission().get_int(),
                      &self.data(),
                      &self.responsible().id()])?;
        let doc_id = self.id();
        for comment in self.comments_mut() {
            comment.save_to_db(db, doc_id)?;
        }
        Ok(self.id())
    }
}
