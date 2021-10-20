mod formula;
mod formula_names;

use self::formula::Formula;
use console::style;
use indicatif::ProgressBar;
use rand::{random, thread_rng, Rng};
use std::{thread::sleep, time::Duration};

pub struct BrewPretender {
    formulas: Vec<Formula>,
}

impl Default for BrewPretender {
    fn default() -> Self {
        let formulas = (10..20).map(|_| random()).collect();

        BrewPretender { formulas }
    }
}

impl BrewPretender {
    pub fn pretend(self) {
        let mut rng = thread_rng();

        println!("Updating Homebrew...");
        sleep(Duration::from_secs(rng.gen_range(5..10)));

        println!(
            "{} Upgrading {} outdated packages:",
            style("==>").green(),
            self.formulas.len()
        );

        for formula in &self.formulas {
            println!(
                "{} {} -> {}",
                formula.name, formula.old_version, formula.new_version
            )
        }

        for formula in &self.formulas {
            println!(
                "{} Upgrading {} {} -> {}",
                style("==>").blue(),
                style(formula.name.to_owned()).blue(),
                formula.old_version,
                formula.new_version
            );

            println!("{} Downloading {}", style("==>").green(), formula.url);

            let size = thread_rng().gen_range(20..100) * 1024;
            let pb = ProgressBar::new(size);

            while pb.position() < size {
                pb.inc(thread_rng().gen_range(50..200));
                sleep(Duration::from_millis(100));
            }
            pb.finish_and_clear();
        }
    }
}
