#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::{Stream};

use std::fs::{File, OpenOptions};
use std::io::Write;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/img/<name>")]
fn send_img(name: String) -> Option<Stream<File>> {
    write_log(&name);
    println!("{}", name);
    File::open("./img/white.jpg").map(|file| Stream::from(file)).ok()
}

fn write_log(desc: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .append(true)
        .open("log.log")
        .unwrap();
    file.write_all(desc.as_bytes()).expect("Unable to write data");
}

fn main() {
    rocket::ignite().mount("/", routes![index, send_img]).launch();
}