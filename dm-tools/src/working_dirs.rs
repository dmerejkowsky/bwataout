use crate::list_helpers::*;
use crate::storage::EntriesCollection;

pub struct WorkingDirs {
    entries: Vec<String>,
}

impl WorkingDirs {
    pub fn new() -> WorkingDirs {
        WorkingDirs { entries: vec![] }
    }
}

impl EntriesCollection for WorkingDirs {
    fn name(&self) -> String { "cwd-history".to_string() }

    fn list(&self) -> &Vec<String> { &self.entries }

    fn kakoune_cmd(&self, entry: &str) -> String { format!("change-working-directory '{}'", entry) }

    fn add(&mut self, entry: &str) {
        self.entries = insert_last_and_dedup(&self.entries, entry);
    }

    fn clean(&mut self) {
        self.entries = remove_non_existing(&self.entries);
    }
}
