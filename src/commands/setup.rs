use crate::shells::Shell;

pub fn call(shell: Box<dyn Shell>) {
    let script = shell.gen_setup_script();
    print!("{}", script)
}
