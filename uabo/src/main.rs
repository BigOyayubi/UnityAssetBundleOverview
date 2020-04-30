use std::error;
use std::process;

mod app;
mod args;
mod asset;
mod asset_bundle;
mod read_ext;
mod decompress;

use args::Args;

type Result<T> = ::std::result::Result<T, Box<dyn error::Error>>;

fn main() {
    if let Err(err) = Args::parse().and_then(try_main) {
        eprintln!("{}", err);
        process::exit(2);
    }
    println!("Hello, world!");
}

fn try_main(args: Args) -> Result<()> {
    use args::Command::*;

    let matched = match args.command()? {
        Files => files(&args),
    }?;

    if matched {
        process::exit(0)
    } else {
        process::exit(1)
    }
}

fn files(args: &Args) -> Result<bool> {
    match args.evaluates() {
        Err(e) => {
            eprintln!("{}", e);
        },
        Ok(evals) => {
            for e in evals {
                println!("{:?}", e);
            }
        }
    }
   Ok(true)
}
