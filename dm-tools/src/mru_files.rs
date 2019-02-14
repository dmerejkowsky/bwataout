use crate::list_helpers::*;
use crate::storage::EntriesCollection;

pub struct MruFiles{
    entries: Vec<String>,
}

impl MruFiles{
    pub fn new() -> MruFiles
   {
        MruFiles
       { entries: vec![] }
    }
}

impl EntriesCollection for MruFiles{
    fn name(&self) -> String { "mru-files".to_string() }

    fn list(&self) -> &Vec<String> { &self.entries }

    fn kakoune_cmd(&self, entry: &str) -> String { format!("edit -existing '{}'", entry) }

    fn add(&mut self, entry: &str) {
        self.entries = insert_last_and_dedup(&self.entries, entry);
    }

    fn clean(&mut self) {
        self.entries = remove_non_existing(&self.entries);
    }
}
