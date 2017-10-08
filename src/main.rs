#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[cfg(test)]
mod tests;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/about")]
fn about() -> io::Result<NamedFile> {
    NamedFile::open("static/about.html")
}

#[get("/rebuilding")]
fn rebuilding() -> io::Result<NamedFile> {
    NamedFile::open("static/contracting.html")
}

#[get("/preparation")]
fn preparation() -> io::Result<NamedFile> {
    NamedFile::open("static/preparation.html")
}

#[get("/contact")]
fn contact() -> io::Result<NamedFile> {
    NamedFile::open("static/contact.html")
}

#[get("/services")]
fn services() -> io::Result<NamedFile> {
    NamedFile::open("static/services.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}


fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, about, rebuilding, preparation, contact, services, files])
}

fn main() {
    rocket().launch();
}