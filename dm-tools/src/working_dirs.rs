use crate::list_helpers::*;
use crate::storage::EntriesCollection;
use std::path::Path;

pub struct WorkingDirs {
    entries: Vec<String>,
}

impl WorkingDirs {
    pub fn new() -> WorkingDirs {
        WorkingDirs { entries: vec![] }
    }
}

impl EntriesCollection for WorkingDirs {
    fn name(&self) -> String {
        "cwd-history".to_string()
    }

    fn list(&self) -> &Vec<String> {
        &self.entries
    }

    fn kakoune_cmd(&self, entry: &str) -> String {
        format!("change-directory '{}'", entry)
    }

    fn add_all(&mut self, entries: &Vec<String>) {
        self.entries = entries.to_vec();
    }

    fn add(&mut self, entry: &str) {
        let path = Path::new(entry);
        if !path.exists() {
            return;
        }
        let abs_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir().unwrap().join(path)
        };
        let clean_path = abs_path
            .canonicalize()
            .unwrap_or_else(|_| panic!("Could not canonicalize {}", entry));
        let entry = clean_path
            .to_str()
            .unwrap_or_else(|| panic!("Could not convert {} to string", clean_path.display()));
        self.entries = insert_last_and_dedup(&self.entries, entry);
    }

    fn clean(&mut self) {
        self.entries = remove_non_existing(&self.entries);
    }
}
