use std::collections::HashMap;

use regex::Regex;

use crate::{prelude::*, routes::ROUTES};

pub type Context = HashMap<String, String>;
pub type RouteHandler = fn(&Request, Context) -> Result<Response>;

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

    pub fn handle(&self, rq: &Request) -> Response {
        if let Some((handler, context)) = self.get(&rq.uri) {
            match handler(rq, context) {
                Ok(response) => response,
                Err(e) => {
                    eprintln!("{e:?}");
                    rq.response(StatusCode::InternalError, None).unwrap()
                }
            }
        } else {
            rq.response(StatusCode::NotFound, None).unwrap()
        }
    }

    fn get(&self, uri: &str) -> Option<(RouteHandler, Context)> {
        let sections = uri.split("/").filter(|s| !s.is_empty()).collect::<Vec<_>>();
        let mut context = Context::new();
        let handler = self.root.get(&sections, &mut context);
        handler.map(|h| (h, context))
    }
}

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
