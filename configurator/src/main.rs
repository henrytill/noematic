use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use clap::Parser;
use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer, Value};

use host_manifest::{Chromium, Firefox, ManifestPath};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to binary
    #[arg(short, long)]
    binary: Option<PathBuf>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let path = match args.binary {
        Some(path) => fs::canonicalize(path)?,
        None => {
            let prefix = default_prefix()?;
            host_manifest::host_binary_path(prefix)?
        }
    };

    let default_dir = env::current_dir()?;

    {
        let manifest = serde_json::to_value(Firefox::new(&path))?;
        let manifest_path = ManifestPath::for_platform(Firefox::path(), &default_dir);
        write(&manifest_path, &manifest)?;
        println!("Firefox host manifest written to: {}", manifest_path.display());
        println!("Firefox host manifest contents: {}", pretty_value(&manifest)?)
    }

    {
        let manifest = serde_json::to_value(Chromium::new(&path))?;
        let manifest_path = ManifestPath::for_platform(Chromium::path(), &default_dir);
        write(&manifest_path, &manifest)?;
        println!("Chromium host manifest written to: {}", manifest_path.display());
        println!("Chromium host manifest contents: {}", pretty_value(&manifest)?)
    }

    Ok(())
}

fn exe_dir() -> Result<PathBuf, anyhow::Error> {
    let exe = env::current_exe()?;
    let parent = exe
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Executable directory not found"))?;
    Ok(PathBuf::from(parent))
}

fn default_prefix() -> Result<PathBuf, anyhow::Error> {
    let exe_dir = exe_dir()?;
    let parent = exe_dir
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Parent directory not found"))?;
    Ok(PathBuf::from(parent))
}

fn pretty_value(value: &Value) -> Result<String, anyhow::Error> {
    let mut buf = Vec::new();
    let formatter = PrettyFormatter::with_indent(b"    ");
    let mut ser = Serializer::with_formatter(&mut buf, formatter);
    value.serialize(&mut ser)?;
    let ret = String::from_utf8(buf)?;
    Ok(ret)
}

fn write(path: impl AsRef<Path>, value: &Value) -> Result<(), anyhow::Error> {
    let json = pretty_value(value)?;
    if let Some(parent) = path.as_ref().parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(path, json).map_err(Into::into)
}

mod host_manifest {
    use std::path::{Path, PathBuf};

    use directories::BaseDirs;
    use serde::Serialize;

    const NAME: &str = "com.github.henrytill.noematic";
    const HOST_BINARY_NAME: &str = "noematic";
    const DESCRIPTION: &str = "Search your backlog";
    const TYPE: &str = "stdio";

    fn file() -> String {
        format!("{}.json", NAME)
    }

    pub fn host_binary_path(prefix: impl AsRef<Path>) -> Result<PathBuf, anyhow::Error> {
        let mut ret = PathBuf::from(prefix.as_ref());
        ret.push("bin");
        ret.push(HOST_BINARY_NAME);
        Ok(ret)
    }

    pub struct ManifestPath {
        linux: PathBuf,
        macos: PathBuf,
        default: PathBuf,
    }

    impl ManifestPath {
        pub fn for_platform(self, default_dir: impl AsRef<Path>) -> PathBuf {
            if cfg!(target_os = "linux") {
                let base_dirs = BaseDirs::new().unwrap();
                let home_dir = base_dirs.home_dir();
                let mut ret = PathBuf::from(home_dir);
                ret.push(self.linux);
                ret
            } else if cfg!(target_os = "macos") {
                let base_dirs = BaseDirs::new().unwrap();
                let home_dir = base_dirs.home_dir();
                let mut ret = PathBuf::from(home_dir);
                ret.push(self.macos);
                ret
            } else if cfg!(target_os = "windows") {
                unimplemented!()
            } else {
                let mut ret = PathBuf::from(default_dir.as_ref());
                ret.push(self.default);
                ret
            }
        }
    }

    #[derive(Serialize)]
    pub struct Firefox {
        name: &'static str,
        description: &'static str,
        path: PathBuf,
        #[serde(rename = "type")]
        ty: &'static str,
        allowed_extensions: [&'static str; 1],
    }

    impl Firefox {
        const ALLOWED_EXTENSIONS: [&'static str; 1] = ["henrytill@gmail.com"];

        pub fn new(path: impl AsRef<Path>) -> Firefox {
            Firefox {
                name: NAME,
                description: DESCRIPTION,
                path: PathBuf::from(path.as_ref()),
                ty: TYPE,
                allowed_extensions: Firefox::ALLOWED_EXTENSIONS,
            }
        }

        pub fn path() -> ManifestPath {
            let f = file();
            let linux = [".mozilla", "native-messaging-hosts", &f];
            let macos = ["Library", "Application Support", "Mozilla", "NativeMessagingHosts", &f];
            let default = ["manifests", "mozilla", &f];
            ManifestPath {
                linux: linux.into_iter().collect::<std::path::PathBuf>(),
                macos: macos.into_iter().collect::<std::path::PathBuf>(),
                default: default.into_iter().collect::<std::path::PathBuf>(),
            }
        }
    }

    #[derive(Serialize)]

    pub struct Chromium {
        name: &'static str,
        description: &'static str,
        path: PathBuf,
        #[serde(rename = "type")]
        ty: &'static str,
        allowed_origins: [&'static str; 1],
    }

    impl Chromium {
        const ALLOWED_ORIGINS: [&'static str; 1] =
            ["chrome-extension://gebmhafgijeggbfhdojjefpibglhdjhh/"];

        pub fn new(path: impl AsRef<Path>) -> Chromium {
            Chromium {
                name: NAME,
                description: DESCRIPTION,
                path: PathBuf::from(path.as_ref()),
                ty: TYPE,
                allowed_origins: Chromium::ALLOWED_ORIGINS,
            }
        }

        pub fn path() -> ManifestPath {
            let f = file();
            let linux = [".config", "chromium", "NativeMessagingHosts", &f];
            let macos = ["Library", "Application Support", "Chromium", "NativeMessagingHosts", &f];
            let default = ["manifests", "chromium", &f];
            ManifestPath {
                linux: linux.into_iter().collect::<std::path::PathBuf>(),
                macos: macos.into_iter().collect::<std::path::PathBuf>(),
                default: default.into_iter().collect::<std::path::PathBuf>(),
            }
        }
    }
}
