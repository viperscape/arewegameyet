#[derive(Deserialize,Debug)]
pub struct Categories {
    pub rendering_2d: Vec<String>,
    pub scripting: Vec<String>
}


impl Default for Categories {
    fn default () -> Categories {
        let c = ::Config::load("./config/categories.toml");
        ::Config::parse(&c).unwrap_or(Categories { rendering_2d: vec![], scripting: vec![] })
    }
}
