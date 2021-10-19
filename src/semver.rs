use rand::{distributions::Standard, prelude::Distribution, random};
use semver::Version;
use std::fmt::Display;

pub enum SemVerSection {
    Major,
    Minor,
    Patch,
}

pub struct SemVer {
    version: Version,
}

impl Distribution<SemVer> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> SemVer {
        let major = rng.gen_range(0..5);
        let minor = rng.gen_range(0..10);
        let patch = rng.gen_range(0..20);

        SemVer {
            version: Version::new(major, minor, patch),
        }
    }
}

impl Distribution<SemVerSection> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> SemVerSection {
        match rng.gen_range(0..2) {
            0 => SemVerSection::Major,
            1 => SemVerSection::Minor,
            _ => SemVerSection::Patch,
        }
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.version)
    }
}

impl SemVer {
    fn major_upgrade(&self) -> Self {
        Self {
            version: Version::new(self.version.major + 1, 0, 0),
        }
    }

    fn minor_upgrade(&self) -> Self {
        Self {
            version: Version::new(self.version.major, self.version.minor + 1, 0),
        }
    }

    fn patch_upgrade(&self) -> Self {
        Self {
            version: Version::new(
                self.version.major,
                self.version.minor,
                self.version.patch + 1,
            ),
        }
    }

    pub fn upgrade(&self, section: SemVerSection) -> Self {
        match section {
            SemVerSection::Major => self.major_upgrade(),
            SemVerSection::Minor => self.minor_upgrade(),
            SemVerSection::Patch => self.patch_upgrade(),
        }
    }

    pub fn upgrade_randomly(&self) -> Self {
        self.upgrade(random())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_major_upgrade() {
        let version = random::<SemVer>();
        let upgrade = version.upgrade(SemVerSection::Major);

        assert_eq!(upgrade.version.major, version.version.major + 1);
        assert_eq!(upgrade.version.minor, 0);
        assert_eq!(upgrade.version.patch, 0);
    }

    #[test]
    fn it_creates_minor_upgrade() {
        let version = random::<SemVer>();
        let upgrade = version.upgrade(SemVerSection::Minor);

        assert_eq!(upgrade.version.major, version.version.major);
        assert_eq!(upgrade.version.minor, version.version.minor + 1);
        assert_eq!(upgrade.version.patch, 0);
    }

    #[test]
    fn it_creates_patch_upgrade() {
        let version = random::<SemVer>();
        let upgrade = version.upgrade(SemVerSection::Patch);

        assert_eq!(upgrade.version.major, version.version.major);
        assert_eq!(upgrade.version.minor, version.version.minor);
        assert_eq!(upgrade.version.patch, version.version.patch + 1);
    }
}
