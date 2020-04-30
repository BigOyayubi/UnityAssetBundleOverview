extern crate clap;

use clap::{crate_authors, crate_version, App, AppSettings};

pub fn app() -> App<'static, 'static> {
    let app = App::new("uabo")
    .author(crate_authors!())
    .version(crate_version!())
    .about("TODO")
    .setting(AppSettings::UnifiedHelpMessage)
    .setting(AppSettings::AllArgsOverrideSelf)
    .usage("TODO")
    .arg(
        clap::Arg::with_name("srcs")
        .help("asset bundle pathes")
        .required(true)
        .multiple(true)
    );

    app
}
