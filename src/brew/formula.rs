use crate::brew::formula_names::FORMULA_NAMES;
use crate::semver::SemVer;
use rand::distributions::Standard;
use rand::prelude::{Distribution, SliceRandom};
use rand::random;
use sha256::digest;
use std::fmt::Display;

pub struct Formula {
    pub name: String,
    pub old_version: SemVer,
    pub new_version: SemVer,
    pub url: String,
}

impl Distribution<Formula> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Formula {
        let name = FORMULA_NAMES.choose(rng).unwrap();
        let old_version: SemVer = random();
        let new_version = old_version.upgrade_randomly();
        let sha256 = digest(name.to_owned());
        let base_url = "https://ghcr.io/v2/homebrew/core";

        Formula {
            name: name.to_string(),
            old_version,
            new_version,
            url: format!("{}/{}/blobs/sha256:{}", base_url, name, sha256),
        }
    }
}

impl Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.name, self.old_version)
    }
}
