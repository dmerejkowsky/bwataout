use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::{io::Write, path::Path};

pub trait Filter {
    fn parse(&self, input: &str) -> Option<String>;
    fn should_clean(&self, entry: &str) -> bool;
}

pub struct DB<T>
where
    T: Filter,
{
    connection: rusqlite::Connection,
    filter: T,
}

impl<T> DB<T>
where
    T: Filter,
{
    pub fn new(path: &Path, filter: T) -> Result<Self> {
        let connection = Connection::open(path)?;
        let res = DB { connection, filter };
        res.migrate()?;
        Ok(res)
    }

    fn migrate(&self) -> Result<()> {
        self.connection
            .execute(
                "
            CREATE TABLE IF NOT EXISTS entries (
              entry TEXT UNIQUE,
              date TEXT DEFAULT CURRENT_TIMESTAMP
            )
       ",
                params![],
            )
            .with_context(|| "Could not migrate db")?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<String>> {
        self.execute_list_query("SELECT entry FROM entries ORDER BY date ASC")
    }

    pub fn list_reversed(&self) -> Result<Vec<String>> {
        self.execute_list_query("SELECT entry FROM entries ORDER BY date DESC")
    }

    fn execute_list_query(&self, sql: &'static str) -> Result<Vec<String>> {
        let mut statement = self.connection.prepare(sql)?;

        let mut rows = statement.query([])?;
        let mut res = vec![];
        while let Some(row) = rows.next()? {
            res.push(row.get(0)?);
        }
        Ok(res)
    }

    pub fn clean(&self, max: Option<isize>) -> Result<()> {
        let mut select_statement = match max {
            Some(_) => self
                .connection
                .prepare("SELECT entry FROM entries ORDER BY date DESC LIMIT ?")?,
            None => self
                .connection
                .prepare("SELECT entry FROM entries ORDER BY date DESC")?,
        };
        let mut select_query = match max {
            Some(m) => select_statement.query(params![m]),
            None => select_statement.query([]),
        }?;

        let count = match max {
            Some(m) => m,
            None => {
                let mut statement = self.connection.prepare("SELECT COUNT(*) FROM ENTRIES")?;
                let mut query = statement.query([])?;
                let res = query.next()?;
                res.unwrap().get(0)?
            }
        };

        let mut to_clean = vec![];
        while let Some(row) = select_query.next()? {
            let value: String = row.get(0)?;
            if self.filter.should_clean(&value) {
                to_clean.push(value);
            }
        }

        let n = to_clean.len();
        let mut i = 0;
        for name in to_clean {
            i += 1;
            print!("{}/{}\r", i, n);
            let _ = std::io::stdout().flush();
            let mut delete_statement = self
                .connection
                .prepare("DELETE FROM ENTRIES WHERE entry = ?")?;
            delete_statement.execute([name])?;
        }

        println!("Cleaned {} entries over {}", n, count);
        Ok(())
    }

    pub fn add(&mut self, input: &str) -> Result<()> {
        let parsed = self.filter.parse(input);
        if let Some(value) = parsed {
            self.connection.execute(
                "
                INSERT INTO entries(entry) VALUES(?)
                ON CONFLICT(entry) DO
                  UPDATE SET date=datetime('now')
              ",
                params![value],
            )?;
        }
        Ok(())
    }

    pub fn remove(&mut self, value: &str) -> Result<()> {
        self.connection
            .execute("DELETE FROM entries WHERE entry = ?", params![value])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyFilter {}

    impl Filter for DummyFilter {
        fn parse(&self, input: &str) -> Option<String> {
            Some(input.to_string())
        }
        fn should_clean(&self, _entry: &str) -> bool {
            false
        }
    }

    impl DB<DummyFilter> {
        fn new_for_tests() -> Self {
            let connection = Connection::open_in_memory().unwrap();
            let filter = DummyFilter {};
            let res = DB { connection, filter };
            res.migrate().unwrap();
            res
        }
    }

    #[test]
    fn test_when_empty() -> Result<()> {
        let db = DB::new_for_tests();
        let out = db.list()?;
        assert!(out.is_empty());
        Ok(())
    }

    #[test]
    fn test_list_sorted_by_time() -> Result<()> {
        let mut db = DB::new_for_tests();
        db.add("path2")?;
        db.add("path1")?;
        let out = db.list()?;
        assert_eq!(out.len(), 2);
        Ok(())
    }

    #[test]
    fn test_dedup() -> Result<()> {
        let mut db = DB::new_for_tests();
        db.add("path1")?;
        db.add("path2")?;
        db.add("path1")?;
        let out = db.list()?;
        assert_eq!(out.len(), 2);
        Ok(())
    }
}
