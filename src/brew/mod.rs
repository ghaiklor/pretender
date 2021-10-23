mod formula;
mod formula_names;

use self::formula::Formula;
use console::{style, Emoji};
use indicatif::{ProgressBar, ProgressStyle};
use rand::{random, thread_rng, Rng};
use std::{thread::sleep, time::Duration};

pub struct BrewPretender {
    formulas: Vec<Formula>,
}

impl Default for BrewPretender {
    fn default() -> Self {
        let formulas = (0..thread_rng().gen_range(10..30))
            .map(|_| random())
            .collect();

        BrewPretender { formulas }
    }
}

impl BrewPretender {
    fn prelude(&self) {
        let mut rng = thread_rng();

        println!("Updating Homebrew...");
        sleep(Duration::from_secs(rng.gen_range(5..10)));

        println!(
            "{} {}",
            style("==>").blue(),
            style("Auto-updated Homebrew!").bold()
        );

        println!("Updated 1 tap (homebrew/core).");
        println!(
            "{} {}",
            style("==>").blue(),
            style("Updated Formulae").bold()
        );

        println!("Updated {} formulae.", rng.gen_range(100..300));
        println!();
    }

    fn upgrading_n_packages(&self) {
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
    }

    fn upgrading_formula(&self, formula: &Formula) {
        let mut rng = thread_rng();

        println!(
            "{} Upgrading {}",
            style("==>").green(),
            style(formula.name.to_owned()).green(),
        );

        println!("  {} -> {}", formula.old_version, formula.new_version);
        println!();

        println!(
            "{} Pouring {}--{}.big_sur.bottle.tar.gz",
            style("==>").blue(),
            formula.name,
            formula.new_version
        );

        sleep(Duration::from_secs(rng.gen_range(1..5)));

        let files_count = rng.gen_range(10..300);
        let size = rng.gen_range(2..10);

        println!(
            "{} /usr/local/Cellar/{}/{}: {} files, {}.0MB",
            Emoji::new("🍺", ""),
            formula.name,
            formula.new_version,
            files_count,
            size
        );

        println!(
            "Removing: /usr/local/Cellar/{}/{}... ({} files, {}.0MB)",
            formula.name, formula.old_version, files_count, size
        );

        sleep(Duration::from_secs(rng.gen_range(1..5)));
    }

    fn downloading_formula(&self, formula: &Formula) {
        let mut rng = thread_rng();

        println!("{} Downloading {}", style("==>").blue(), formula.url);

        let size = rng.gen_range(20..100) * 1024 * 1024;
        let style = ProgressStyle::default_bar()
            .template("{bar:72} {percent}.0%")
            .progress_chars("## ");

        let bar = ProgressBar::new(size).with_style(style);

        while bar.position() < size {
            bar.inc(rng.gen_range(10..500) * 1024);
            sleep(Duration::from_millis(rng.gen_range(5..100)));
        }

        bar.finish();
    }

    pub fn pretend(self) {
        self.prelude();
        self.upgrading_n_packages();

        for formula in &self.formulas {
            self.downloading_formula(formula);
        }

        for formula in &self.formulas {
            self.upgrading_formula(formula);
        }
    }
}
