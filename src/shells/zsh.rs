use super::Shell;

pub struct Zsh;

impl Shell for Zsh {
    fn name(&self) -> &'static str {
        "zsh"
    }
    fn gen_setup_script(&self) -> &'static str {
        r#"
__npx_bin_helper() {
    eval "$(npx-bin-helper -s zsh)"
}

add-zsh-hook chpwd __npx_bin_helper && __npx_bin_helper
"#
    }
}
