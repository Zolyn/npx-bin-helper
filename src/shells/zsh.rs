use super::Shell;

pub struct Zsh;

impl Shell for Zsh {
    fn gen_setup_script(&self) -> &'static str {
        r#"
__npx_bin_helper() {
    eval "$(npx_bin_helper -s zsh)"
}

add-zsh-hook chpwd __npx_bin_helper && __npx_bin_helper
"#
    }
}
