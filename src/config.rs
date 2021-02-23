use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub domain: String,
    pub port: u16,
}

impl Config {
    pub fn create_config() {
        if Path::new("config.json").exists() {
            return;
        }
        println!("This app need domain and port!");
        io::stdout().flush();
        print!("{0}: ", "domain");
        io::stdout().flush();
        let mut domain: String = String::new();
        io::stdin().read_line(&mut domain);
        print!(
            "{0}: ",
            "port
        "
        );
        io::stdout().flush();
        let mut port_string: String = String::new();
        io::stdin().read_line(&mut port_string);
        let mut port_string = port_string.replace("\n", "");
        let mut domain = domain.replace("\n", "");
        let port = port_string.parse::<u16>().unwrap();
        let config = Config {
            domain: domain,
            port: port,
        };
        let json = serde_json::to_string(&config).unwrap();
        let json_str: &str = &json[..];
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open("config.json")
            .unwrap();
        file.write_all(json_str.as_bytes())
            .expect("Unable to write file");
    }

    pub fn read_config() -> Result<Config> {
        let content = fs::read_to_string("config.json").unwrap();
        let config: Config = serde_json::from_str(&content[..]).unwrap();
        Ok(config)
    }
}
