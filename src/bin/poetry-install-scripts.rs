use anyhow::{anyhow, bail, Context, Result};
use directories::BaseDirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Output, Stdio};

#[derive(Deserialize, Debug)]
struct PyProject {
    tool: Tool,
}

#[derive(Deserialize, Debug)]
struct Tool {
    poetry: Poetry,
}

#[derive(Deserialize, Debug)]
struct Poetry {
    scripts: HashMap<String, String>,
}

fn main() -> Result<()> {
    let base_dirs = BaseDirs::new().ok_or_else(|| anyhow!("Could not get base dirs"))?;
    let scripts_path = base_dirs.home_dir().join(".local/bin");

    let Output { status, stdout, .. } = Command::new("poetry")
        .args(["env", "info", "-p"])
        .stdout(Stdio::piped())
        .spawn()
        .context("When spawning `poetry env info -p`")?
        .wait_with_output()
        .context("When running `poetry env info -p`")?;

    if !status.success() {
        bail!("`poetry env info -p` exited with non-zero status code or was interrupted");
    }

    let env_path = String::from_utf8(stdout).context("Non-utf8 output for `poetry env info -p`")?;
    let env_path = env_path.trim_end();

    let toml_contents = std::fs::read_to_string(Path::new("pyproject.toml"))
        .context("Could not read pyproject.toml")?;

    let script_template = r#"#!{env_path}/bin/python

from importlib import import_module
module = import_module("{module}")
module.{func}()
"#
    .replace("{env_path}", env_path);

    let py_project: PyProject =
        toml::from_str(&toml_contents).context("Could not parse pyproject.toml")?;

    for (name, script) in py_project.tool.poetry.scripts.iter() {
        let [module, func]: [&str; 2] = script
            .split(':')
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| anyhow!("Expected exactly one ':' in {script}"))?;
        generate_script(&scripts_path, &script_template, name, module, func)?;
    }
    Ok(())
}

fn generate_script(
    scripts_path: &Path,
    script_template: &str,
    name: &str,
    module: &str,
    func: &str,
) -> Result<()> {
    let to_write = script_template
        .replace("{module}", module)
        .replace("{func}", func);
    let script_path = scripts_path.join(name);
    std::fs::write(&script_path, to_write)
        .with_context(|| format!("Could not write {script_path:?}"))?;

    let script_file = File::open(&script_path)
        .with_context(|| format!("Could not open {script_path:?} to set permission",))?;
    let mut perms = script_file
        .metadata()
        .with_context(|| format!("Could not get filesystem metadata of {script_path:?}",))?
        .permissions();
    perms.set_mode(0o777);
    script_file
        .set_permissions(perms)
        .with_context(|| format!("Could not set executable permissions for {script_path:?}",))?;
    println!("Generated: {script_path:?}");
    Ok(())
}
