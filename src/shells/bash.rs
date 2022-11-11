use super::Shell;

pub struct Bash;

impl Shell for Bash {
    fn name(&self) -> &'static str {
        "bash"
    }
    fn gen_setup_script(&self) -> &'static str {
        r#"
__npx_bin_helper_pwd="$PWD"

__npx_bin_helper() {
    eval "$(npx-bin-helper -s bash)"
}

__npx_bin_helper_hook() {
    if [ "$PWD" != "$__npx_bin_helper_pwd" ]; then
        __npx_bin_helper_pwd="$PWD"
        __npx_bin_helper
    fi
}

PROMPT_COMMAND="__npx_bin_helper_hook;${PROMPT_COMMAND#;}"
__npx_bin_helper
"#
    }
}
