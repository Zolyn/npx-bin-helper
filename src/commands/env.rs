use anyhow::{Context, Ok, Result};

use crate::{
    consts::ENV_NAME,
    generate::{self, EnvironmentSettings},
    shells::Shell,
};

pub fn call(shell: Box<dyn Shell>) -> Result<()> {
    if let Some(EnvironmentSettings { bin, path }) =
        generate::gen_env_settings(&*shell).context("Cannot generate environment settings")?
    {
        let combined_path = if bin.is_empty() {
            path
        } else {
            format!("{}{sep}{}", &bin, &path, sep = shell.env_separator())
        };

        println!("{}", shell.set_env(ENV_NAME, &bin));
        println!("{}", shell.set_env("PATH", &combined_path))
    }

    Ok(())
}
