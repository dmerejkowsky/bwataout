use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::process::Command;

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

struct Storage {
    pub db_path: PathBuf,
}

impl Storage {
    pub fn new(db_path: PathBuf) -> Storage {
        let cloned_path = db_path.clone();
        let parent_path = cloned_path.parent().expect("Could not get parent path");
        if !parent_path.exists() {
            fs::create_dir_all(parent_path).expect("Could not create database directory");
        }
        Storage { db_path }
    }

    pub fn add(&mut self, entry: &str) {
        let mut entries = &mut self.read_db();
        insert_in_order_and_dedup(&mut entries, entry);
        &mut self.write_db(&entries);
    }

    pub fn list(&self) -> Vec<String> {
        if !self.db_path.exists() {
            return vec![];
        }
        let contents = fs::read_to_string(&self.db_path).expect(&format!(
            "Could not read form {}",
            self.db_path.to_string_lossy()
        ));
        contents.lines().map(|x| String::from(x)).collect()
    }

    pub fn remove(&mut self, entry: &str) {
        let mut entries = &mut self.read_db();
        remove_from_list(&mut entries, entry);
        &mut self.write_db(&entries);
    }

    pub fn clean(&mut self) -> u32 {
        let mut res = 0;
        let entries = &mut self.read_db();
        for entry in entries {
            let path = Path::new(entry);
            if !path.exists() {
                res += 1;
                &mut self.remove(entry);
            }
        }
        res
    }

    fn read_db(&self) -> Vec<String> {
        if !self.db_path.exists() {
            return vec![];
        }
        let contents = fs::read_to_string(&self.db_path).expect("Could not read from db");
        contents.lines().map(|x| String::from(x)).collect()
    }

    fn write_db(&mut self, entries: &Vec<String>) {
        fs::write(&self.db_path, entries.join("\n")).expect("Could not write to db");
    }
}

fn get_db_path() -> PathBuf {
    let home = std::env::home_dir();
    let home = home.expect("Could no get home directory");
    home.join(".local/share/zsh/cwd_history")
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cwd-history [add|list]");
        process::exit(1);
    }

    let mut target = String::new();
    let action = &args[1];
    if action == "add" || action == "remove" {
        if args.len() < 3 {
            eprintln!("Action {} requires one argument", action);
            process::exit(1);
        }
        target = args[2].clone();
    }

    let db_path = get_db_path();
    let mut storage = Storage::new(db_path);

    match args[1].as_ref() {
        "add" => {
            let path = Path::new(&target);
            if !path.exists() {
                eprintln!("{:?} does not exist", path);
                process::exit(1);
            }
            let path = path.canonicalize().unwrap();
            let as_str = path.to_str().expect("Could not convet ptah to string");
            storage.add(&as_str);
        }
        "clean" => {
            let cleaned = storage.clean();
            if cleaned > 0 {
                println!("Cleaned {} entries", cleaned);
            } else {
                println!("Already clean");
            }
        }
        "edit" => {
            let editor = env::var("EDITOR").unwrap();
            let status = Command::new(editor)
                .args(&[storage.db_path])
                .status()
                .expect("Failed to execute process");
            if !status.success() {
                eprintln!("Editing the db failed");
                process::exit(1);
            }
        }
        "list" => {
            for entry in storage.list() {
                println!("{}", entry);
            }
        }
        "remove" => {
            storage.remove(&target);
        }
        a @ _ => {
            eprintln!("Unknown action: {}", a);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate tempdir;
    use self::tempdir::TempDir;
    use super::*;
    use std::env;

    fn storage_from_temp(path: &Path) -> Storage {
        let temp_path = path.join("cwd_history");
        Storage::new(temp_path)
    }

    #[test]
    fn starts_empty() {
        let temp_dir = TempDir::new("test-cwd-history").expect("failed to create temp dir");
        let storage = storage_from_temp(&temp_dir.path());
        assert_eq!(storage.list().len(), 0);
    }

    #[test]
    fn can_add_some_paths() {
        let temp_dir = TempDir::new("test-cwd-history").expect("failed to create temp dir");
        let mut storage = storage_from_temp(&temp_dir.path());
        storage.add("foo/bar/baz");
        assert_eq!(storage.list(), vec!["foo/bar/baz"]);
    }

    #[test]
    fn db_is_persistent() {
        let temp_dir = TempDir::new("test-cwd-history").expect("failed to create temp dir");
        let mut storage1 = storage_from_temp(&temp_dir.path());
        storage1.add("foo");

        let storage2 = storage_from_temp(&temp_dir.path());
        assert_eq!(storage2.list(), vec!["foo"]);
    }

    #[test]
    fn remove_duplicates() {
        let temp_dir = TempDir::new("test-cwd-history").expect("failed to create temp dir");
        let mut storage = storage_from_temp(&temp_dir.path());
        storage.add("bar");
        storage.add("foo");
        storage.add("baz");
        storage.add("foo");

        assert_eq!(storage.list(), vec!["bar", "baz", "foo"]);
    }

    #[test]
    fn can_remove() {
        let temp_dir = TempDir::new("test-cwd-history").expect("failed to create temp dir");
        let mut storage = storage_from_temp(&temp_dir.path());
        storage.add("bar");
        storage.add("foo");

        storage.remove("foo");
        assert_eq!(storage.list(), vec!["bar"]);
    }

    #[test]
    fn can_clean() {
        let temp_dir = TempDir::new("test-cwd-history").expect("failed to create temp dir");
        let mut storage = storage_from_temp(&temp_dir.path());

        let current_path = env::current_dir().unwrap();
        let cwd_str = current_path.to_string_lossy();
        storage.add(&cwd_str);
        storage.add("/no/such/file");

        storage.clean();

        assert_eq!(storage.list(), vec![cwd_str]);
    }
}
