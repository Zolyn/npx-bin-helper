use std::process;

fn main() {
    let result = npx_bin_helper::gen_command().unwrap_or_else(|e| {
        eprintln!("[NPX_BIN_HELPER]: An error was occurred: {}", e);
        process::exit(1)
    });
    print!("{}", result);
}
