extern crate rast = "rast#0.3";
extern crate collections;
use collections::HashMap;
use std::io::{Reader, Writer, MemReader, MemWriter};
use rast::{Handler,App};

struct TestApp;

impl rast::App for TestApp {
  fn call<R: Reader, W: Writer, B: Writer>(
    &self,
    env: rast::Environment,
    h: HashMap<&str, &str>,
    body: R, err: &mut W, res: &mut B
  ) -> (
    rast::http::StatusCode,
    HashMap<&str, &str>
  ) {
    res.write_str("World");
    (rast::http::StatusCode(200), HashMap::new())
  }
}

struct TestHandler;

impl rast::Handler for TestHandler {
  fn run<T: rast::App>(&self, app: &T) {
    let env = rast::Environment {
      request_method: rast::http::Get,
      script_name: "/",
      path_info: "/",
      query_string: "?",
      server_name: "localhost",
      server_port: rast::http::Port(8080),
      url_scheme: rast::http::Http,
      content_length: 0
    };

    let mut heads = HashMap::new();
    heads.insert("foo", "bar");
    let inpt = MemReader::new(std::vec::Vec::from_slice("Hello".to_owned().into_bytes()));
    let mut errr = MemWriter::new();
    let mut resp = MemWriter::new();

    println!("{:?}", app.call(env, heads, inpt, &mut errr, &mut resp));
    println!("{:?}", resp);
  }
}

#[test]
fn the_universe() {
  let app = TestApp;
  let han = TestHandler;
  han.run(&app);
}

fn main() {
  let app = TestApp;
  let han = TestHandler;
  han.run(&app);
}
