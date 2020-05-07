#[macro_use] 
extern crate lazy_static;

use std::fs;
use std::error;
use std::process;
use std::io::{BufWriter, Write};
use log::{info, trace, warn};
use serde::{Deserialize, Serialize};

mod app;
mod args;
mod asset;
mod asset_bundle;
mod binary_reader;
mod decompress;
mod endian;
mod class_info;
mod constants;
mod object_info;
mod type_info;

use args::Args;

type Result<T> = ::std::result::Result<T, Box<dyn error::Error>>;

fn main() {
    if let Err(err) = Args::parse().and_then(try_main) {
        warn!("{}", err);
        process::exit(2);
    }
    info!("Hello, world!");
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
            warn!("{}", e);
        },
        Ok(eval) => {
            let serialized = serde_json::to_string_pretty(&eval).unwrap();
            let mut f = BufWriter::new(fs::File::create(args.dest()).unwrap());
            f.write(&serialized.as_bytes()).unwrap();
            //println!("{}", serialized);
        }
    }
   Ok(true)
}
