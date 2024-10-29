use std::{env, fs, io::Write, net::TcpListener, path::PathBuf, thread};

use crate::{prelude::*, router::Router};

pub struct Server {
    pub cwd: Option<PathBuf>,
    pub host: String,
    pub port: u32,
}

impl Server {
    pub fn new() -> Result<Self> {
        let mut server = Self {
            cwd: None,
            host: "127.0.0.1".into(),
            port: 4221,
        };

        let args = env::args().zip(env::args().skip(1));

        for (a, b) in args {
            match (a.as_str(), b.as_str()) {
                ("--directory", path) => {
                    server.cwd = Some(path.into());
                }
                ("--host", host) => {
                    server.host = host.to_string();
                }
                ("--port", port) => {
                    server.port = port.parse::<u32>()?;
                }
                _ => {}
            }
        }

        Ok(server)
    }

    pub fn run(&self) -> Result<()> {
        if let Some(cwd) = &self.cwd {
            if !cwd.exists() {
                fs::create_dir_all(cwd)?;
            }

            env::set_current_dir(cwd)?;
        }

        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port))?;
        let router = Router::build();

        for stream in listener.incoming() {
            let router = router.clone();

            thread::spawn(move || {
                println!("{:-<30}", "");
                let mut stream = stream.unwrap();

                let request = Request::parse(&mut stream).unwrap();
                println!("{request}");

                let response = router.handle(&request);
                println!("{response}");

                stream.write_all(&response.to_bytes()).unwrap();
            });
        }

        Ok(())
    }
}
