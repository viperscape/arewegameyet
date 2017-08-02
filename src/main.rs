#[macro_use] extern crate serde_derive;

use std::collections::HashMap;

mod config;
use config::Config;

mod categories;
use categories::Categories;

mod crates;
use crates::Crate;

mod render;
use render::Render;

fn main () {
    let cats = Categories::default();

    let scripting = collect(&cats.scripting);
    println!("{:?}",scripting);

    let r = Render::from_file("./views/crates.html").expect("Cannot parse template view");
    let mut data = HashMap::new();
    data.insert("crates", scripting);
    let b = r.render(data);
}

/// Parse and collect crate info
fn collect (v: &[String]) -> Vec<Crate> {
    v.iter().map(|ref n| {
            let url = "https://crates.io/api/v1/crates/".to_owned() + &n;
            let c = Crate::from_url(&url);
            c
    }).filter(|n| n.is_some()).map(|n| n.unwrap()).collect()
}
