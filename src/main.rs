#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate rocket;

mod hashing;
mod storage;
mod transport;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                transport::http::redirect,
                transport::http::shorten,
                transport::http::delete
            ],
        )
        .launch();
}
