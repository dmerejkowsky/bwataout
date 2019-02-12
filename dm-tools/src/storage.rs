pub struct Storage {
    db_path: std::path::PathBuf,
    entries: Vec<String>,
}

fn remove_from_list(list: &mut Vec<String>, elem: &str) {
    let pos = list.iter().position(|x| *x == elem);
    if let Some(index) = pos {
        list.remove(index);
    }
}

fn insert_in_order_and_dedup(list: &mut Vec<String>, elem: &str) {
    remove_from_list(list, elem);
    list.push(elem.to_string());
}

fn read_db(path: &std::path::PathBuf) -> Vec<String> {
    if ! path.exists() {
        return vec![];
    }
    let contents = std::fs::read_to_string(path).expect("Could not read db");
    contents.lines().map(|x| x.to_string()).collect()
}

fn write_db(path: &std::path::PathBuf, entries: &Vec<String>) {
    let parent_path = path.parent().unwrap();
    if !parent_path.exists() {
        std::fs::create_dir_all(parent_path).expect("could not create db parent path");
    }
    std::fs::write(path, entries.join("\n")).expect("Could not write db")
}

impl Storage {
    pub fn new(name: &str, path: &std::path::PathBuf) -> Storage {
        let db_path = path.join(name);
        let entries = read_db(&db_path);
        Storage {
            db_path,
            entries,
        }
    }

    pub fn db_path(&self) -> std::path::PathBuf {
        self.db_path.to_path_buf()
    }

    pub fn list(self) -> Vec<String> {
        self.entries
    }

    pub fn add(&mut self, entry: &str) {
        insert_in_order_and_dedup(&mut self.entries, entry);
        write_db(&self.db_path, &self.entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;


    #[test]
    fn test_storage_starts_empty() {
        let temp_path = TempDir::new("test-dm-tools").unwrap().path().to_path_buf();
        let storage = Storage::new("dummy", &temp_path);
        assert_eq!(storage.list().len(), 0)
    }

    #[test]
    fn test_storage_can_add_entry() {
        let temp_path = TempDir::new("test-dm-tools").unwrap().path().to_path_buf();
        let mut storage = Storage::new("dummy", &temp_path);
        storage.add("foo");
        assert_eq!(storage.list(), vec!["foo"]);
    }

    #[test]
    fn test_storage_dedups_and_preserve_order() {
        let temp_path = TempDir::new("test-dm-tools").unwrap().path().to_path_buf();
        let mut storage = Storage::new("dummy", &temp_path);
        storage.add("foo");
        storage.add("bar");
        storage.add("foo");
        assert_eq!(storage.list(), vec!["bar", "foo"]);
    }

    #[test]
    fn test_storage_is_persistent() {
        let temp_path = TempDir::new("test-dm-tools").unwrap().path().to_path_buf();
        let mut storage1 = Storage::new("dummy", &temp_path);
        storage1.add("foo");
        storage1.add("bar");

        let storage2 = Storage::new("dummy", &temp_path);
        assert_eq!(storage2.list(), vec!["foo", "bar"]);
    }
}
