use std::path::{Path, PathBuf};
use std::sync::Arc;
use crate::app;
use crate::asset_bundle::AssetBundle;
use crate::Result;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command {
    Files,
}

#[derive(Clone, Debug)]
pub struct Args(Arc<ArgsImp>);

#[derive(Clone, Debug)]
struct ArgsImp {
    srcs: Vec<PathBuf>,
}

impl Args {
    pub fn parse() -> Result<Args> {
        let matches = app::app().get_matches();
        let mut pats = vec![];
        match matches.values_of_os("srcs") {
            None => {

            },
            Some(srcs) => {
                for src in srcs {
                    pats.push( Path::new(src).to_path_buf() );
                }
            }
        }
        Ok(Args(Arc::new(ArgsImp{
            srcs: pats
        })))
    }

    pub fn command(&self) -> Result<Command>{
        Ok(Command::Files)
    }

    pub fn evaluates(&self) -> Result<Vec<AssetBundle>>{
        let mut asset_bundles = vec![];
        for src in &self.0.srcs {
            asset_bundles.push( AssetBundle::load(src.to_path_buf())? );
        }
        Ok(asset_bundles)
    }
}
