use std::env::{self, VarError};
use std::error::Error;
use std::path::Path;

#[cfg(windows)]
const PATH_SEPARATOR: char = ';';
#[cfg(unix)]
const PATH_SEPARATOR: char = ':';

const ENV_NAME: &str = "_AUTO_NPX";

const CONVERT_PATH_ERROR: &str = "Cannot convert path to valid UTF-8 string";

pub fn get_last_bin_dir() -> Result<String, VarError> {
    let v = env::var("_AUTO_NPX")?;
    Ok((*v.split(PATH_SEPARATOR).collect::<Vec<_>>().last().unwrap()).into())
}

pub fn gen_command() -> Result<String, Box<dyn Error>> {
    let last_bin_dir = {
        let result = get_last_bin_dir();

        if let Err(e @ VarError::NotUnicode(_)) = result {
            return Err(Box::new(e));
        };

        // Return an empty string if env var does not exist
        result.unwrap_or_default()
    };

    let bin_dir_buf = {
        let mut cwd = env::current_dir()?;
        cwd.push("node_modules");

        if !cwd.as_path().is_dir() {
            return Ok(String::new());
        }

        cwd.push(".bin");
        cwd
    };

    let bin_dir = bin_dir_buf.to_str().ok_or(CONVERT_PATH_ERROR)?;

    let result = if last_bin_dir.is_empty() {
        let path_env = env::var("PATH")?;
        format!(
            "export {name}={bin};export PATH={path}{sep}${name}",
            name = ENV_NAME,
            bin = bin_dir,
            sep = PATH_SEPARATOR,
            path = path_env
        )
    } else {
        let is_subdir = {
            let parent_dir = Path::new(&last_bin_dir)
                .ancestors()
                .nth(2)
                .ok_or("Error while parsing path")?
                .to_str()
                .ok_or(CONVERT_PATH_ERROR)?;
            last_bin_dir != bin_dir && bin_dir.starts_with(&parent_dir)
        };

        if is_subdir {
            format!(
                "export {name}={old}{sep}{bin}",
                name = ENV_NAME,
                old = last_bin_dir,
                sep = PATH_SEPARATOR,
                bin = bin_dir
            )
        } else {
            format!("export {name}={bin}", name = ENV_NAME, bin = bin_dir)
        }
    };

    Ok(result)
}
