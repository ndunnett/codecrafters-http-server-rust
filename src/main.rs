#![allow(dead_code)]
mod error;
mod prelude;
mod request;
mod response;
mod router;
mod server;

use crate::{prelude::*, server::Server};

fn main() -> Result<()> {
    if let Err(e) = Server::run("127.0.0.1", 4221) {
        eprintln!("{e}");
        std::process::exit(1)
    } else {
        std::process::exit(0)
    }
}
