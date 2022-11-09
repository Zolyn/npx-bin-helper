use anyhow::{Context, Ok, Result};

use crate::{
    consts::ENV_NAME,
    generate::{self, EnvironmentSettings},
    shells::Shell,
};

pub fn call(shell: Box<dyn Shell>) -> Result<()> {
    let Some(env_settings) = generate::gen_env_settings(&*shell).context("Cannot generate environment settings")? else {
        return Ok(())
    };

    let EnvironmentSettings { bin, path, .. } = env_settings;

    let res = format!(
        "{};{}",
        shell.set_env(ENV_NAME, &bin),
        shell.set_env("PATH", &path)
    );

    print!("{}", res);

    Ok(())
}
