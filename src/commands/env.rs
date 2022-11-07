use std::env::{self, VarError};

use anyhow::{anyhow, Context, Ok, Result};

use log::debug;

use crate::{
    consts::{ENV_NAME, NOT_UNICODE_ERR},
    shells::Shells,
};

mod format;
mod generate;

pub fn gen_env_settings(shell: Shells) -> Result<String> {
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

    let env_settings = generate::gen_env_settings(env_npx_bin, env_path)
        .context("Cannot generate environment settings")?;

    let result = if let Some(settings) = env_settings {
        format::format_env_settings(shell, settings)
    } else {
        "".into()
    };

    debug!("Result: {}", result);

    Ok(result)
}
