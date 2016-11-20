//! Category implementation.
//!
//! TODO Write documentation.

use db::Db;
use errors::DbResult;

#[derive(Debug)]
pub struct Category {
    id: i64,
    name: String,
}

impl Category {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Category {
            id: 0,
            name: name.into(),
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn save_to_db(&mut self, db: &Db) -> DbResult<i64> {
        let mut stmt = db.conn()
            .prepare("
INSERT INTO categories VALUES (NULL, ?);
")?;
        self.id = stmt.insert(&[&self.name()])?;
        Ok(self.id())
    }

    pub fn get_category(db: &Db, name: &str) -> DbResult<Category> {
        db.conn().query_row_and_then("SELECT * FROM categories WHERE name = ?;", &[&name], |row| {
            Ok(Category {
                id: row.get_checked(0)?,
                name: row.get_checked(1)?,
            })
        })
    }

    pub fn get_categories(db: &Db) -> DbResult<Vec<Category>> {
        let mut categories: Vec<Category> = Vec::new();
        let mut stmt = db.conn()
            .prepare("
SELECT * FROM categories;
")?;
        let mut rows = stmt.query(&[])?;
        while let Some(row) = rows.next() {
            let row = row?;
            categories.push(Category {
                id: row.get_checked(0)?,
                name: row.get_checked(1)?,
            });
        }
        Ok(categories)
    }

    pub fn exists(&self, db: &Db) -> DbResult<bool> {
        let mut stmt = db.conn()
            .prepare("
SELECT * FROM categories WHERE name = ?;
")?;
        stmt.exists(&[&self.name()])
            .map_err(|e| From::from(e))
    }
}
