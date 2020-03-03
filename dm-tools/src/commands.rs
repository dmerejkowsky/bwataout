use crate::list_helpers::*;
use crate::storage::EntriesCollection;

pub struct Commands {
    entries: Vec<String>,
}

impl Commands {
    pub fn new() -> Commands {
        Commands { entries: vec![] }
    }
}

impl EntriesCollection for Commands {
    fn name(&self) -> String {
        "commands-history".to_string()
    }

    fn add(&mut self, entry: &str) {
        if entry.starts_with(' ') {
            return;
        }
        self.entries = insert_last_and_dedup(&self.entries, entry);
    }

    fn add_all(&mut self, entries: Vec<String>) {
        self.entries = entries;
    }

    fn list(&self) -> &[String] {
        &self.entries
    }

    fn clean(&mut self) {
        // no-op
    }

    fn remove(&mut self, entry: &str) {
        self.entries.retain(|x| x != entry);
    }

    fn init_kakoune(&self) {}
}
