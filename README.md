Rast
====

_A Rust Webserver Interface_

Based on [Rack](https://rack.github.io/), implements a subset of its features.

## Build

Currently tracks Rust nightlies.

```bash
$ git clone git://github.com/passcod/rast
$ cd rast
$ rustc rast.rs
```

## Link

```rust
extern crate rast = "rast#0.3";
use rast::{Handler, App};
```

## Use

### For servers:

```rust
extern crate collections;
use collections::HashMap;
struct SomeServerHandler {
  port: uint,
  // ...other config...
};

impl rast::Handler for SomeServerHandler {
  fn run<T: rast::App>(&self, app: &T) {
    let server = SomeServer::create();
    server.on_get(proc(req) {
      let env = rast::Environment {
        request_method: rast::http::Get,
        script_name: "",
        path_info: req.path,
        query_string: req.query,
        server_name: server.name,
        server_port: rast::http::Port(server.port),
        url_scheme: if server.is_secure() {
          "https"
        } else {
          "http"
        },
        content_length: req.length
      };

      let mut errors = std::io::MemWriter::new();
      let mut body = std::io::MemWriter::new();

      let status, headers = app.call(env, req.headers, req.content, &mut errors, &mut body);
      server.respond(status as u16, headers, body.unwrap());
    });
    server.run_loop();
  }
}
```

### For apps:

```rust
extern crate collections;
extern crate someserverhandler;

use collections::HashMap;
use somewerverhandler::SomeServerHandler;

struct SomeApp;

impl rast::App for SomeApp {
  fn call<R: std::io::Reader, W: std::io::Writer, B: std::io::Writer>(
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
  let handler = SomeServerHandler { port: 8080 };
  let app = SomeApp;
  handler.run(&app);
}
```

## Community

- Adapted and inspired from [Rack](https://rack.github.io)
- [Public Domain](https://passcod.name/PUBLIC.txt)
