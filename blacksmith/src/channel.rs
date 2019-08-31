use std::collections::BTreeMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Target {
    pub available: bool,
}

#[derive(Deserialize)]
pub struct Rust {
    pub version: String,
    pub target: BTreeMap<String, Target>,
}

#[derive(Deserialize)]
pub struct Packages {
    pub rust: Rust,
}

#[derive(Deserialize)]
pub struct Channel {
    pub pkg: Packages,
}

