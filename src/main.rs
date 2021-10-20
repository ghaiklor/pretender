use brew::BrewPretender;
use cli::build_app;
use core::panic;

mod brew;
mod cli;
mod semver;

fn main() {
    let pretender_app = build_app();
    let matches = pretender_app.get_matches();
    let app_to_pretend = matches.value_of("app").unwrap();

    match app_to_pretend {
        "brew" => BrewPretender::default().pretend(),
        _ => panic!("No such application to pretend"),
    }
}
