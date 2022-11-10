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
        let res = format!(
            "{};{}",
            shell.set_env(ENV_NAME, &bin),
            shell.set_env("PATH", &path)
        );

        print!("{}", res);
    }

    Ok(())
}
