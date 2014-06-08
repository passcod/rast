extern crate rast = "rast#0.3";
extern crate rast_http;

use collections::HashMap;
use rast_http::HttpHandler;
use std::io::{Reader, Writer};

struct SomeApp;

impl rast::App for SomeApp {
  fn call<R: Reader, W: Writer, B: Writer>(
    &self,
    env: rast::Environment,
    headers: HashMap<&str, &str>,
    body: R, err: &mut W, output: &mut B
  ) -> (
    rast::http::StatusCode,
    HashMap<&str, &str>
  ) {
    // do something clever...

    output.write(a_bunch_of_bytes);
    (rast::http::StatusCode(some_status), a_few_headers)
  }
}

fn main() {
  let handler = HttpHandler { port: 8080 };
  let app = SomeApp;
  handler.run(&app);
}
