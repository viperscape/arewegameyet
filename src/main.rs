#[macro_use] extern crate serde_derive;

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

    for n in cats.scripting {
        let url = "https://crates.io/api/v1/crates/".to_owned() + &n;
        let c = Crate::from_url(&url);
        println!("{:?}",c);
    }

    let r = Render::from_file("./views/crates.html").expect("Cannot parse template view");
    let _b = r.render();
}
