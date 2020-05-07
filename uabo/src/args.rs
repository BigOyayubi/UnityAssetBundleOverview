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
    src: PathBuf,
    dst: PathBuf,
}

impl Args {
    pub fn parse() -> Result<Args> {
        let matches = app::app().get_matches();
        let src = Path::new(matches.value_of("src").unwrap());
        let dst = Path::new(matches.value_of("dst").unwrap());
        Ok(Args(Arc::new(ArgsImp{
            src: src.to_path_buf(),
            dst: dst.to_path_buf(),
        })))
    }

    pub fn command(&self) -> Result<Command>{
        Ok(Command::Files)
    }

    pub fn evaluates(&self) -> Result<AssetBundle>{
        AssetBundle::load(&self.0.src)
    }

    pub fn dest(&self) -> String {
        String::from(self.0.dst.to_str().unwrap())
    }
}
