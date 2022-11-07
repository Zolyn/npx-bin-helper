use crate::{consts::ENV_NAME, shells::Shells};

use super::generate::EnvironmentSettings;

pub fn format_env_settings(_s: Shells, settings: EnvironmentSettings) -> String {
    let set_env_cmd = "export";
    format!(
        "{cmd} {name}={bin};{cmd} PATH={path}",
        cmd = set_env_cmd,
        bin = settings.bin,
        path = settings.path,
        name = ENV_NAME
    )
}
