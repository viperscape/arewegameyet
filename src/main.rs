#[macro_use] extern crate serde_derive;

mod config;
use config::Config;

mod categories;
use categories::Categories;

fn main () {
    let c = Categories::default();
    println!("{:?}",c);
}
