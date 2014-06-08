#![crate_id = "rast_http#0.1"]

//! # Rast - Rust-Http Handler
//!
//! - https://github.com/passcod/rast
//! - https://github.com/chris-morgan/rust-http

#![crate_type = "lib"]
#![comment = "Rast handler for rust-http"]
#![license = "Public Domain"]

extern crate http = "http#0.1-pre";
extern crate rast = "rast#0.3";
use std::collections::HashMap;
use std::io;
use http::server::{Config, Server, Request, ResponseWriter};
use http::headers::HeaderEnum;
use http::headers::content_type::MediaType;
use rast::{Handler, App};

struct HttpHandler {
  config: Config
}

struct HttpServer {
  config: Config,
  app: &rast::App
}

impl Server for HttpServer {
  fn get_config(&self) -> Config { self.config }
  fn handle_request(&self, r: &Request, w: &mut ResponseWriter) {
    let env = rast::Environment {
      request_method: ,
      script_name: ,
      path_info: ,
      query_string: ,
      server_name: ,
      server_port: ,
      url_scheme: ,
      content_length: 
    };

    let stderr = io::stdio::stderr(); // Not ideal, but necessary until
    let mut error = &mut stderr;      // Rast handles that itself.

    //
  }
}

impl rast::Handler for HttpHandler {
  fn run<T: rast::App>(&self, app: &T) {
    let server = HttpServer {config: self.config.clone(), app: app};
    server.serve_forever();
  }
}
