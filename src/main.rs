#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod config;

use rocket::config::{Config, Environment};
use rocket::response::{content, Stream};
use rocket_contrib::json::{self, Json, JsonValue};

use uuid::Uuid;

use serde_json::{Result, Value};

use chrono::prelude::*;

use crossterm::{
    event, execute,
    style::{Colorize, Styler},
    ExecutableCommand,
};

use std::fs::{self, File, OpenOptions};
use std::io::{stdout, Read, Write};
use std::path::Path;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/new")]
fn new() -> JsonValue {
    let new_uuid = Uuid::new_v4();
    let mut file = OpenOptions::new()
        .write(true)
        .open("data/data.json")
        .unwrap();
    let res: JsonValue = rocket_contrib::json!({
        "uuid": new_uuid,
    });
    let mut ori_data = fs::read_to_string("data/data.json").unwrap();
    let mut ov: Value = serde_json::from_str(&ori_data[..]).unwrap();
    let mut v = ov.as_object_mut().unwrap();
    v.insert(new_uuid.to_string(), Value::String("false".to_string()));
    file.write_all(&serde_json::to_string(&v).unwrap()[..].as_bytes());
    res
}

#[get("/<uuid>")]
fn get_info(uuid: String) -> String {
    let data_file = fs::read_to_string("data/data.json").unwrap();
    let data: Value = serde_json::from_str(&data_file[..]).unwrap();
    format!("{}", data[uuid])
}

#[get("/img/<uuid>")]
fn send_img(uuid: String) -> Option<Stream<File>> {
    // write_log(&uuid);
    let data_file = fs::read_to_string("data/data.json").unwrap();
    let mut data: Value = serde_json::from_str(&data_file[..]).unwrap();
    let now: String = Utc::now().to_rfc2822();
    data[uuid.clone()] = Value::String(now.clone());
    OpenOptions::new()
        .write(true)
        .open("data/data.json")
        .unwrap()
        .write_all(&serde_json::to_string(&data).unwrap()[..].as_bytes());
    println!(
        "    {} {} : {}",
        "=>".bold(),
        uuid.as_str().bold().dark_yellow(),
        now.as_str().bold()
    );
    File::open("./img/transparent.png")
        .map(|file| Stream::from(file))
        .ok()
}

fn write_log(desc: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("log.log")
        .unwrap();
    file.write_all(desc.as_bytes())
        .expect("Unable to write data");
}

fn main() {
    if !Path::new("data").exists() {
        fs::create_dir("data");
    }
    if !Path::new("data/data.json").exists() {
        let mut data_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open("data/data.json")
            .unwrap();
        data_file.write_all(b"{}");
    }
    config::Config::create_config();
    let config = config::Config::read_config();
    let configs = Config::build(Environment::Staging)
        .port(config.unwrap().port)
        .unwrap();
    rocket::custom(configs)
        .mount("/", routes![index, new, send_img, get_info])
        .launch();
}
