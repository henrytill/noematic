#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SchemaVersion(semver::Version);

impl SchemaVersion {
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        SchemaVersion(semver::Version::new(major, minor, patch))
    }

    pub const CURRENT: Self = SchemaVersion::new(0, 1, 0);

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
