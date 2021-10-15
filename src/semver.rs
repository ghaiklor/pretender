use rand::{thread_rng, Rng};
use std::fmt::Display;

const MAX_MAJOR_VERSION: u8 = 5;
const MAX_MINOR_VERSION: u8 = 10;
const MAX_PATCH_VERSION: u8 = 20;

pub enum SemVerSection {
    Major,
    Minor,
    Patch,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SemVer {
    major: u8,
    minor: u8,
    patch: u8,
}

impl SemVer {
    pub fn generate() -> Self {
        let mut rng = thread_rng();

        SemVer {
            major: rng.gen_range(0..MAX_MAJOR_VERSION),
            minor: rng.gen_range(0..MAX_MINOR_VERSION),
            patch: rng.gen_range(0..MAX_PATCH_VERSION),
        }
    }

    pub fn increment(mut self, section: SemVerSection) -> Self {
        match section {
            SemVerSection::Major => self.major += 1,
            SemVerSection::Minor => self.minor += 1,
            SemVerSection::Patch => self.patch += 1,
        }

        self
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_random_version() {
        let version = SemVer::generate();

        assert!(version.major <= MAX_MAJOR_VERSION);
        assert!(version.minor <= MAX_MINOR_VERSION);
        assert!(version.patch <= MAX_PATCH_VERSION);
    }

    #[test]
    fn it_increments_version() {
        let version = SemVer::generate();
        let v0 = version.increment(SemVerSection::Patch);
        let v1 = version.increment(SemVerSection::Minor);
        let v2 = version.increment(SemVerSection::Major);

        assert_eq!(v0.patch, version.patch + 1);
        assert_eq!(v1.minor, version.minor + 1);
        assert_eq!(v2.major, version.major + 1);
    }
}
