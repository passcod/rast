#![crate_id="rast#0.1"]
#![crate_type="lib"]

extern crate collections;
use collections::HashMap;

trait Handler {
  fn run(app: &App);
}

trait App {
  fn call(environment: HashMap<&str, &str>) -> Response;
}

struct Response<'a> {
  status: int,
  headers: HashMap<&'a str, &'a str>,
  body: &'a str
}

struct Rast<'r> {
  handler: &'r Handler,
  app: &'r App
}
