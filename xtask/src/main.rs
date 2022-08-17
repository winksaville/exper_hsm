use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

type DynError = Box<dyn std::error::Error>;

fn main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_ref().map(|it| it.as_str()) {
        Some("pre-commit") => pre_commit()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        r#"Tasks:
pre-commit:    Runs `cargo fmt` and `cargo test`"#
    );
}

fn pre_commit() -> Result<(), DynError> {
    cargo_fmt()?;
    cargo_test()?;

    Ok(())
}

fn cargo_fmt() -> Result<(), DynError> {
    eprintln!("Run cargo fmt");
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .arg("fmt")
        .status()?;

    if !status.success() {
        Err("cargo build failed")?;
    }

    Ok(())
}

fn cargo_test() -> Result<(), DynError> {
    eprintln!("Run cargo test");
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .arg("test")
        .status()?;

    if !status.success() {
        Err("cargo test failed")?;
    }

    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
