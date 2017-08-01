extern crate toml;
extern crate serde;

use self::serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

pub struct Config;
impl Config {
    pub fn load (path: &str) -> String {
        let file = File::open(path);
        let mut contents = String::new();
        if let Ok(mut file) = file {
            let _ = file.read_to_string(&mut contents);
        }

        contents
    }
    
    pub fn parse<'c, C: Deserialize<'c>> (contents: &'c str) -> Option<C> {
        if let Ok(config) = toml::from_str::<C>(&contents) {
            return Some(config)
        }

        None
    }
}
