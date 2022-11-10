use super::Shell;

pub struct Fish;

impl Shell for Fish {
    fn name(&self) -> &'static str {
        "fish"
    }
    fn env_separator(&self) -> char {
        ' '
    }
    fn env_separator_str(&self) -> &'static str {
        " "
    }
    fn set_env(&self, key: &str, value: &str) -> String {
        format!("set -gx {} {}", key, value)
    }
    fn gen_setup_script(&self) -> &'static str {
        r#"
function __npx_bin_helper -v PWD
    eval (npx-bin-helper -s fish)
end

__npx_bin_helper
"#
    }
}
