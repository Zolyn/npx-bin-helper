use crate::shells::Shells;

pub trait Setup {
    fn gen_setup_script(&self) -> String;
}

pub fn create_setup(s: Shells) -> Box<dyn Setup> {
    match s {
        Shells::Bash => Box::new(Bash),
        Shells::Zsh => Box::new(Zsh),
    }
}

macro_rules! impl_setup {
    ($s:ident, $v:expr) => {
        pub struct $s;

        impl Setup for $s {
            fn gen_setup_script(&self) -> String {
                $v.to_string()
            }
        }
    };
}

impl_setup! {
    Bash,
    "\
__npx_bin_helper_pwd=$PWD

__npx_bin_helper() {
    if [ \"$PWD\" != \"$__npx_bin_helper_pwd\" ]; then
        __npx_bin_helper_pwd=\"PWD\"
        eval \"$(npx_bin_helper --shell bash)\"
    fi
}

PROMPT_COMMAND=\"__npx_bin_helper;${PROMPT_COMMAND#;}\"    
"
}

impl_setup! {
    Zsh,
    "\
__npx_bin_helper() {
    eval \"$(npx_bin_helper --shell zsh)\"
}

add-zsh-hook chpwd __npx_bin_helper && __npx_bin_helper
"
}
