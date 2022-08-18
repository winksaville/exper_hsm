use duct::cmd;
use std::{
    env,
    path::{Path, PathBuf},
    process::exit,
};

type DynError = Box<dyn std::error::Error>;

fn main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("pre-commit") => pre_commit()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        r#"Tasks:
pre-commit:    Runs `cargo fmt`, `cargo clippy` and `cargo test`"#
    );
}

fn pre_commit() -> Result<(), DynError> {
    cargo_fmt()?;
    cargo_clippy()?;
    cargo_test()?;

    Ok(())
}

fn cargo_clippy() -> Result<(), DynError> {
    eprintln!("Run cargo clippy");
    cmd!(cargo_string(), "clippy").dir(project_root()).run()?;

    Ok(())
}

fn cargo_fmt() -> Result<(), DynError> {
    eprintln!("Run cargo fmt");
    cmd!(cargo_string(), "fmt").dir(project_root()).run()?;

    Ok(())
}

fn cargo_test() -> Result<(), DynError> {
    eprintln!("Run cargo test");
    cmd!(cargo_string(), "test").dir(project_root()).run()?;

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
    //println!("env!(CARGO): {:?}", env!("CARGO"));
    //println!("env!(CARGO_MANIFEST_DIR): {:?}", env!("CARGO_MANIFEST_DIR"));
    //let dot_dir = Path::new(".");
    //println!("dot_dir={:?}", dot_dir);
    //let bin_dir = Path::new(&env!("CARGO"));
    //println!("bin_dir={:?}", bin_dir);
    //let man_dir = Path::new(&env!("CARGO_MANIFEST_DIR"));
    //println!("project_root(): man_dir={man_dir:?}");
    //let mut ansestors = man_dir.ancestors();
    //println!("project_root(): ansestors={:?}", &ansestors);
    //let nth1 = ansestors.nth(1).unwrap();
    //println!("project_root(): nth1={:?}", &nth1);
    //let pb = nth1.to_path_buf();
    //println!("project_root(): pb={pb:?}");
    //pb
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
