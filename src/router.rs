use std::{collections::HashMap, fs, path::Path};

use regex::Regex;

use crate::prelude::*;

type Context = HashMap<String, String>;
type RouteHandler = fn(&Request, Context) -> Result<Response>;

const ROUTES: [(&str, RouteHandler); 4] = [
    ("/", home),
    (r#"/echo/{(?<message>\w+)}"#, echo),
    ("/user-agent", user_agent),
    (r#"/files/{(?<filename>[\w\-_\.]+)}"#, serve_file),
];

#[derive(Clone)]
struct Node<'a> {
    endpoint: Option<RouteHandler>,
    static_paths: HashMap<&'a str, Node<'a>>,
    pattern_paths: HashMap<&'a str, (Regex, Node<'a>)>,
}

impl<'a> Node<'a> {
    pub fn new() -> Self {
        Self {
            endpoint: None,
            static_paths: HashMap::new(),
            pattern_paths: HashMap::new(),
        }
    }

    pub fn get(&self, sections: &[&'a str], context: &mut Context) -> Option<RouteHandler> {
        if sections.is_empty() {
            self.endpoint
        } else if let Some(child) = self.static_paths.get(sections[0]) {
            child.get(&sections[1..], context)
        } else {
            for (re, child) in self.pattern_paths.values() {
                if let Some(caps) = re.captures_iter(sections[0]).next() {
                    let handler = child.get(&sections[1..], context);

                    if handler.is_some() {
                        for group in re.capture_names().flatten() {
                            context.insert(group.into(), caps[group].into());
                        }

                        return handler;
                    }
                }
            }

            None
        }
    }

    pub fn apply(&mut self, sections: &[&'a str], handler: RouteHandler) {
        if sections.is_empty() {
            self.endpoint = Some(handler);
        } else if sections[0].starts_with("{") && sections[0].ends_with("}") {
            self.apply_pattern(sections, handler);
        } else {
            self.apply_static(sections, handler);
        }
    }

    fn apply_pattern(&mut self, sections: &[&'a str], handler: RouteHandler) {
        if let Some((_, child)) = self.pattern_paths.get_mut(sections[0]) {
            child.apply(&sections[1..], handler);
        } else {
            let mut child = Node::new();
            child.apply(&sections[1..], handler);

            let pattern = &sections[0][1..sections[0].len() - 1];
            let re = Regex::new(pattern).unwrap();

            self.pattern_paths.insert(sections[0], (re, child));
        }
    }

    fn apply_static(&mut self, sections: &[&'a str], handler: RouteHandler) {
        if let Some(child) = self.static_paths.get_mut(sections[0]) {
            child.apply(&sections[1..], handler);
        } else {
            let mut child = Node::new();
            child.apply(&sections[1..], handler);
            self.static_paths.insert(sections[0], child);
        }
    }
}

#[derive(Clone)]
pub struct Router<'a> {
    root: Node<'a>,
}

impl<'a> Router<'a> {
    pub fn build() -> Self {
        let mut root = Node::new();

        for (uri, handler) in ROUTES {
            let sections = uri.split("/").filter(|s| !s.is_empty()).collect::<Vec<_>>();
            root.apply(&sections, handler);
        }

        Self { root }
    }

    pub fn handle(&self, request: &Request) -> Response {
        if let Some((handler, context)) = self.get(&request.uri) {
            match handler(request, context) {
                Ok(response) => response,
                Err(e) => {
                    eprintln!("{e:?}");
                    Response::new(
                        StatusCode::InternalError,
                        CONTENT_INTERNAL_ERROR,
                        MimeType::Html,
                    )
                }
            }
        } else {
            Response::new(StatusCode::NotFound, CONTENT_NOT_FOUND, MimeType::Html)
        }
    }

    fn get(&self, uri: &str) -> Option<(RouteHandler, Context)> {
        let sections = uri.split("/").filter(|s| !s.is_empty()).collect::<Vec<_>>();
        let mut context = Context::new();
        let handler = self.root.get(&sections, &mut context);
        handler.map(|h| (h, context))
    }
}

fn home(_: &Request, _: Context) -> Result<Response> {
    Ok(Response::new(StatusCode::Ok, CONTENT_HOME, MimeType::Html))
}

fn echo(_: &Request, cx: Context) -> Result<Response> {
    if let Some(message) = cx.get("message") {
        Ok(Response::new(StatusCode::Ok, message, MimeType::PlainText))
    } else {
        Err("Failed to get message from context.".into())
    }
}

fn user_agent(rq: &Request, _: Context) -> Result<Response> {
    if let Some(user_agent) = rq.headers.get("User-Agent") {
        Ok(Response::new(
            StatusCode::Ok,
            user_agent,
            MimeType::PlainText,
        ))
    } else {
        Err("Failed to get user agent from request headers.".into())
    }
}

fn serve_file(_: &Request, cx: Context) -> Result<Response> {
    if let Some(filename) = cx.get("filename") {
        let path = Path::new(filename);

        match fs::read_to_string(path) {
            Ok(contents) => Ok(Response::new(
                StatusCode::Ok,
                &contents,
                MimeType::OctetStream,
            )),
            Err(_) => Ok(Response::new(
                StatusCode::NotFound,
                CONTENT_NOT_FOUND,
                MimeType::Html,
            )),
        }
    } else {
        Err("Failed to get filename from context.".into())
    }
    // // Construct the full file path, if "/" the use index.html
    // let file_path = if file_path == "/" {
    //     format!("{}/index.html", ROOT_DIR)
    // } else {
    //     format!("{}/{}", ROOT_DIR, &file_path[1..])
    // };

    // let path = Path::new(&file_path);

    // // Generate the HTTP response
    // let response = match fs::read_to_string(&path) {
    //     Ok(contents) => format!(
    //         "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    //         contents.len(),
    //         contents
    //     ),
    //     Err(_) => {
    //         let not_found = "404 Not Found.";
    //         format!(
    //             "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
    //             not_found.len(),
    //             not_found
    //         )
    //     }
    // };
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
