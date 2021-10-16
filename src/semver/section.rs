use rand::{distributions::Standard, prelude::Distribution};

pub enum Section {
    Major,
    Minor,
    Patch,
}

impl Distribution<Section> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Section {
        match rng.gen_range(0..=2) {
            0 => Section::Major,
            1 => Section::Minor,
            _ => Section::Patch,
        }
    }
}
