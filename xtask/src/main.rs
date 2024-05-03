use std::{env, process::ExitCode};

use task::Error;

fn try_main() -> Result<(), Error> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("build") => task::build(),
        Some("test") => task::test(),
        Some("create_host_manifest") => task::create_host_manifest(),
        _ => Err(String::from("unknown task"))?,
    }
}

fn main() -> ExitCode {
    if let Err(err) = try_main() {
        eprintln!("Error: {}", err);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

mod task {
    use std::{
        env,
        path::{Path, PathBuf},
        process::Command,
    };

    pub type Error = Box<dyn std::error::Error>;

    const EXTENSION_ROOT: &str = "extension";
    const CARGO: &str = "cargo";
    const NPM: &str = "npm";
    const MSG_NO_PROJECT_ROOT: &str = "no project root";

    fn project_root() -> Option<PathBuf> {
        Path::new(&env!("CARGO_MANIFEST_DIR")).ancestors().nth(1).map(Path::to_path_buf)
    }

    pub fn build() -> Result<(), Error> {
        let cargo = env::var("CARGO").unwrap_or_else(|_| String::from(CARGO));
        let project_root = project_root().ok_or(MSG_NO_PROJECT_ROOT)?;
        let cargo_status = Command::new(cargo)
            .current_dir(&project_root)
            .args(["build", "--all-targets"])
            .status()?;
        if !cargo_status.success() {
            Err("cargo build failed")?;
        }
        let npm_status = Command::new(NPM)
            .current_dir(&project_root)
            .args(["--prefix", EXTENSION_ROOT, "run", "check"])
            .status()?;
        if !npm_status.success() {
            Err("npm run build failed")?;
        }
        let npm_status = Command::new(NPM)
            .current_dir(&project_root)
            .args(["--prefix", EXTENSION_ROOT, "run", "build"])
            .status()?;
        if !npm_status.success() {
            Err("npm run build failed")?;
        }
        Ok(())
    }

    pub fn test() -> Result<(), Error> {
        let cargo = env::var("CARGO").unwrap_or_else(|_| String::from(CARGO));
        let project_root = project_root().ok_or(MSG_NO_PROJECT_ROOT)?;
        let cargo_status =
            Command::new(cargo).current_dir(&project_root).args(["test"]).status()?;
        if !cargo_status.success() {
            Err("cargo build failed")?;
        }
        let npm_status = Command::new(NPM)
            .current_dir(&project_root)
            .args(["--prefix", EXTENSION_ROOT, "run", "test"])
            .status()?;
        if !npm_status.success() {
            Err("npm run build failed")?;
        }
        Ok(())
    }

    pub fn create_host_manifest() -> Result<(), Error> {
        let project_root = project_root().ok_or(MSG_NO_PROJECT_ROOT)?;
        let npm_status = Command::new(NPM)
            .current_dir(&project_root)
            .args(["--prefix", EXTENSION_ROOT, "run", "create-host-manifest"])
            .status()?;
        if !npm_status.success() {
            Err("npm run create-host-manifest failed")?;
        }
        Ok(())
    }
}
