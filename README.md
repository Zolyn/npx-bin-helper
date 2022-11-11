# npx-bin-helper

[![Crates.io](https://img.shields.io/crates/v/npx-bin-helper?style=flat-square)](https://crates.io/crates/npx-bin-helper)

Generate commands that add `node_modules/.bin` to PATH. Supports Linux, MacOS and Windows.

## Installation
```bash
cargo install npx-bin-helper
```

## Usage
```bash
npx-bin-helper -s <SHELL>
```
This will generate commands to set environment variables depending on your shell type.

Currently npx-bin-helper supports the following shells:

- `bash` (Including Git Bash on Windows)
- `zsh`
- `fish`
- `powershell`

For example:
```bash
npx-bin-helper -s bash
```
Output:
```bash
export _NPX_BIN="/path/to/project/node_modules/.bin"
export PATH="/path/to/project/node_modules/.bin:..."
```

To generate and evaluate commands automatically when changing directory, you need to set up your shell.

## Shell setup
### Bash
Add the following to `.bashrc`
```bash
eval "$(npx-bin-helper setup -s bash)"
```

### Zsh
Add the following to `.zshrc`
```zsh
eval "$(npx-bin-helper setup -s zsh)"
```

### Fish
Create `~/.config/fish/conf.d/npx-bin-helper.fish` and add the following to it
```fish
npx-bin-helper setup -s fish | source
```

### PowerShell
Add the following to your profile
```powershell
npx-bin-helper setup -s pwsh | Out-String | Invoke-Expression
```

## License
[MIT](./LICENSE)

## References
- [fnm](https://github.com/Schniz/fnm)
- [zoxide](https://github.com/ajeetdsouza/zoxide)
