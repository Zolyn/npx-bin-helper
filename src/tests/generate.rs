use std::{env::join_paths, fs, path::PathBuf};

use anyhow::{anyhow, Ok, Result};
use rstest::*;
use serial_test::serial;
use tempfile::TempDir;

use super::{gen_env_settings_by as gen_env_settings, GenerateStatus};
use crate::{
    consts::NOT_UNICODE_ERR,
    shells::{Bash, Shell},
};

type BoxedShell = Box<dyn Shell>;

#[fixture]
#[once]
fn shell(_log: ()) -> BoxedShell {
    Box::new(Bash)
}

#[fixture]
#[once]
fn log() {
    env_logger::builder().format_timestamp(None).init()
}

#[fixture]
fn path() -> String {
    "/usr/bin".to_string()
}

type Temp = Result<TempDir>;

#[fixture]
fn tmp() -> Temp {
    Ok(tempfile::tempdir()?)
}

#[rstest]
#[serial]
fn empty_var_has_modules(shell: &BoxedShell, path: String, tmp: Temp) -> Result<()> {
    let shell = shell.as_ref();
    let tmp = tmp?;
    let tmp_dir = tmp.path();
    fs::create_dir(tmp_dir.join("node_modules"))?;

    let envs = (String::new(), path, tmp_dir.to_path_buf());
    let status = gen_env_settings(shell, envs)?.1;

    assert_eq!(status, GenerateStatus::EmptyVarHasModules);
    tmp.close()?;

    Ok(())
}

#[rstest]
#[serial]
fn empty_var_no_modules(shell: &BoxedShell, path: String, tmp: Temp) -> Result<()> {
    let shell = shell.as_ref();
    let tmp = tmp?;

    let envs = (String::new(), path, tmp.path().to_path_buf());
    let status = gen_env_settings(shell, envs)?.1;

    assert_eq!(status, GenerateStatus::EmptyVarNoModules);
    tmp.close()?;

    Ok(())
}

#[rstest]
#[serial]
fn same_dir(shell: &BoxedShell, path: String, tmp: Temp) -> Result<()> {
    let shell = shell.as_ref();
    let tmp = tmp?;
    let tmp_dir = tmp.path();
    fs::create_dir(tmp_dir.join("node_modules"))?;

    let mut env_npx_bin = tmp_dir.join("node_modules");
    env_npx_bin.push(".bin");

    let env_npx_bin = env_npx_bin
        .to_str()
        .ok_or_else(|| anyhow!(NOT_UNICODE_ERR))?
        .to_string();

    let envs = (env_npx_bin, path, tmp_dir.to_path_buf());
    let status = gen_env_settings(shell, envs)?.1;

    assert_eq!(status, GenerateStatus::AlreadyAdded);
    tmp.close()?;

    Ok(())
}

#[rstest]
#[serial]
fn reset_path(shell: &BoxedShell, path: String, tmp: Temp) -> Result<()> {
    let shell = shell.as_ref();
    let tmp = tmp?;
    let tmp_dir = tmp.path();

    let mut env_npx_bin = tmp_dir.join("node_modules");
    env_npx_bin.push(".bin");

    let env_npx_bin = env_npx_bin
        .to_str()
        .ok_or_else(|| anyhow!(NOT_UNICODE_ERR))?
        .to_string();
    let cwd = tmp_dir.ancestors().nth(1).unwrap().to_path_buf();

    let envs = (env_npx_bin, path, cwd);
    let status = gen_env_settings(shell, envs)?.1;

    assert_eq!(status, GenerateStatus::ResetPath);
    tmp.close()?;

    Ok(())
}

#[rstest]
#[serial]
fn subdir_no_modules(shell: &BoxedShell, path: String, tmp: Temp) -> Result<()> {
    let shell = shell.as_ref();
    let tmp = tmp?;
    let tmp_dir = tmp.path();

    let mut env_npx_bin = tmp_dir.join("node_modules");
    env_npx_bin.push(".bin");

    let env_npx_bin = env_npx_bin
        .to_str()
        .ok_or_else(|| anyhow!(NOT_UNICODE_ERR))?
        .to_string();

    let envs = (env_npx_bin, path, tmp_dir.join("foo"));
    let status = gen_env_settings(shell, envs)?.1;

    assert_eq!(status, GenerateStatus::KeepVar);
    tmp.close()?;

    Ok(())
}

#[rstest]
#[serial]
fn use_old_dirs_only(shell: &BoxedShell, path: String, tmp: Temp) -> Result<()> {
    let shell = shell.as_ref();
    let tmp = tmp?;
    let tmp_dir = tmp.path();

    let tmp_dir_str = tmp_dir.to_str().ok_or_else(|| anyhow!(NOT_UNICODE_ERR))?;

    let env_npx_bin_iter = [tmp_dir_str, "foo", "bar"]
        .iter()
        .collect::<PathBuf>()
        .ancestors()
        .take(3)
        .map(|e| {
            let mut bin_dir = e.join("node_modules");
            bin_dir.push(".bin");
            bin_dir.to_str().unwrap().to_string()
        })
        .collect::<Vec<_>>();

    fs::create_dir(tmp_dir.join("node_modules"))?;

    let env_npx_bin = join_paths(env_npx_bin_iter)?
        .into_string()
        .map_err(|_| anyhow!(NOT_UNICODE_ERR))?;
    let envs = (env_npx_bin, path, tmp_dir.to_path_buf());
    let status = gen_env_settings(shell, envs)?.1;

    assert_eq!(status, GenerateStatus::UseOldDirsOnly);
    tmp.close()?;

    Ok(())
}

#[rstest]
#[serial]
fn concat_dirs(shell: &BoxedShell, path: String, tmp: Temp) -> Result<()> {
    let shell = shell.as_ref();
    let tmp = tmp?;
    let tmp_dir = tmp.path();

    let cwd = tmp_dir.join("foo");

    fs::create_dir_all(cwd.join("node_modules"))?;

    let mut env_npx_bin = tmp_dir.join("node_modules");
    env_npx_bin.push(".bin");

    let env_npx_bin = env_npx_bin
        .to_str()
        .ok_or_else(|| anyhow!(NOT_UNICODE_ERR))?
        .to_string();

    let envs = (env_npx_bin, path, cwd);
    let status = gen_env_settings(shell, envs)?.1;

    assert_eq!(status, GenerateStatus::ConcatDirs);
    tmp.close()?;

    Ok(())
}

#[rstest]
#[serial]
fn use_new_dir_only(shell: &BoxedShell, path: String, tmp: Temp) -> Result<()> {
    let shell = shell.as_ref();
    let tmp = tmp?;
    let tmp_dir = tmp.path();

    fs::create_dir(tmp_dir.join("node_modules"))?;

    let mut env_npx_bin = tmp_dir.join("foo");
    env_npx_bin.push("node_modules");
    env_npx_bin.push(".bin");

    let env_npx_bin = env_npx_bin
        .to_str()
        .ok_or_else(|| anyhow!(NOT_UNICODE_ERR))?
        .to_string();

    let envs = (env_npx_bin, path, tmp_dir.to_path_buf());
    let status = gen_env_settings(shell, envs)?.1;

    assert_eq!(status, GenerateStatus::UseNewDirOnly);
    tmp.close()?;

    Ok(())
}
