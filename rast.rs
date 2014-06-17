#![crate_id = "rast#0.4"]

//! # Rast - A Rust Webserver Interface
//!
//! Inspired from [Rack], but doesn't really work the same.
//!
//! https://github.com/passcod/rast
//! [Rack]: http://rack.rubyforge.org/doc/SPEC.html

#![crate_type = "lib"]
#![comment = "A Rust Webserver Interface"]
#![license = "Public Domain"]

extern crate url;
use std::{fmt,io,vec};
use std::collections::HashMap;
use std::string::String;
pub mod memstream;
pub mod log;

/// The trait web servers must implement.
///
/// The only requirement is to have a `run` method which
/// takes a `handler` closure with `Request` and `Response`
/// arguments. That handler must be called whenever a request
/// is made to the server.
pub trait Server {
  fn run<R: io::Reader, W: io::Writer>(&self,
    handler: |req: &mut Request<R>, res: &mut Response<W>|);
}

/// The trait web applications must implement.
///
/// An `App` has a `call` function, which is invoked for
/// each and every request, with three parameters: the
/// `Request` and `Response` objects, as well as a `log`
/// closure which should be used for all logging purposes.
pub trait App {
  fn call<R: io::Reader, W: io::Writer, M: fmt::Show>(&self,
    req: &mut Request<R>, res: &mut Response<W>,
    log: |level: log::Level, message: M|);
}

/// HTTP Methods
///
/// Listed in alphabetical order and taken from [RFCnnn].
/// 
/// The `ExtensionMethod` construct allows arbitrary methods
/// for servers that support these.
pub enum Method {
  Connect,
  Delete,
  Get,
  Head,
  Options,
  Patch,
  Post,
  Put,
  Trace,
  ExtensionMethod(String)
}

/// Represents an HTTP request.
///
/// If `url` is `None`, it indicates the request's
/// URL couldn't be parsed or was missing. The [asterisk
/// from RFC2616§5.1.2][rfc2616sec512] (`*`) should be
/// translated to the valid `Url` `http://*`.
///
/// The `raw_url` should contain the original URL as
/// received by the server.
///
/// [rfc2616sec512]: http://www.w3.org/Protocols/rfc2616/rfc2616-sec5.html#sec5.1.2
pub struct Request<'a, R> {
  pub url: Option<url::Url>,
  pub raw_url: String,
  pub request_method: Method,
  pub headers: HashMap<String, String>,
  pub body: io::RefReader<'a, R>
}

/// Represents an HTTP response.
///
/// This object is created by the server, which has the possibility
/// of setting server defaults at this point, then passed to Rast,
/// then to the application, by reference. It may be mutated at any
/// point in its journey, although the most significant changes are
/// expected to be made by the application.
pub struct Response<'a, W> {
  pub status: u16,
  pub headers: HashMap<String, String>,
  pub body: io::RefWriter<'a, W>
}

/// An ensemble of a server, one or more apps, and optional middlewares.
///
/// Apps and middlewares are really the same thing: they take a
/// mutable `Request` and `Response`, modify them in some way,
/// and return. Rast takes care of invoking each `App` in turn
/// and handling the server.
pub struct Stack<'a> {
  server: &'a Server,
  apps: vec::Vec<&'a App>
}

impl<'a> Stack<'a> {
  fn new(server: &'a Server) -> Stack<'a> {
    Stack { server: server, apps: vec::Vec::new() }
  }

  fn uses(&mut self, app: &'a App) {
    self.apps.push(app);
  }

  fn starts(&mut self) {
    //
  }
}
