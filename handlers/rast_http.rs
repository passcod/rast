#![crate_id = "rast_http#0.1"]

//! # Rast - Rust-Http Handler
//!
//! _"It's not very pretty, but it works."_
//!
//! - https://github.com/passcod/rast
//! - https://github.com/chris-morgan/rust-http

#![crate_type = "lib"]
#![comment = "Rast handler for rust-http"]
#![license = "Public Domain"]

extern crate http = "http#0.1-pre";
extern crate rast = "rast#0.4";
extern crate url;
use http::headers::HeaderEnum;
use http::headers::content_type::MediaType;
use http::server::{Config, Server, request, Request, ResponseWriter};
use http::status::Status;
use rast::{Handler, App, memstream};
use std::collections::HashMap;
use std::io;
use std::string::String;

/// Translates rust-http methods into rast ones.
///
/// Extension methods are not supported, and will
/// default to `Get`.
fn http_method_to_rast(method: http::method::Method) -> rast::http::Method {
  match method {
    http::method::Options => rast::http::Options,
    http::method::Get     => rast::http::Get,
    http::method::Head    => rast::http::Head,
    http::method::Post    => rast::http::Post,
    http::method::Put     => rast::http::Put,
    http::method::Delete  => rast::http::Delete,
    http::method::Trace   => rast::http::Trace,
    http::method::Connect => rast::http::Connect,
    http::method::Patch   => rast::http::Patch,
    _ /* extensions */    => rast::http::Get
  }
}

/// Takes a rust-http request and forces it into a `Url`.
///
/// `None` indicates a parsing error but still allows the request
/// to go through; the `raw_url` may then be used to error out.
///
/// `Star` is translated to "http://*", [as per rast's docs][0].
/// [0]: https://passcod.name/rast/rast/struct.Environment.html
fn http_req_to_rast_url(r: &Request) -> (Option<url::Url>, String) {
  match r.request_uri {
    request::Star => (match url::from_str("http://*") {
      Ok(u) => Some(u),
      Err(s) => None // never happens
    }, String::from_str("*")),
    request::AbsoluteUri(u) => (Some(u), format!("{}", u)),
    request::AbsolutePath(p) => {
      let mut o = url::from_str(format!("http://foo{}", p.as_slice()).as_slice());
      (match o {
        Ok(u) => {
          u.scheme = String::from_str("");
          u.host = String::from_str("");
          Some(u)
        },
        Err(s) => {
          println!("[rast-http] Error decoding absolute path: {}", s);
          None
        }
      }, p)
    },
    request::Authority(a) => (match url::from_str(a.as_slice()) {
      Ok(u) => Some(u),
      Err(s) => {
        println!("[rast-http] Error decoding authority: {}", s);
        None
      }
    }, a)
  }
}

struct HttpHandler {
  config: Config
}

struct HttpServer<'a> {
  config: Config,
  app: &'a rast::App
}

impl<'a> Server for HttpServer<'a> {
  fn get_config(&self) -> Config { self.config }
  fn handle_request(&self, r: &Request, w: &mut ResponseWriter) {
    let (u, raw_url) = http_req_to_rast_url(r);
    let env = rast::Environment {
      url: u,
      raw_url: raw_url,
      request_method: http_method_to_rast(r.method),
      content_length: match r.headers.content_length {
        Some(x) => x,
        None => 0
      }
    };

    let req_headers: HashMap<String, String> = HashMap::new();
    for header in r.headers.iter() {
      req_headers.insert(header.header_name(), header.header_value());
    }

    let stderr = io::stdio::stderr(); // Not ideal, but necessary until
    let mut error = &mut stderr;      // Rast handles that itself.
    
    let body_reader = io::BufReader::new(r.body.as_slice().as_bytes());

    let resp_stream: memstream::DListStream = memstream::MemStream::new();
    let app = self.app;
    let (status, resp_headers) = app.call(env, req_headers, body_reader, error, &mut resp_stream);

    let rast::http::StatusCode(i) = status;
    let s: Option<http::status::Status> = FromPrimitive::from_uint(i as uint);
    w.status = match s {
      Some(x) => x,
      None => http::status::Status::from_code_and_reason(i, String::from_str(""))
    };

    for (header, value) in resp_headers.iter() {
      w.headers.extensions.insert(String::from_str(*header), String::from_str(*value)); 
    }

    w.write(match resp_stream.read_to_end() {
      Ok(v) => v.as_slice(),
      Err(e) => bytes!("")
    }).unwrap();
  }
}

impl rast::Handler for HttpHandler {
  fn run<T: rast::App>(&self, app: &T) {
    let server = HttpServer {config: self.config.clone(), app: app};
    server.serve_forever();
  }
}
