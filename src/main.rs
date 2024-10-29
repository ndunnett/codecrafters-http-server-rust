#![allow(dead_code)]
mod error;
mod prelude;
mod request;
mod response;
mod router;
mod routes;
mod server;

use crate::prelude::*;

fn run() -> Result<()> {
    server::Server::new()?.run()
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    } else {
        std::process::exit(0)
    }
}
