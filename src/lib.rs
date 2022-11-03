use log::debug;
use std::{
    env::{self, VarError},
    io,
    path::Path,
};
use thiserror::Error;

cfg_if::cfg_if! {
    if #[cfg(windows)] {
        pub const ENV_SEPARATOR: char = ';';
        pub const ENV_SEPARATOR_STR: &str = ";";
    } else {
        pub const ENV_SEPARATOR: char = ':';
        pub const ENV_SEPARATOR_STR: &str = ":";
    }
}

pub const ENV_NAME: &str = "_NPX_BIN";

#[derive(Error, Debug)]
pub enum GenCommandError {
    #[error(transparent)]
    VariableError(#[from] VarError),
    #[error("Cannot convert path to valid UTF-8 string")]
    InvalidPath,
    #[error("Error while parsing path")]
    ParsePathError,
    #[error("IO Error: {0}")]
    IOError(#[from] io::Error),
}

pub fn format_command(bin: &str, path: &str) -> String {
    format!(
        "export {name}={bin};export PATH=${name}{sep}{path}",
        name = ENV_NAME,
        sep = ENV_SEPARATOR,
        bin = bin,
        path = path
    )
}

pub fn strip_bin_path(v: &str) -> Result<&str, GenCommandError> {
    // Note: Only ParsePathError will be returned
    Path::new(v)
        .ancestors()
        .nth(2)
        .ok_or(GenCommandError::ParsePathError)?
        .to_str()
        .ok_or(GenCommandError::InvalidPath)
}

pub fn gen_command() -> Result<String, GenCommandError> {
    let env_npx_bin = {
        let result = env::var(ENV_NAME);

        if let Err(e @ VarError::NotUnicode(_)) = result {
            return Err(GenCommandError::VariableError(e));
        };

        // Return an empty string if env var does not exist
        result.unwrap_or_default()
    };

    debug!("_NPX_BIN: {}", env_npx_bin);

    let env_path = env::var("PATH")?;

    debug!("PATH: {}", env_path);

    let bin_dir_buf = {
        let mut cwd = env::current_dir()?;
        debug!("Cwd: {:?}", cwd);
        cwd.push("node_modules");

        if !cwd.as_path().is_dir() {
            return Ok(String::new());
        }

        cwd.push(".bin");
        cwd
    };

    let new_bin_dir = bin_dir_buf.to_str().ok_or(GenCommandError::InvalidPath)?;

    debug!("New bin dir: {}", new_bin_dir);

    if env_npx_bin.is_empty() {
        debug!("_NPX_BIN not found, return command which including unstriped path directly");
        let result = format_command(new_bin_dir, &env_path);
        debug!("Generated command: {}", result);
        return Ok(result);
    }

    let bin_dirs: Vec<_> = env_npx_bin.split(ENV_SEPARATOR).collect();

    let first_bin_dir = bin_dirs.first().unwrap();

    // Do nothing if bin dir has already added
    if *first_bin_dir == new_bin_dir {
        return Ok(String::new());
    }

    debug!("Raw bin_dirs: {:?}", bin_dirs);

    let striped_path = env_path
        .split(ENV_SEPARATOR)
        .filter(|e| !bin_dirs.contains(e))
        .collect::<Vec<_>>()
        .join(ENV_SEPARATOR_STR);

    debug!("Striped PATH env: {}", striped_path);

    let mut bin_dirs_iter = bin_dirs.into_iter().peekable();

    let mut use_bin_dirs_only = false;

    loop {
        let next = bin_dirs_iter.peek();

        debug!("Peek result: {:?}", next);

        if let Some(v) = next {
            let striped_path = strip_bin_path(v)?;

            debug!("Striped path: {}", striped_path);

            // Use "truncated" bin dirs directly if it's already in there
            if new_bin_dir == *v {
                debug!("Use bin dirs directly");
                use_bin_dirs_only = true;
                break;
            } else if new_bin_dir.starts_with(striped_path) {
                debug!("Preserve parent dir(s)");
                break;
            }
        } else {
            break;
        }

        bin_dirs_iter.next();
    }

    let bin_dirs = bin_dirs_iter.collect::<Vec<_>>().join(ENV_SEPARATOR_STR);

    debug!("Filtered bin_dirs: {}", bin_dirs);

    let result = if !bin_dirs.is_empty() {
        if use_bin_dirs_only {
            format_command(&bin_dirs, &striped_path)
        } else {
            format_command(
                &format!(
                    "{bin}{sep}{old}",
                    bin = new_bin_dir,
                    sep = ENV_SEPARATOR,
                    old = bin_dirs
                ),
                &striped_path,
            )
        }
    } else {
        format_command(new_bin_dir, &striped_path)
    };

    debug!("Generated command: {}", result);

    Ok(result)
}
