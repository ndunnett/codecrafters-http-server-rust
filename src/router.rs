use std::collections::HashMap;

use crate::prelude::*;

type RouteHandler = fn(&Request) -> Result<Response>;

const ROUTES: [(&str, RouteHandler); 2] = [("/", home), ("/fail", fail)];

pub struct Router<'a> {
    map: HashMap<&'a str, RouteHandler>,
}

impl<'a> Router<'a> {
    pub fn build() -> Self {
        Self {
            map: HashMap::from_iter(ROUTES),
        }
    }

    pub fn handle(&self, request: &Request) -> Response {
        if let Some(route_handler) = self.map.get(request.uri.as_str()) {
            match route_handler(request) {
                Ok(response) => response,
                Err(e) => {
                    eprintln!("{e:?}");
                    Response::new(StatusCode::InternalError, CONTENT_INTERNAL_ERROR)
                }
            }
        } else {
            Response::new(StatusCode::NotFound, CONTENT_NOT_FOUND)
        }
    }
}

fn home(_request: &Request) -> Result<Response> {
    Ok(Response::new(StatusCode::Ok, CONTENT_HOME))
}

fn fail(_request: &Request) -> Result<Response> {
    Err("Test error.".into())
}

const CONTENT_HOME: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>"#;

const CONTENT_NOT_FOUND: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Oops!</title>
  </head>
  <body>
    <h1>Oops!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>"#;

const CONTENT_INTERNAL_ERROR: &str = r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Oops!</title>
  </head>
  <body>
    <h1>Oops!</h1>
    <p>Something's gone wrong.</p>
  </body>
</html>"#;
