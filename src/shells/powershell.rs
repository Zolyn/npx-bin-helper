use super::Shell;

pub struct PowerShell;

impl Shell for PowerShell {
    fn env_separator(&self) -> char {
        ';'
    }
    fn env_separator_str(&self) -> &'static str {
        ";"
    }
    fn set_env(&self, key: &str, value: &str) -> String {
        format!(r#"$env:{} = "{}""#, key, value)
    }
    fn gen_setup_script(&self) -> &'static str {
        r#"
"#
    }
}
