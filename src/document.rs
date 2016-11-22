//! Document structure.
//!
//! TODO Write documentation.

use chrono::UTC;

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

    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
        self.metadata_mut().set_m_time(UTC::now());
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn metadata_mut(&mut self) -> &mut Metadata {
        &mut self.metadata
    }

    pub fn permission(&self) -> Permission {
        self.permission
    }

    pub fn set_permission(&mut self, permission: Permission) {
        self.permission = permission;
        self.metadata_mut().set_m_time(UTC::now());
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

    pub fn add_comment(&mut self, db: &Db, mut comment: Comment) -> DbResult<i64> {
        self.metadata_mut().set_m_time(UTC::now());
        self.metadata_mut().update(db)?;
        comment.save_to_db(db, self.id)
    }

    pub fn responsible(&self) -> &User {
        &self.responsible
    }

    pub fn set_responsible(&mut self, responsible: User) {
        self.responsible = responsible;
        self.metadata_mut().set_m_time(UTC::now());
    }

    pub fn update(&self, db: &Db) -> DbResult<i32> {
        db.conn()
            .execute("UPDATE docs SET name = ?, permission = ?, responsible = ? WHERE id = ?;",
                     &[&self.name(),
                       &self.permission().get_int(),
                       &self.responsible().id(),
                       &self.id()])
            .map_err(From::from)
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

    pub fn get_docs(db: &Db) -> DbResult<Vec<Document>> {
        let mut docs: Vec<Document> = Vec::new();
        let mut stmt = db.conn()
            .prepare("
SELECT * FROM docs;
")?;
        let mut rows = stmt.query(&[])?;
        while let Some(row) = rows.next() {
            let row = row?;
            docs.push(Document {
                id: row.get_checked(0)?,
                name: row.get_checked(1)?,
                metadata: Metadata::get_by_id(db, row.get_checked(2)?)?,
                permission: Permission::from_int(row.get_checked(3)?),
                data: row.get_checked(4)?,
                comments: Comment::get_by_doc_id(db, row.get_checked(0)?)?,
                responsible: User::get_by_id(db, row.get_checked(5)?)?,
            });
        }
        Ok(docs)
    }

    pub fn get_by_id(db: &Db, doc_id: i64) -> DbResult<Document> {
        db.conn().query_row_and_then("SELECT * FROM docs WHERE id = ?", &[&doc_id], |row| {
            Ok(Document {
                id: row.get_checked(0)?,
                name: row.get_checked(1)?,
                metadata: Metadata::get_by_id(db, row.get_checked(2)?)?,
                permission: Permission::from_int(row.get_checked(3)?),
                data: row.get_checked(4)?,
                comments: Comment::get_by_doc_id(db, row.get_checked(0)?)?,
                responsible: User::get_by_id(db, row.get_checked(5)?)?,
            })
        })
    }
}
