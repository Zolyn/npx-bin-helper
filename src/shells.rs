use anyhow::{anyhow, Result};
use std::ffi::OsString;

use crate::error_consts::NOT_UNICODE_ERR;

pub trait Shell {
    fn gen_setup_command(&self) -> String;
}

pub fn shell_from_os_string(s: OsString) -> Result<Box<dyn Shell>> {
    let v = s.to_str().ok_or_else(|| anyhow!(NOT_UNICODE_ERR))?;
    match v {
        "bash" => Ok(Box::new(Bash)),
        "zsh" => Ok(Box::new(Zsh)),
        _ => Err(anyhow!("Unsupported shell, try setup manually")),
    }
}

macro_rules! impl_shell {
    ($s:ident, $v:expr) => {
        pub struct $s;

        impl Shell for $s {
            fn gen_setup_command(&self) -> String {
                $v.to_string()
            }
        }
    };
}

impl_shell! {
    Bash,
    "\
__npx_bin_helper_pwd=$PWD

__npx_bin_helper() {
    if [ \"$PWD\" != \"$__npx_bin_helper_pwd\" ]; then
        __npx_bin_helper_pwd=\"PWD\"
        eval \"$(npx_bin_helper)\"
    fi
}

PROMPT_COMMAND=\"__npx_bin_helper;${PROMPT_COMMAND#;}\"    
"
}

impl_shell! {
    Zsh,
    "\
__npx_bin_helper() {
    eval \"$(npx_bin_helper)\"
}

add-zsh-hook chpwd __npx_bin_helper && __npx_bin_helper
"
}
