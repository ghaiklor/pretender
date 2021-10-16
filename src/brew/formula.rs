use crate::brew::formula_names::FORMULA_NAMES;
use crate::semver::SemVer;
use rand::prelude::SliceRandom;
use rand::{random, thread_rng};
use sha256::digest;
use std::fmt::Display;

pub struct Formula {
    pub name: String,
    pub old_version: SemVer,
    pub new_version: SemVer,
    pub url: String,
}

impl Formula {
    pub fn generate() -> Self {
        let name = FORMULA_NAMES.choose(&mut thread_rng()).unwrap().to_string();
        let old_version: SemVer = random();
        let new_version = old_version.upgrade_randomly();
        let sha256 = digest(&name);

        Formula {
            name: name.to_owned(),
            old_version,
            new_version,
            url: format!(
                "https://ghcr.io/v2/homebrew/core/{}/blobs/sha256:{}",
                &name, sha256
            ),
        }
    }
}

impl Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.name, self.old_version)
    }
}
