use super::Shell;

pub struct PowerShell;

impl Shell for PowerShell {
    fn name(&self) -> &'static str {
        "powershell"
    }
    #[cfg(windows)]
    fn env_separator(&self) -> char {
        ';'
    }
    #[cfg(unix)]
    fn env_separator(&self) -> char {
        ':'
    }
    #[cfg(windows)]
    fn env_separator_str(&self) -> &'static str {
        ";"
    }
    #[cfg(unix)]
    fn env_separator_str(&self) -> &'static str {
        ":"
    }
    fn set_env(&self, key: &str, value: &str) -> String {
        format!(r#"$env:{} = "{}""#, key, value)
    }
    fn gen_setup_script(&self) -> &'static str {
        r#"
function __npx_bin_helper_pwd {
    $cwd = Get-Location
    if ($cwd.Provider.Name -eq "FileSystem") {
        $cwd.ProviderPath
    }
}

function __npx_bin_helper {
    $result = __npx_bin_helper_pwd
    if ($null -ne $result) {
        npx-bin-helper -s powershell | Out-String | Invoke-Expression
    }
}

$__npx_bin_helper_hooked = (Get-Variable __npx_bin_helper_hooked -ValueOnly -ErrorAction SilentlyContinue)

if ($__npx_bin_helper_hooked -ne 1) {
    $__npx_bin_helper_hooked = 1
    $prompt_old = $function:prompt
    function prompt {
        $null = __npx_bin_helper
        & $prompt_old
    }
}
"#
    }
}
