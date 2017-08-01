extern crate mustache;

use self::mustache::Template;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub struct Render(Template);

impl Render {
    pub fn from_file(path: &str) -> Option<Render> {
        let file = File::open(path);
        let mut contents = String::new();
        if let Ok(mut file) = file {
            let _ = file.read_to_string(&mut contents);

            if let Ok(t) = mustache::compile_str(&contents) {
                return Some(Render(t))
            }
        }
        
        None
    }

    pub fn render(&self) -> Vec<u8> {
        let data: HashMap<String,String> = HashMap::new();
        let mut bytes = vec![];
        let _ = self.0.render(&mut bytes, &data);
        bytes
    }
}
