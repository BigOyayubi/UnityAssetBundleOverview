use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Reference {
    path: String,
    guid: Option<Vec<u8>>,
    type_: Option<i32>,
    file_path: String,
}

impl Reference {
    pub fn new(path: String, guid: Option<Vec<u8>>, type_: Option<i32>, file_path: String) -> Reference {
        Reference{
            path: path,
            guid: guid,
            type_: type_,
            file_path: file_path,
        }
    }
}