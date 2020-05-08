extern crate clap;

use clap::{crate_authors, crate_version, App, AppSettings};

pub fn app() -> App<'static, 'static> {
    let app = App::new("uabo")
    .author(crate_authors!())
    .version(crate_version!())
    .about("Unity AssetBundle Deserialize Tool")
    .setting(AppSettings::UnifiedHelpMessage)
    .setting(AppSettings::AllArgsOverrideSelf)
    .usage("uabo --src /path/to/foo.unity3d --dst /path/to/baa.json")
    .arg(
        clap::Arg::with_name("src")
        .help("asset bundle pathe")
        .short("s")
        .long("src")
        .takes_value(true)
        .required(true)
    ).arg(
        clap::Arg::with_name("dst")
        .help("dst path")
        .short("d")
        .long("dst")
        .takes_value(true)
        .required(true)
    );
    app
}
