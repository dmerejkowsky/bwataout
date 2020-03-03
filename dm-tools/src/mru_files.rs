use crate::list_helpers::*;
use crate::storage::EntriesCollection;

pub struct MruFiles {
    entries: Vec<String>,
}

impl MruFiles {
    pub fn new() -> MruFiles {
        MruFiles { entries: vec![] }
    }
}

const BLACK_LISTED_NAMES: [&str; 2] = [".git/COMMIT_EDITMSG", "git-rebase-todo"];

fn is_blacklisted(entry: &str) -> bool {
    for name in &BLACK_LISTED_NAMES {
        if entry.ends_with(name) {
            return true;
        }
    }
    false
}

impl EntriesCollection for MruFiles {
    fn name(&self) -> String {
        "mru-files".to_string()
    }

    fn list(&self) -> &[String] {
        &self.entries
    }

    fn kakoune_cmd(&self, entry: &str) -> String {
        format!("edit -existing '{}'", entry)
    }

    fn add(&mut self, entry: &str) {
        if is_blacklisted(entry) {
            return;
        }
        self.entries = insert_last_and_dedup(&self.entries, entry);
    }

    fn add_all(&mut self, entries: Vec<String>) {
        self.entries = entries;
    }

    fn remove(&mut self, entry: &str) {
        self.entries.retain(|x| x != entry);
    }

    fn clean(&mut self) {
        self.entries = remove_non_existing(&self.entries);
        self.entries.retain(|x| !is_blacklisted(x));
    }
}
