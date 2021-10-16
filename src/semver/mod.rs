mod section;

use self::section::Section;
use rand::{distributions::Standard, prelude::Distribution, random, Rng};
use std::fmt::Display;

const MAX_MAJOR_VERSION: u8 = 5;
const MAX_MINOR_VERSION: u8 = 10;
const MAX_PATCH_VERSION: u8 = 20;

pub struct SemVer {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Distribution<SemVer> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SemVer {
        let major = rng.gen_range(0..MAX_MAJOR_VERSION);
        let minor = rng.gen_range(0..MAX_MINOR_VERSION);
        let patch = rng.gen_range(0..MAX_PATCH_VERSION);

        SemVer {
            major,
            minor,
            patch,
        }
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl SemVer {
    fn major_upgrade(&self) -> Self {
        Self {
            major: self.major + 1,
            minor: 0,
            patch: 0,
        }
    }

    fn minor_upgrade(&self) -> Self {
        Self {
            major: self.major,
            minor: self.minor + 1,
            patch: 0,
        }
    }

    fn patch_upgrade(&self) -> Self {
        Self {
            major: self.major,
            minor: self.minor,
            patch: self.patch + 1,
        }
    }

    pub fn upgrade(&self, section: Section) -> Self {
        match section {
            Section::Major => self.major_upgrade(),
            Section::Minor => self.minor_upgrade(),
            Section::Patch => self.patch_upgrade(),
        }
    }

    pub fn upgrade_randomly(&self) -> Self {
        self.upgrade(random())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;

    #[test]
    fn it_creates_random_version() {
        let version: SemVer = random();

        assert!(version.major <= MAX_MAJOR_VERSION);
        assert!(version.minor <= MAX_MINOR_VERSION);
        assert!(version.patch <= MAX_PATCH_VERSION);
    }

    #[test]
    fn it_creates_major_upgrade() {
        let version: SemVer = random();
        let upgrade = version.upgrade(Section::Major);

        assert_eq!(upgrade.major, version.major + 1);
        assert_eq!(upgrade.minor, 0);
        assert_eq!(upgrade.patch, 0);
    }

    #[test]
    fn it_creates_minor_upgrade() {
        let version: SemVer = random();
        let upgrade = version.upgrade(Section::Minor);

        assert_eq!(upgrade.major, version.major);
        assert_eq!(upgrade.minor, version.minor + 1);
        assert_eq!(upgrade.patch, 0);
    }

    #[test]
    fn it_creates_patch_upgrade() {
        let version: SemVer = random();
        let upgrade = version.upgrade(Section::Patch);

        assert_eq!(upgrade.major, version.major);
        assert_eq!(upgrade.minor, version.minor);
        assert_eq!(upgrade.patch, version.patch + 1);
    }
}
