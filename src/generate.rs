use anyhow::{anyhow, Context, Ok, Result};
use log::debug;
use std::{
    env::{self, VarError},
    path::{Path, PathBuf},
};

use crate::{
    consts::{ENV_NAME, NOT_UNICODE_ERR},
    shells::Shell,
};

#[derive(Debug)]
pub struct EnvironmentSettings {
    pub bin: String,
    pub path: String,
}

impl EnvironmentSettings {
    fn new(bin: String, path: String) -> Self {
        Self { bin, path }
    }
}

fn strip_bin_path(v: &str) -> Result<&str> {
    // Note: Only ParsePathError will be returned
    Path::new(v)
        .ancestors()
        .nth(2)
        .ok_or_else(|| anyhow!("Error while parsing path"))?
        .to_str()
        .ok_or_else(|| anyhow!(NOT_UNICODE_ERR))
}

fn _gen_env_settings(
    shell: &dyn Shell,
    envs: (String, String, PathBuf),
) -> Result<Option<EnvironmentSettings>> {
    let (bin, path, mut cwd) = envs;

    let env_separator = shell.env_separator();
    let env_separator_str = shell.env_separator_str();
    let mut has_node_modules = false;

    let bin_dir_buf = {
        debug!("Cwd: {:?}", cwd);
        cwd.push("node_modules");

        if cwd.as_path().is_dir() {
            has_node_modules = true;
        }

        cwd.push(".bin");
        cwd
    };

    let new_bin_dir = bin_dir_buf
        .to_str()
        .ok_or_else(|| anyhow!(NOT_UNICODE_ERR))
        .context("Failed to generate bin dir")?
        .to_string();

    debug!("New bin dir: {}", new_bin_dir);

    if bin.is_empty() {
        debug!("_NPX_BIN is empty");
        if has_node_modules {
            debug!("has node_modules, return command which including unstriped path directly");
            let result = EnvironmentSettings::new(new_bin_dir, path);
            debug!("Generated settings: {:?}", result);
            return Ok(Some(result));
        } else {
            debug!("No node_modules, do nothing");
            return Ok(None);
        }
    }

    let bin_dirs = bin.split(env_separator).collect::<Vec<_>>();

    let first_bin_dir = *bin_dirs.first().unwrap();

    // Do nothing if bin dir has already added
    if first_bin_dir == new_bin_dir {
        debug!("Already added, do nothing");
        return Ok(None);
    }

    debug!("Raw bin_dirs: {:?}", bin_dirs);

    let striped_path = path
        .split(env_separator)
        .filter(|e| !bin_dirs.contains(e))
        .collect::<Vec<_>>()
        .join(env_separator_str);

    debug!("Striped PATH env: {}", striped_path);

    // Reset PATH if current directory does not contain node_modules
    if !has_node_modules {
        debug!("_NPX_BIN is not empty but no node_modules found, reset PATH");
        return Ok(Some(EnvironmentSettings::new("".into(), striped_path)));
    }

    let mut bin_dirs_iter = bin_dirs.into_iter().peekable();

    let mut use_bin_dirs_only = false;

    while let Some(&next) = bin_dirs_iter.peek() {
        debug!("Peek result: {:?}", next);

        let striped_bin_path = strip_bin_path(next).context("Failed to strip bin path")?;

        debug!("Striped path: {}", striped_bin_path);

        // Use "truncated" bin dirs directly if it's already in there
        if new_bin_dir == next {
            debug!("Use bin dirs directly");
            use_bin_dirs_only = true;
            break;
        } else if new_bin_dir.starts_with(striped_bin_path) {
            debug!("Preserve parent dir(s)");
            break;
        }

        bin_dirs_iter.next();
    }

    let bin_dirs = bin_dirs_iter.collect::<Vec<_>>().join(env_separator_str);

    debug!("Filtered bin_dirs: {}", bin_dirs);

    let result = if !bin_dirs.is_empty() {
        debug!("Reuse bin dirs");
        if use_bin_dirs_only {
            EnvironmentSettings::new(bin_dirs, striped_path)
        } else {
            EnvironmentSettings::new(
                format!(
                    "{bin}{sep}{old}",
                    bin = new_bin_dir,
                    sep = env_separator,
                    old = bin_dirs
                ),
                striped_path,
            )
        }
    } else {
        debug!("Use new bin dir only");
        EnvironmentSettings::new(new_bin_dir, striped_path)
    };

    debug!("Generated settings: {:?}", result);

    Ok(Some(result))
}

pub fn gen_env_settings(shell: &dyn Shell) -> Result<Option<EnvironmentSettings>> {
    let env_npx_bin = {
        let result = env::var(ENV_NAME);

        if let Err(VarError::NotUnicode(_)) = result {
            return Err(anyhow!(NOT_UNICODE_ERR))
                .context("Cannot get environment variable _NPX_BIN");
        };

        // Return an empty string if env var does not exist
        result.unwrap_or_default()
    };

    debug!("_NPX_BIN: {}", env_npx_bin);

    let env_path = env::var("PATH").context("Cannot get environment variable PATH")?;

    debug!("PATH: {}", env_path);

    let cwd = env::current_dir().context("Cannot get current directory")?;

    _gen_env_settings(shell, (env_npx_bin, env_path, cwd))
}