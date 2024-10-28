use std::{io::Write, net::TcpListener};

use crate::{prelude::*, router::Router};

pub struct Server {}

impl Server {
    pub fn run(host: &str, port: u32) -> Result<()> {
        let listener = TcpListener::bind(format!("{host}:{port}"))?;
        let router = Router::build();

        for stream in listener.incoming() {
            println!("{:-<30}", "");

            let mut stream = stream?;

            match Request::parse(&mut stream) {
                Ok(request) => {
                    println!("{request}");
                    let response = router.handle(&request);
                    println!("{response}");
                    stream.write_all(response.as_bytes())?;
                }
                Err(e) => {
                    eprintln!("{e:?}");
                }
            }
        }

        Ok(())
    }
}
