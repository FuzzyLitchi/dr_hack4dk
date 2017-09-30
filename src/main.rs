#![feature(const_fn)]
#[macro_use]
extern crate serde_derive;

extern crate serde_json;
#[macro_use]
extern crate tantivy;
extern crate tempdir;
extern crate hyper;
extern crate futures;

mod data;
mod searching;
mod server;

use server::Server;

fn main() {
    Server::serve();
    panic!("it ended");
}
