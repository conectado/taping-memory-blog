use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Articles {
    pub articles: Vec<String>,
}
