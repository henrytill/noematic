use std::{
    convert::Infallible,
    fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Semver(semver::Error),
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Self::Io(other)
    }
}

impl From<semver::Error> for Error {
    fn from(other: semver::Error) -> Self {
        Self::Semver(other)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SchemaVersion(semver::Version);

impl SchemaVersion {
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        SchemaVersion(semver::Version::new(major, minor, patch))
    }

    pub fn parse(version: &str) -> Result<Self, semver::Error> {
        let version = semver::Version::parse(version)?;
        Ok(SchemaVersion(version))
    }

    pub const CURRENT: Self = SchemaVersion::new(0, 1, 0);
}

impl ToString for SchemaVersion {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<semver::Version> for SchemaVersion {
    fn from(version: semver::Version) -> Self {
        SchemaVersion(version)
    }
}

pub trait SchemaVersioner {
    type Error;

    /// Read the schema version from the versioner. If the versioner has not
    /// been written to, this should return `Ok(None)`.
    fn read(&mut self) -> Result<Option<SchemaVersion>, Self::Error>;

    /// Write the schema version to the versioner.
    fn write(&mut self, version: &SchemaVersion) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub struct InMemorySchemaVersioner {
    version: Option<SchemaVersion>,
}

impl InMemorySchemaVersioner {
    pub const fn new(version: Option<SchemaVersion>) -> Self {
        Self { version }
    }
}

impl Default for InMemorySchemaVersioner {
    fn default() -> Self {
        InMemorySchemaVersioner::new(None)
    }
}

impl SchemaVersioner for InMemorySchemaVersioner {
    type Error = Infallible;

    fn read(&mut self) -> Result<Option<SchemaVersion>, Self::Error> {
        Ok(self.version.to_owned())
    }

    fn write(&mut self, version: &SchemaVersion) -> Result<(), Self::Error> {
        self.version = Some(version.to_owned());
        Ok(())
    }
}

#[derive(Debug)]
pub struct PersistentSchemaVersioner {
    /// Path to the file containing the schema version.
    path: PathBuf,
}

impl PersistentSchemaVersioner {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        Self { path }
    }
}

impl SchemaVersioner for PersistentSchemaVersioner {
    type Error = Error;

    fn read(&mut self) -> Result<Option<SchemaVersion>, Self::Error> {
        if !self.path.exists() {
            return Ok(None);
        }
        let contents = fs::read_to_string(&self.path)?;
        let version = SchemaVersion::parse(&contents)?;
        Ok(Some(version))
    }

    fn write(&mut self, version: &SchemaVersion) -> Result<(), Self::Error> {
        fs::write(&self.path, version.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use tempfile;

    use super::*;

    #[test]
    fn test_in_memory_schema_versioner() {
        let mut versioner = InMemorySchemaVersioner::default();

        let actual = versioner.read().expect("read failed");
        assert!(actual.is_none());

        let expected = SchemaVersion::new(0, 0, 1);
        versioner.write(&expected).expect("write failed");
        let actual = versioner.read().expect("read failed");
        assert_eq!(Some(expected), actual);
    }

    #[test]
    fn test_persistent_schema_versioner() {
        let tempdir = tempfile::tempdir().expect("tempdir failed");
        let path = tempdir.path().join("schema");
        let mut versioner = PersistentSchemaVersioner::new(&path);
        assert_eq!(path, versioner.path);
        assert!(!path.exists());

        let actual = versioner.read().expect("read failed");
        assert!(actual.is_none());
        assert!(!path.exists());

        let expected = SchemaVersion::new(0, 0, 1);
        versioner.write(&expected).expect("write failed");
        assert!(path.exists());
        let actual = versioner.read().expect("read failed");
        assert_eq!(Some(expected), actual);
    }
}
