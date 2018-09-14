extern crate dirs;
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

#[derive(Debug)]
struct RitozError {
    description: String,
}

impl RitozError {
    pub fn new(description: &str) -> RitozError {
        RitozError {
            description: String::from(description),
        }
    }
}

impl std::fmt::Display for RitozError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.description)
    }
}

impl From<std::io::Error> for RitozError {
    fn from(error: std::io::Error) -> RitozError {
        RitozError {
            description: error.to_string(),
        }
    }
}

struct Storage {
    pub db_path: PathBuf,
}

impl Storage {
    pub fn new(db_path: PathBuf) -> Result<Storage, RitozError> {
        let cloned_path = db_path.clone();
        let parent_path = cloned_path.parent();
        if parent_path.is_none() {
            return Err(RitozError::new("Could not get parent path"));
        }
        let parent_path = parent_path.unwrap();
        if !parent_path.exists() {
            fs::create_dir_all(parent_path)?;
        }
        Ok(Storage { db_path })
    }

    pub fn add(&mut self, entry: &str) -> Result<(), RitozError> {
        let mut entries = &mut self.read_db()?;
        insert_in_order_and_dedup(&mut entries, entry);
        &mut self.write_db(&entries);
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<String>, RitozError> {
        if !self.db_path.exists() {
            return Ok(vec![]);
        }
        let contents = fs::read_to_string(&self.db_path)?;
        let res = contents.lines().map(|x| String::from(x)).collect();
        Ok(res)
    }

    pub fn remove(&mut self, entry: &str) -> Result<(), RitozError> {
        let mut entries = &mut self.read_db()?;
        remove_from_list(&mut entries, entry);
        &mut self.write_db(&entries)?;
        Ok(())
    }

    pub fn clean(&mut self) -> Result<u32, RitozError> {
        let mut res = 0;
        let entries = &mut self.read_db()?;
        for entry in entries {
            let path = Path::new(entry);
            if !path.exists() {
                res += 1;
                &mut self.remove(entry);
            }
        }
        Ok(res)
    }

    fn read_db(&self) -> Result<Vec<String>, RitozError> {
        if !self.db_path.exists() {
            return Ok(vec![]);
        }
        let contents = fs::read_to_string(&self.db_path)?;
        Ok(contents.lines().map(|x| String::from(x)).collect())
    }

    fn write_db(&mut self, entries: &Vec<String>) -> Result<(), RitozError> {
        fs::write(&self.db_path, entries.join("\n"))?;
        Ok(())
    }
}

fn get_db_path() -> PathBuf {
    let home = dirs::home_dir();
    let home = home.expect("Could no get home directory");
    home.join(".local/share/zsh/cwd_history")
}

fn add(storage: &mut Storage, target: &str) -> Result<(), RitozError> {
    let path = Path::new(&target);
    if !path.exists() {
        return Err(RitozError::new(&format!("{} does not exist", target)));
    }
    let path = path.canonicalize()?;
    let as_str = path.to_str();
    if as_str.is_none() {
        return Err(RitozError::new(&format!(
            "Could not convert {:?} to a string",
            path
        )));
    }
    return storage.add(&as_str.unwrap());
}

fn clean(storage: &mut Storage) -> Result<(), RitozError> {
    let cleaned = storage.clean()?;
    if cleaned > 0 {
        println!("Cleaned {} entries", cleaned);
    } else {
        println!("Already clean");
    }
    Ok(())
}

fn edit(storage: &Storage) -> Result<(), RitozError> {
    let editor = env::var("EDITOR");
    if editor.is_err() {
        return Err(RitozError::new("EDITOR not set"));
    }
    let editor = editor.unwrap();
    let status = Command::new(editor)
        .args(&[storage.db_path.clone()])
        .status()
        .expect("Failed to execute process");
    if !status.success() {
        return Err(RitozError::new("Editing the db failed"));
    }
    Ok(())
}

fn list(storage: &Storage) -> Result<(), RitozError> {
    let entries = storage.list()?;
    for entry in entries {
        println!("{}", entry);
    }
    Ok(())
}

fn remove(storage: &mut Storage, target: &str) -> Result<(), RitozError> {
    storage.remove(&target)
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
    let storage = Storage::new(db_path);
    if storage.is_err() {
        eprintln!("{}", storage.err().unwrap());
        process::exit(1);
    }
    let mut storage = storage.unwrap();

    let outcome = match args[1].as_ref() {
        "add" => add(&mut storage, &target),
        "clean" => clean(&mut storage),
        "edit" => edit(&storage),
        "list" => list(&storage),
        "remove" => remove(&mut storage, &target),
        a @ _ => Err(RitozError::new(&format!("Unknown action: {}", a))),
    };

    if let Err(error) = outcome {
        eprintln!("{}", error);
        process::exit(1);
    }
    process::exit(0);
}

#[cfg(test)]
mod tests {
    extern crate tempdir;
    use self::tempdir::TempDir;
    use super::*;
    use std::env;

    fn storage_from_temp(path: &Path) -> Storage {
        let temp_path = path.join("cwd_history");
        Storage::new(temp_path).unwrap()
    }

    #[test]
    fn starts_empty() {
        let temp_dir = TempDir::new("test-cwd-history").expect("failed to create temp dir");
        let storage = storage_from_temp(&temp_dir.path());
        assert_eq!(storage.list().unwrap().len(), 0);
    }

    #[test]
    fn can_add_some_paths() {
        let temp_dir = TempDir::new("test-cwd-history").expect("failed to create temp dir");
        let mut storage = storage_from_temp(&temp_dir.path());
        storage.add("foo/bar/baz").unwrap();
        assert_eq!(storage.list().unwrap(), vec!["foo/bar/baz"]);
    }

    #[test]
    fn db_is_persistent() {
        let temp_dir = TempDir::new("test-cwd-history").expect("failed to create temp dir");
        let mut storage1 = storage_from_temp(&temp_dir.path());
        storage1.add("foo").unwrap();

        let storage2 = storage_from_temp(&temp_dir.path());
        assert_eq!(storage2.list().unwrap(), vec!["foo"]);
    }

    #[test]
    fn remove_duplicates() {
        let temp_dir = TempDir::new("test-cwd-history").expect("failed to create temp dir");
        let mut storage = storage_from_temp(&temp_dir.path());
        storage.add("bar").unwrap();
        storage.add("foo").unwrap();
        storage.add("baz").unwrap();
        storage.add("foo").unwrap();

        assert_eq!(storage.list().unwrap(), vec!["bar", "baz", "foo"]);
    }

    #[test]
    fn can_remove() {
        let temp_dir = TempDir::new("test-cwd-history").expect("failed to create temp dir");
        let mut storage = storage_from_temp(&temp_dir.path());
        storage.add("bar").unwrap();
        storage.add("foo").unwrap();

        storage.remove("foo").unwrap();
        assert_eq!(storage.list().unwrap(), vec!["bar"]);
    }

    #[test]
    fn can_clean() {
        let temp_dir = TempDir::new("test-cwd-history").expect("failed to create temp dir");
        let mut storage = storage_from_temp(&temp_dir.path());

        let current_path = env::current_dir().expect("Could not get current dir");
        let cwd_str = current_path.to_string_lossy();
        storage.add(&cwd_str).unwrap();
        storage.add("/no/such/file").unwrap();

        storage.clean().unwrap();

        assert_eq!(storage.list().unwrap(), vec![cwd_str]);
    }
}
