use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{self, Path},
};

use crate::{
    prelude::*,
    router::{Context, RouteHandler},
};

pub const ROUTES: [(&str, RouteHandler); 4] = [
    ("/", home),
    (r#"/echo/{(?<message>\w+)}"#, echo),
    ("/user-agent", user_agent),
    (r#"/files/{(?<filename>[\w\-_\.]+)}"#, files),
];

fn method_guard(rq: &Request, methods: &[Method]) -> Option<Result<Response>> {
    if methods.contains(&rq.method) {
        None
    } else {
        Some(rq.response(StatusCode::MethodNotAllowed, None))
    }
}

fn path_guard(rq: &Request, path: &Path) -> Result<Option<Result<Response>>> {
    if path::absolute(path)?.starts_with(env::current_dir()?) {
        Ok(None)
    } else {
        Ok(Some(rq.response(StatusCode::Forbidden, None)))
    }
}

fn home(rq: &Request, _: Context) -> Result<Response> {
    if let Some(reponse) = method_guard(rq, &[Method::Get]) {
        return reponse;
    }

    rq.response(StatusCode::Ok, None)
}

fn echo(rq: &Request, cx: Context) -> Result<Response> {
    if let Some(reponse) = method_guard(rq, &[Method::Get]) {
        return reponse;
    }

    if let Some(message) = cx.get("message") {
        let content = Content::new(MimeType::PlainText, message);
        rq.response(StatusCode::Ok, Some(content))
    } else {
        Err("Failed to get message from context.".into())
    }
}

fn user_agent(rq: &Request, _: Context) -> Result<Response> {
    if let Some(reponse) = method_guard(rq, &[Method::Get]) {
        return reponse;
    }

    if let Some(user_agent) = rq.headers.get("User-Agent") {
        let content = Content::new(MimeType::PlainText, user_agent);
        rq.response(StatusCode::Ok, Some(content))
    } else {
        Err("Failed to get user agent from request headers.".into())
    }
}

fn files(rq: &Request, cx: Context) -> Result<Response> {
    match rq.method {
        Method::Get => serve_file(rq, cx),
        Method::Post => upload_file(rq, cx),
    }
}

fn serve_file(rq: &Request, cx: Context) -> Result<Response> {
    let path = Path::new(cx.get("filename").ok_or(Error::Generic(
        "Failed to get filename from context.".into(),
    ))?);

    if !path.exists() {
        return rq.response(StatusCode::NotFound, None);
    }

    if let Some(reponse) = path_guard(rq, path)? {
        return reponse;
    }

    let file = fs::read_to_string(path)?;
    let content = Content::new(MimeType::OctetStream, &file);
    rq.response(StatusCode::Ok, Some(content))
}

fn upload_file(rq: &Request, cx: Context) -> Result<Response> {
    let path = Path::new(cx.get("filename").ok_or(Error::Generic(
        "Failed to get filename from context.".into(),
    ))?);

    if let Some(reponse) = path_guard(rq, path)? {
        return reponse;
    }

    let bytes = if let Some(content) = &rq.content {
        content.body.as_bytes()
    } else {
        return Err("Request did not contain content.".into());
    };

    let mut file = File::create(path)?;
    file.write_all(bytes)?;
    rq.response(StatusCode::Created, None)
}
