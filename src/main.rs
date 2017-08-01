#[macro_use] extern crate serde_derive;

mod config;
use config::Config;

mod categories;
use categories::Categories;

mod crates;
use crates::Crate;

fn main () {
    let c = Categories::default();
    let lichen = Crate::from_url("https://crates.io/api/v1/crates/lichen");

    println!("{:?}",lichen);
}
