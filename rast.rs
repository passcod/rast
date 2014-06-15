#![crate_id = "rast#0.4"]

//! # Rast - A Rust Webserver Interface
//!
//! Adapted and inspired from [Rack].
//!
//! https://github.com/passcod/rast
//! [Rack]: http://rack.rubyforge.org/doc/SPEC.html

#![crate_type = "lib"]
#![comment = "A Rust Webserver Interface"]
#![license = "Public Domain"]

extern crate url;
use std::collections::HashMap;
use std::io;
use std::string::String;
pub mod memstream;

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
    headers: HashMap<String, String>,
    input: R, error: &mut W, body: &mut B
  ) -> (http::StatusCode, HashMap<&str, &str>);
}

/// The `rast::http` mod contains types used by HTTP
pub mod http {
  pub struct StatusCode(pub u16);
  pub struct Port(pub u16);
  #[deriving(FromPrimitive)] pub enum Method { Connect, Delete, Get, Head, Options, Patch, Post, Put, Trace }
  pub enum Scheme { Http, Https }
}

/// Describes the environment of an HTTP request.
///
/// If `url` is `None`, it indicates the request's
/// URL couldn't be parsed or was missing. The [asterisk
/// from RFC2616§5.1.2][rfc2616sec512] (`*`) should be
/// translated to the valid `Url` `http://*`.
///
/// The `raw_url` should contain the original URL as
/// received by the server.
///
/// The `content_length` should reflect the value given
/// in the headers, or `0` if not present.
///
/// [rfc2616sec512]: http://www.w3.org/Protocols/rfc2616/rfc2616-sec5.html#sec5.1.2
pub struct Environment<'e> {
  pub url: Option<url::Url>,
  pub raw_url: String,
  pub request_method: http::Method,
  pub content_length: uint
}
