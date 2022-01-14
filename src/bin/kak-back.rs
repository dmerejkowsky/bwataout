use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use directories::ProjectDirs;

// À une vache près, c'est pas une scrience exacte
const ONE_MONTH: std::time::Duration = std::time::Duration::from_secs(60 * 60 * 24 * 30);

#[derive(Debug, Parser)]
#[clap(
    name = "kak-back",
    about = "backup and restore files edited with Kakoune"
)]
struct Command {
    #[clap(subcommand)]
    pub sub_cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[clap(name = "backup", about = "backup the given file")]
    Backup {
        #[clap(help = "path to back up")]
        path: PathBuf,
    },

    #[clap(name = "clean", about = "clean old backups")]
    Clean {
        #[clap(long = "dry-run", help = "don't actually clean the files")]
        dry_run: bool,
    },

    #[clap(name = "list", about = "list known backups")]
    List {},

    #[clap(name = "restore", about = "try and restore a backup")]
    Restore { dest: PathBuf },
}

struct BackupFile {
    relative_path: String,
    mtime: std::time::SystemTime,
}

impl BackupFile {
    fn full_path(&self) -> String {
        self.relative_path.replace("%", "/")
    }
}

struct BackupStore {
    path: PathBuf,
    files: Vec<BackupFile>,
}

impl BackupStore {
    fn new(path: &Path) -> Result<Self> {
        let mut files = vec![];
        let dir = std::fs::read_dir(&path).with_context(|| "Could not read backups dir")?;
        for entry in dir {
            let entry = entry?;
            let metadata = entry
                .metadata()
                .with_context(|| format!("Could not get file metadata for {entry:?}"))?;
            let file_type = metadata.file_type();
            if file_type.is_file() {
                let mtime = metadata
                    .modified()
                    .with_context(|| format!("Could not get mtime for {entry:?}"))?;
                let file_name = entry.file_name();
                let relative_path = file_name
                    .into_string()
                    .map_err(|_| anyhow!("Invalid file name for {entry:?}"))?;
                let backup_file = BackupFile {
                    relative_path,
                    mtime,
                };
                files.push(backup_file);
            }
        }
        files.sort_by_key(|x| x.mtime);
        Ok(BackupStore {
            files,
            path: path.to_path_buf(),
        })
    }

    fn full_path_to_backup_path(&self, full_path: &Path) -> Result<PathBuf> {
        let full_path_name = full_path
            .to_str()
            .ok_or_else(|| anyhow!("non-unicode full path"))?;
        let backup_name = &full_path_name.replace("/", "%");
        Ok(self.path.join(backup_name))
    }

    fn backup(&self, path: &Path) -> Result<()> {
        let full_path = path
            .canonicalize()
            .with_context(|| format!("Could not canoncalize: {path:?}"))?;
        let backup_path = self.full_path_to_backup_path(&full_path)?;
        std::fs::copy(&full_path, &backup_path)
            .with_context(|| format!("Could not copy from\n{full_path:?}\nto\n{backup_path:?}",))?;
        Ok(())
    }

    fn restore(&self, dest: &Path) -> Result<()> {
        // Note: canoncalize returns Err() if dest does not exist because reasons,
        // so if `dest` does not exist, create an empty file beforehand.
        // Worst case scenario: we get an empty file instead of nothing
        if !dest.exists() {
            std::fs::write(dest, "")
                .with_context(|| "Could not create restored file".to_string())?;
        }
        let full_path = dest
            .canonicalize()
            .with_context(|| format!("Could not canoncalize: {dest:?}"))?;
        let backup_path = self.full_path_to_backup_path(&full_path)?;
        std::fs::copy(&backup_path, dest)
            .with_context(|| format!("Could not copy from\n{backup_path:?}\nto\n{dest:?}",))?;
        Ok(())
    }

    fn clean(&self, dry_run: bool) -> Result<()> {
        let total = self.files.len();
        let mut cleaned = 0;
        let now = std::time::SystemTime::now();
        for file in &self.files {
            let relative_path = &file.relative_path;
            let duration = now
                .duration_since(file.mtime)
                .with_context(|| format!("mtime in the future for {relative_path}"))?;
            if duration > ONE_MONTH {
                cleaned += 1;
                if !dry_run {
                    let full_path = &self.path.join(&file.relative_path);
                    std::fs::remove_file(&full_path)
                        .with_context(|| format!("Could not remove {full_path:?}"))?;
                }
            }
        }
        if dry_run {
            println!("Would have cleaned {cleaned} entries over {total}");
        } else {
            println!("Cleaned {cleaned} entries over {total}");
        }
        Ok(())
    }

    fn list(&self) -> Result<()> {
        for file in &self.files {
            let full_path = file.full_path();
            println!("{full_path}");
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Command::parse();
    let project_dirs = ProjectDirs::from("info", "dmerej", "kak")
        .ok_or_else(|| anyhow!("Could not get project dirs"))?;
    let backups_dir = project_dirs.data_dir().join("backups");
    std::fs::create_dir_all(&backups_dir)
        .map_err(|e| anyhow!("Could not create backups dir: {e}"))?;
    let backup_store = BackupStore::new(&backups_dir)?;

    match args.sub_cmd {
        SubCommand::Backup { path } => backup_store.backup(&path),
        SubCommand::List {} => backup_store.list(),
        SubCommand::Clean { dry_run } => backup_store.clean(dry_run),
        SubCommand::Restore { dest } => backup_store.restore(&dest),
    }
}
