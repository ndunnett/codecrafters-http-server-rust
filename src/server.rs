use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

use crate::{prelude::*, router::Router};

pub struct Server {}

impl Server {
    pub fn run(host: &str, port: u32) -> Result<()> {
        let listener = TcpListener::bind(format!("{host}:{port}"))?;
        let router = Router::build();

        for stream in listener.incoming() {
            println!("{:-<30}", "");

            let mut stream = stream?;

            let reader = BufReader::new(&mut stream);
            let s = reader
                .lines()
                .map_while(|line| line.ok())
                .take_while(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join("\r\n");

            let request = Request::parse(&s)?;
            println!("{request}");

            let response = router.handle(&request);
            println!("{response}");

            stream.write_all(response.as_bytes())?;
        }

        Ok(())
    }
}
