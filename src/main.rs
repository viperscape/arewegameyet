#[macro_use] extern crate serde_derive;

mod config;
use config::Config;

mod categories;
use categories::Categories;

mod crates;
use crates::Crate;

fn main () {
    let cats = Categories::default();

    for n in cats.scripting {
        let url = "https://crates.io/api/v1/crates/".to_owned() + &n;
        let c = Crate::from_url(&url);
        println!("{:?}",c);
    }
}
