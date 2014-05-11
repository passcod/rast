#![crate_id="rast#0.1"]
#![crate_type="lib"]

extern crate collections;
use collections::HashMap;
use std::io::{Reader, Writer};

trait Handler {
  fn run(app: &App);
}

trait App {
  fn call(environment: HashMap<&str, &str>, input: &Reader, error: &Writer) -> Response;
}

struct Response<'a> {
  status: int,
  headers: HashMap<&'a str, &'a str>,
  body: &'a Reader
}

struct Rast<'r> {
  handler: &'r Handler,
  app: &'r App
}
