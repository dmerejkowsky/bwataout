// Convert a lock with lots of info to something more readable
//
// For instance:
//
//    [[package]]
//    name = "foo"
//    version = "1.4"
//
// Becomes
//
//    foo@1.4
//
// Note that this works both with poetry.lock and Cargo.lock - both use toml
// with the same syntax :)
//
// For this to work, you need to register the 'lockdiff' in .gitattributes:
//
//    Cargo.lock diff=lockdiff
//
// And in git config:
//
//    [diff "lockdiff"]
//    textconv = lock-diff

use std::fmt::Display;

use anyhow::{bail, Context, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Lock {
    package: Vec<Package>,
}

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    version: String,
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.name, self.version)
    }
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        bail!("Expected exactly one arg");
    }

    let lock_path = &args[1];
    let lock_contents = std::fs::read_to_string(&lock_path).context("Could not read lock file")?;
    let lock: Lock = toml::from_str(&lock_contents).context("Could not parse lock")?;
    for package in lock.package {
        println!("{package}");
    }

    Ok(())
}
