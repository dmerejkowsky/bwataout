use crate::list_helpers::*;
use crate::storage::EntriesCollection;

pub struct Commands {
    entries: Vec<String>,
}

impl Commands{
    pub fn new() -> Commands {
        Commands { entries: vec![] }
    }
}

impl EntriesCollection for Commands {
    fn name(&self) -> String { "commands-history".to_string() }

    fn kakoune_cmd(&self, entry: &str) -> String { format!("%sh{{ {} }}", entry) }

    fn add(&mut self, entry: &str) {
        self.entries = insert_last_and_dedup(&self.entries, entry);
    }

    fn list(&self) -> &Vec<String> {
        &self.entries
    }

    fn clean(&mut self) {
        // no-op
    }
}
