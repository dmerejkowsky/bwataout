fn read_db(path: &std::path::PathBuf) -> Vec<String> {
    if !path.exists() {
        return vec![];
    }
    let contents = std::fs::read_to_string(path).expect("Could not read db");
    contents.lines().map(|x| x.to_string()).collect()
}

fn write_db(path: &std::path::PathBuf, entries: &[String]) {
    let parent_path = path.parent().unwrap();
    if !parent_path.exists() {
        std::fs::create_dir_all(parent_path).expect("could not create db parent path");
    }
    std::fs::write(path, entries.join("\n") + "\n").expect("Could not write db")
}

pub trait EntriesCollection {
    fn name(&self) -> &'static str;
    fn add(&mut self, entry: &str);
    fn add_all(&mut self, entries: Vec<String>);
    fn clean(&mut self);
    fn list(&self) -> &[String];
    fn remove(&mut self, entry: &str);
    fn init_kakoune(&self);
}

pub struct Storage {
    db_path: std::path::PathBuf,
    entries_collection: Box<dyn EntriesCollection>,
}

impl Storage {
    pub fn new(
        mut entries_collection: Box<dyn EntriesCollection>,
        path: &std::path::PathBuf,
    ) -> Storage {
        let db_path = path.join(entries_collection.name());
        let entries = read_db(&db_path);
        entries_collection.add_all(entries);
        Storage {
            db_path,
            entries_collection,
        }
    }

    pub fn db_path(&self) -> std::path::PathBuf {
        self.db_path.to_path_buf()
    }

    pub fn list(&self) -> &[String] {
        &self.entries_collection.list()
    }

    pub fn add(&mut self, entry: &str) {
        self.entries_collection.add(&entry);
        write_db(&self.db_path, &self.list())
    }

    pub fn remove(&mut self, entry: &str) {
        self.entries_collection.remove(&entry);
        write_db(&self.db_path, &self.list())
    }

    pub fn clean(&mut self) {
        self.entries_collection.clean();
        write_db(&self.db_path, &self.list())
    }

    pub fn init_kakoune(&self) {
        self.entries_collection.init_kakoune()
    }
}
