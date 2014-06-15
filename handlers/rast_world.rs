#![crate_id = "rast_world#0.1"]

//! # Rast - Very fake server
//!
//! - https://github.com/passcod/rast

#![crate_type = "lib"]
#![comment = "Very fake server for Rast"]
#![license = "Public Domain"]

extern crate debug;
extern crate rast = "rast#0.4";
extern crate url;
use rast::{Handler, App};
use std::collections::HashMap;
use std::string::String;
use std::io;

struct HttpHandler;

struct HttpServer<'a> {
  app: &'a rast::App
}

impl<'a> HttpServer<'a> {
  fn handle(&self) {
    let u = url::Url {
      scheme: "http".to_string(),
      user: None,
      host: "example.com".to_string(),
      port: None,
      path: "foo.bar".to_string(),
      query: vec!(("baz".to_string(), "quz".to_string())),
      fragment: None
    };
    let env = rast::Environment {
      url: Some(u),
      raw_url: "*".to_string(),
      request_method: rast::http::Get,
      content_length: 11
    };

    let req_headers: HashMap<String, String> = HashMap::new();
    
    let stderr = io::stdio::stderr(); // Not ideal, but necessary until
    let mut error = &mut stderr;      // Rast handles that itself.
    
    let body_reader = io::BufReader::new(bytes!("hello=world"));

    let (status, resp_headers) = self.app.call(env, req_headers, body_reader, error, error);

    println!("{:?}\n\n{:?}", status, resp_headers);
  }

  fn serve(&self) {
    io::timer::sleep(1000 * 5);
    self.handle();
  }
}

impl rast::Handler for HttpHandler {
  fn run<T: rast::App>(&self, app: &T) {
    let server = HttpServer {app: app};
    server.serve();
  }
}
