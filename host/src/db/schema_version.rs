#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SchemaVersion(semver::Version);

impl SchemaVersion {
    pub const fn new(major: u64, minor: u64, patch: u64) -> SchemaVersion {
        SchemaVersion(semver::Version::new(major, minor, patch))
    }

    pub const CURRENT: SchemaVersion = SchemaVersion::new(0, 1, 0);

    pub fn major(&self) -> u64 {
        self.0.major
    }

    pub fn minor(&self) -> u64 {
        self.0.minor
    }

    pub fn patch(&self) -> u64 {
        self.0.patch
    }
}

impl std::fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<semver::Version> for SchemaVersion {
    fn from(version: semver::Version) -> SchemaVersion {
        SchemaVersion(version)
    }
}
