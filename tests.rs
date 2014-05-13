extern crate rast;
extern crate collections;
use collections::HashMap;
use std::io::{Reader, Writer, MemReader, MemWriter};
use rast::{Handler,App};

struct TestApp;

impl rast::App for TestApp {
  fn call<R: Reader, W: Writer>(&self,
          env: rast::Environment,
          h: HashMap<&str, &str>,
          body: R, err: W) -> rast::Response {
    let r = MemReader::new((~"World").into_bytes());
    rast::Response {
      status: rast::http::StatusCode(200),
      headers: HashMap::new(),
      body: r
    }
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
    let inpt = MemReader::new((~"Hello").into_bytes());
    let errr = MemWriter::new();

    println!("{:?}", app.call(env, heads, inpt, errr).body);
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
