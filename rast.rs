#![crate_id = "rast#0.3"]

//! # Rast - A Rust Webserver Interface
//!
//! Adapted and inspired from [Rack].
//!
//! https://github.com/passcod/rast
//! [Rack]: http://rack.rubyforge.org/doc/SPEC.html

#![crate_type = "lib"]
#![comment = "A Rust Webserver Interface"]
#![license = "Public Domain"]

use std::collections::HashMap;
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
/// each and every request, with five parameters:
/// 
/// - the `environment`, which is precisely defined;
/// - the `headers`, in the shape of a free-form `HashMap`;
/// - the `input`, a `Reader` which provides the contents of the request;
/// - the `error`, a `Writer` to be used to provide error information per-request.
/// - the `output`, a `Writer` to be used to write the body of the response
///
/// The `call` function must return a tuple containing the `StatusCode`
/// and the headers `HashMap`. When the function returns, and not before,
/// the server should start reading the body and crafting the response.
pub trait App {
  fn call<R: io::Reader, W: io::Writer, B: io::Writer>(
    &self,
    environment: Environment,
    headers: HashMap<&str, &str>,
    input: R, error: &mut W, body: &mut B
  ) -> (http::StatusCode, HashMap<&str, &str>);
}

/// The `rast::http` mod contains types used by HTTP
pub mod http {
  pub struct StatusCode(pub u16);
  pub struct Port(pub u16);
  pub enum Method { Get, Head, Post, Put, Delete, Trace, Options, Connect, Patch }
  pub enum Scheme { Http, Https }
}

pub struct Environment<'e> {
  pub request_method: http::Method,
  pub script_name: &'e str,
  pub path_info: &'e str,
  pub query_string: &'e str,
  pub server_name: &'e str,
  pub server_port: http::Port,
  pub url_scheme: http::Scheme,
  pub content_length: uint
}
