use std::{
    env,
    path::{Path, PathBuf},
    process::{exit, Command},
};

type DynError = Box<dyn std::error::Error>;

fn main() -> Result<(), DynError> {
    let mut env_iter = env::args();
    let task = env_iter.nth(1);
    let remaining_args: Vec<String> = env_iter.collect();
    //let remaining_args: Vec<String> = env_iter.map(|a| a).collect();
    match task.as_deref() {
        Some("fmt") => cargo_cmd("fmt", &remaining_args)?,
        Some("test") => cargo_cmd("test", &remaining_args)?,
        Some("clippy") => cargo_cmd("clippy", &remaining_args)?,
        Some("pre-commit") => pre_commit(&remaining_args)?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        r#"Tasks:
clippy:        Runs `cargo clippy`
fmt:           Runs `cargo fmt`
test:          Runs `cargo test`
pre-commit:    Runs `cargo fmt`, `cargo clippy` and `cargo test`"#
    );
}

fn pre_commit(remaining_args: &Vec<String>) -> Result<(), DynError> {
    cargo_cmd_prj_root("test", remaining_args)?;
    cargo_cmd_prj_root("clippy", remaining_args)?;
    cargo_cmd_prj_root("fmt", remaining_args)?;

    Ok(())
}

fn cargo_cmd(cmd: &str, remaining_args: &Vec<String>) -> Result<(), DynError> {
    eprintln!("Run cargo {cmd} {remaining_args:?}");

    let status = Command::new(cargo_string())
        .arg(cmd)
        .args(remaining_args)
        .status()?;

    if !status.success() {
        Err("cargo {cmd} {remaining_args:?} Failed")?;
    }
    Ok(())
}

fn cargo_cmd_prj_root(cmd: &str, remaining_args: &Vec<String>) -> Result<(), DynError> {
    eprintln!("Run cargo {cmd} {remaining_args:?}");

    let status = Command::new(cargo_string())
        .current_dir(project_root())
        .arg(cmd)
        .args(remaining_args)
        .status()?;

    if !status.success() {
        Err("cargo {cmd} {remaining_args:?} Failed")?;
    }
    Ok(())
}

fn cargo_string() -> String {
    match env::var("CARGO") {
        Ok(cs) => cs,
        Err(_) => {
            eprintln!("No CARGO environment variable, is cargo installed?");
            exit(2);
        }
    }
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
