use crate::storage::Storage;
use app_dirs::{AppDataType, AppInfo};

const APP_INFO: AppInfo = AppInfo {
    name: "dm-tools",
    author: "Dimitri Merejkowsky",
};

pub struct App {
    storage: Storage,
}

impl App {
    pub fn new(name: &str) -> App {
        let app_dir = app_dirs::app_dir(AppDataType::UserData, &APP_INFO, "")
            .expect("could not create app dir");
        let storage = Storage::new(name, &app_dir);
        App { storage }
    }

    pub fn add(&mut self, entry: &str) {
        self.storage.add(entry)
    }

    pub fn edit(&self) {
        let editor = std::env::var("EDITOR").expect("EDITOR not set");
        let status = std::process::Command::new(editor)
            .args(&[self.storage.db_path()])
            .status()
            .expect("Failed to execute editor");
        if !status.success() {
            panic!("Editing the db failed");
        }
    }

    pub fn print_self(self) {
        let entries = self.storage.list();
        println!("{}", entries.join("\n"))
    }

    pub fn for_kakoune(self, command: &str) {
        print!("{}", "menu ");
        for entry in self.storage.list().iter().rev().take(10) {
            print!("{entry} \"{cmd} {entry}\" ", cmd = command, entry = entry)
        }
    }
}
