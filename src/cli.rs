use clap::{crate_authors, crate_version, App, Arg};

pub fn build_app() -> App<'static, 'static> {
    App::new("Pretender")
        .author(crate_authors!())
        .version(crate_version!())
        .long_version(crate_version!())
        .name("pretender")
        .about("CLI to help you pretend your machine does upgrades, compilation, docker pulling or whatever")
        .arg(Arg::with_name("app").required(true).possible_value("brew"))
}
