use std::path::{Component, Path, Prefix};

use anyhow::{anyhow, bail, Ok, Result};

use crate::{consts::NOT_UNICODE_ERR, shells::Shell};

/**
 * Forked from: https://github.com/rhysd/path-slash/blob/3cbcbb0e3a88d5bcafb51a685bca5b2cfda03c8f/src/lib.rs#L209
 */
#[cfg(windows)]
pub fn to_unix_like_path<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();
    let mut result = String::from("/");

    for comp in path.components() {
        match comp {
            Component::Prefix(prefix) => {
                if let Prefix::Disk(d) = prefix.kind() {
                    result.push(d as char)
                } else {
                    bail!("Unsupported path prefix")
                }
            }
            Component::Normal(p) => {
                result.push_str(p.to_str().ok_or_else(|| anyhow!(NOT_UNICODE_ERR))?)
            }
            Component::CurDir => result.push('.'),
            Component::ParentDir => result.push_str(".."),
            Component::RootDir => continue,
        }
        result.push('/')
    }

    // Remove last '/'
    if result != "/" {
        result.pop();
    }

    Ok(result)
}

pub fn is_git_bash(shell: &dyn Shell) -> bool {
    cfg!(windows) && shell.name() != "powershell"
}
