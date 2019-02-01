#![allow(non_snake_case)]

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub (crate) struct EpwingRoot {
    pub (crate) charCode: String,
    pub (crate) discCode: String,
    pub (crate) subbooks: Vec<EpwingBook>
}

#[derive(Serialize, Deserialize)]
pub (crate) struct EpwingBook {
    pub (crate) title: String,
    pub (crate) copyright: String,
    pub (crate) entries: Vec<EpwingEntry>
}

#[derive(Serialize, Deserialize)]
pub (crate) struct EpwingEntry {
    pub (crate) heading: String,
    pub (crate) text: String
}
