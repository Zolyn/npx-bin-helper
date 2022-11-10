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
    if [ "$PWD" != "$__npx_bin_helper_pwd" ]; then
        __npx_bin_helper_pwd="$PWD"
        eval "$(npx-bin-helper -s bash)"
    fi
}

PROMPT_COMMAND="__npx_bin_helper;${PROMPT_COMMAND#;}"
"#
    }
}
