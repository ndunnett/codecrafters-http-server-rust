#![allow(dead_code)]
mod error;
mod prelude;
mod request;
mod response;
mod router;
mod server;

use std::env;

use crate::{prelude::*, server::Server};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if let Some(cmd) = args.get(1) {
        if cmd == "--directory" {
            env::set_current_dir(args.get(2).unwrap()).unwrap();
        }
    }

    if let Err(e) = Server::run("127.0.0.1", 4221) {
        eprintln!("{e}");
        std::process::exit(1)
    } else {
        std::process::exit(0)
    }
}
