extern crate reqwest;
extern crate serde_json;



use self::serde_json::Value;
use std::io::Read;

#[derive(Debug,RustcEncodable)]
pub struct Crate {
    name: String,
    ver: String,
    desc: String,
    lic: String,
    repo: String
}

impl Crate {
    pub fn from_url (path: &str) -> Option<Crate> {
        if let Ok(mut rsp) = reqwest::get(path) {
            if rsp.status().is_success() {
                let mut c = String::new();
                let _ = rsp.read_to_string(&mut c);
                if let Ok(r) = serde_json::from_str::<Value>(&c) {
                    return Crate::from_json(r)
                }
            }
        }

        None
    }

    pub fn from_json (json: Value) -> Option<Crate> {
        // Note: Crates v1 api displays versions, the first is the latest crate info
        if let Some(v) = json.get("versions").and_then(|n| n.get(0)) {
            let mut name = String::new();
            match v.get("crate") {
                Some(&Value::String(ref s)) => { name = s.to_owned(); },
                _ => {}
            }

            let mut ver = String::new();
            match v.get("num") {
                Some(&Value::String(ref s)) => { ver = s.to_owned(); },
                _ => {}
            }

            let mut lic = String::new();
            match v.get("license") {
                Some(&Value::String(ref s)) => { lic = s.to_owned(); },
                _ => {}
            }

            if let Some(v) = json.get("crate") {
                let mut repo = String::new();
                match v.get("repository") {
                    Some(&Value::String(ref s)) => { repo = s.to_owned(); },
                    _ => {}
                }

                let mut desc = String::new();
                match v.get("description") {
                    Some(&Value::String(ref s)) => { desc = s.to_owned(); },
                    _ => {}
                }

                let c = Crate {
                    repo: repo,
                    ver: ver,
                    lic: lic,
                    name: name,
                    desc: desc,
                };

                return Some(c)
            }
        }
        
        None
    }
}
