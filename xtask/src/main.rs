use std::{env, process::ExitCode};

use task::Error;

fn try_main() -> Result<(), Error> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("build") => task::build(),
        Some("test") => task::test(),
        Some("create_host_manifest") => task::create_host_manifest(),
        _ => Err("unknown task")?,
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

    const CARGO: &str = "cargo";
    const NPM: &str = "npm";

    const MSG_NO_PROJECT_ROOT: &str = "no project root";

    fn project_root() -> Option<PathBuf> {
        Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(1).map(Path::to_path_buf)
    }

    fn npm_run(working_dir: impl AsRef<Path>, cmd: &str) -> Result<(), Error> {
        let cmd = ["--prefix", "extension", "run", cmd];
        let status = Command::new(NPM).current_dir(working_dir).args(cmd).status()?;
        if !status.success() {
            let code = status.code().unwrap();
            Err(format!("Error: npm {} returned {}", cmd.join(" "), code))?;
        }
        Ok(())
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
        npm_run(&project_root, "check")?;
        npm_run(&project_root, "build")?;
        Ok(())
    }

    pub fn test() -> Result<(), Error> {
        let cargo = env::var("CARGO").unwrap_or_else(|_| String::from(CARGO));
        let project_root = project_root().ok_or(MSG_NO_PROJECT_ROOT)?;
        let cargo_status =
            Command::new(cargo).current_dir(&project_root).args(["test"]).status()?;
        if !cargo_status.success() {
            Err("cargo test failed")?;
        }
        npm_run(&project_root, "test")?;
        Ok(())
    }

    pub fn create_host_manifest() -> Result<(), Error> {
        let project_root = project_root().ok_or(MSG_NO_PROJECT_ROOT)?;
        npm_run(&project_root, "create-host-manifest")?;
        Ok(())
    }
}
