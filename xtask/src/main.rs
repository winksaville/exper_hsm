// xtask: based on https://github.com/winksaville/cargo-xtask/examples/wink, which
// is based on https://github.com/matklad/cargo-xtask.
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
        Some("pre-commit") => pre_commit(&remaining_args)?,
        Some("gen-cov") => gen_cov(&get_current_dir())?,
        Some("fmt") => cargo_cmd(&get_current_dir(), "fmt", &remaining_args)?,
        Some("test") => cargo_cmd(&get_current_dir(), "test", &remaining_args)?,
        Some("clippy") => cargo_cmd(&get_current_dir(), "clippy", &remaining_args)?,
        Some("gen-profraw") => gen_profraw(&get_current_dir())?,
        Some("gen-html") => gen_html(&get_current_dir())?,
        Some("gen-lcov") => gen_lcov(&get_current_dir())?,
        Some("gen-covdir") => gen_covdir(&get_current_dir())?,
        Some("cwd") => cwd()?,
        Some("pcr") => pcr()?,
        Some("mk-empty-cov-dir") => mk_empty_cov_dir(&get_current_dir())?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        r#"Tasks:
pre-commit:    Runs `cargo fmt`, `cargo clippy` and `cargo test`
gen-cov:       Removes <current-dir>/coverage/ then generates coverage data in <current-dir>/coverage/
               using gen-profraw, gen-html gen-lcov and gen-covdir.

tasks for testing gen-phl:
    clippy:           Runs `cargo clippy`
    fmt:              Runs `cargo fmt`
    test:             Runs `cargo test`
    gen-profraw:      Runs `cargo test` with `-cinstrument-coverage` generating `<proj-root>/coverage/*.profraw` files
    gen-html:         Runs `grcov` generating html files in `<proj-root>/coverage/html/`
    gen-lcov:         Rust `grcov` generating `<proj-root>/coverage/tests.lcov`
    gen-covdir:       Rust `grcov` generating `<proj-root>/coverage/tests.covdir.json`
    cwd:              Display current working director
    pcr:              Display project coverage root
    mk-empty-cov-dir  Make empty coverage directory
"#
    );
}

fn pre_commit(remaining_args: &Vec<String>) -> Result<(), DynError> {
    cargo_cmd(&project_root(), "fmt", remaining_args)?;
    cargo_cmd(&project_root(), "clippy", remaining_args)?;
    cargo_cmd(&project_root(), "test", remaining_args)?;

    Ok(())
}

fn gen_cov(root: &Path) -> Result<(), DynError> {
    cargo_clean()?;
    mk_empty_cov_dir(root)?;
    gen_profraw(root)?;
    gen_html(root)?;
    gen_lcov(root)?;
    gen_covdir(root)?;

    Ok(())
}

fn mk_empty_cov_dir(root: &Path) -> Result<(), DynError> {
    // Ignore errors as coverage dir may or may not exit
    if std::fs::remove_dir_all(root.join("coverage")).is_ok() {}

    // But always create but return errors if it fails :)
    std::fs::create_dir_all(root.join("coverage"))?;

    Ok(())
}

fn gen_profraw(root: &Path) -> Result<(), DynError> {
    let coverage_dir = project_coverage_root(root)?;
    eprintln!("Create profraw data at {coverage_dir}");

    let status = Command::new("cargo")
        // env flags from:
        //   https://doc.rust-lang.org/beta/unstable-book/compiler-flags/profile.html
        //   https://github.com/mozilla/grcov/blob/master/README.md
        .env("CARGO_INCREMENTAL", "0")

        // Using -Zprofile:
        //   Directory	        Line Coverage	    Functions	        Branches
        //   hsm0-executor/src  89.49%	664 / 742	86.44%	255 / 295	46.1%	651 / 1412
        //.env("RUSTFLAGS", "-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort")

        // Using -Zinstrument-coverage:
        //   Directory	        Line Coverage	    Functions	        Branches
        //   hsm0-executor/src	100%	240 / 240	91.55%	195 / 213	100%	0 / 0
        .env("RUSTFLAGS", "-Cinstrument-coverage -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests")

        .env("TMPDIR", coverage_dir)
        .env("LLVM_PROFILE_FILE", "%t/cargo-test-%p-%m.profraw")
        .arg("test")
        .arg("--lib")
        //.args(["-p", "sub", "-p", "add"]) // All packages if none, else choose specific packages
        //.args(remaining_args)
        .status()?;

    if !status.success() {
        Err("`cargo test --lib` with code-coverage Failed")?;
    }

    Ok(())
}

fn gen_html(root: &Path) -> Result<(), DynError> {
    let output_path_buf = root.join("coverage").join("html");
    gen_coverage("html", &output_path_buf)
}

fn gen_lcov(root: &Path) -> Result<(), DynError> {
    let output_path_buf = root.join("coverage").join("tests.lcov");
    gen_coverage("lcov", &output_path_buf)
}

fn gen_covdir(root: &Path) -> Result<(), DynError> {
    let output_path_buf = root.join("coverage").join("tests.covdir.json");
    gen_coverage("covdir", &output_path_buf)
}

fn gen_coverage(output_type: &str, output_path_buf: &Path) -> Result<(), DynError> {
    let output_path = output_path_buf.to_string_lossy();
    eprintln!("Create {output_path}");

    let pb = project_root().join("target").join("debug").join("deps");
    let binary_path = pb.to_string_lossy();

    //let pb = Path::new("/home/wink/prgs/rust/clones/grcov/target/debug/grcov");
    //let grcov = pb.to_string_lossy().to_string();
    let grcov = "grcov";
    let status = Command::new(grcov)
        .current_dir(project_root())
        .args([
            ".",
            "--binary-path",
            &binary_path,
            "--branch",
            "--ignore-not-existing",
            "--source-dir",
            ".",
            // All --ignore options are releative to --source-dir
            "--ignore",
            "xtask/*",
            //"--ignore",
            //"*/src/tests/*",
            //"--ignore",
            //"../*", // Ignore all explicitly relative paths
            //"--ignore",
            //"/*", // Ignore all absolute paths
            "--output-type",
            output_type,
            "--output-path",
            &output_path,
        ])
        //.args(remaining_args)
        .status()?;

    if !status.success() {
        Err(format!("Creating {output_path} Failed"))?;
    }

    Ok(())
}

fn cwd() -> Result<(), DynError> {
    eprintln!("{}", get_current_dir().to_string_lossy());

    Ok(())
}

fn pcr() -> Result<(), DynError> {
    let coverage_dir = project_coverage_root(&get_current_dir())?;
    eprintln!("{coverage_dir}");

    Ok(())
}

fn cargo_clean() -> Result<(), DynError> {
    cargo_cmd(&get_current_dir(), "clean", &Vec::<String>::new())
}

fn cargo_cmd(root: &Path, cmd: &str, remaining_args: &Vec<String>) -> Result<(), DynError> {
    eprintln!("Run cargo {cmd} {remaining_args:?}");

    let status = Command::new(cargo_string())
        .current_dir(root)
        .arg(cmd)
        .args(remaining_args)
        .status()?;

    if !status.success() {
        Err(format!("cargo {cmd} {remaining_args:?} Failed"))?;
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

fn project_coverage_root(root: &Path) -> Result<String, DynError> {
    let pb = root.join("coverage");
    let coverage_dir = match pb.to_str() {
        Some(dir) => dir,
        None => return Err("Unable to create coverage dir".into()),
    };

    Ok(coverage_dir.to_owned())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn get_current_dir() -> PathBuf {
    env::current_dir().unwrap()
}
