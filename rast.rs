#![crate_type="lib"]
#![crate_id="rast#0.1"]

//! Rast - A Rust Webserver Interface
//!
//! Adapted and inspired from [Rack].
//! [Rack]: http://rack.rubyforge.org/doc/SPEC.html

extern crate collections;
use collections::HashMap;
use std::io;

/// `Handler` is the trait web servers must implement.
///
/// The only requirement is to have a `run` method which
/// takes a reference to an `App`. All further requirements
/// are defined by the `App` trait.
pub trait Handler {
  fn run<T: App>(&self, app: &T);
}

/// `App` is the trait web applications must implement,
/// as well as what web servers must handle.
///
/// An `App` has a `call` function, which is invoked for
/// each and every request, with four parameters:
/// 
/// - the `environment`, which is precisely defined;
/// - the `headers`, in the shape of a free-form `HashMap`;
/// - the `input`, a `Reader` which provides the contents of the request;
/// - the `error`, a `Writer` to be used to provide error information per-request.
///
/// The `call` function must return a `Response` object.
pub trait App {
  fn call<R: io::Reader, W: io::Writer>(&self,
          environment: Environment,
          headers: HashMap<&str, &str>,
          input: R,
          error: W) -> Response;
}

/// The `rast::http` mod contains types used by HTTP
pub mod http {
  pub struct StatusCode(u16);
  pub struct Port(u16);
  pub enum Method { Get, Head, Post, Put, Delete, Trace, Options, Connect, Patch }
  pub enum Scheme { Http, Https }
}

pub struct Environment<'e> {
  request_method: http::Method,
  script_name: &'e str,
  path_info: &'e str,
  query_string: &'e str,
  server_name: &'e str, // May want to constrain these strs further (at runtime?)
  server_port: http::Port,
  url_scheme: http::Scheme,
  content_length: uint
  // Note: There must not be a Content-Length header when
  // the Status is 1xx, 204, 205 or 304. There must not be
  // a Content-Type, when the Status is 1xx, 204, 205 or 304.
}

pub struct Response<'a> {
  status: http::StatusCode,
  headers: HashMap<&'a str, &'a str>,
  body: io::MemReader
  // Not great, but until Rust has type generics,
  // it's probably good enough.
}
