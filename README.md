Rast
====

_A Rust Webserver Interface_

Based on [Rack](https://rack.github.io/), implements a subset of its features.

## Build

```bash
$ git clone git://github.com/passcod/rast
$ cd rast
$ rustc rast.rs
```

## Link

```rust
extern crate rast;
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

      let response = app.call(env, req.headers, req.content, std::io::MemWriter::new());
      server.respond(response.status as u16, response.headers, response.body.read());
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
  fn call<R: std::io::Reader, W: std::io::Writer>(&self,
    env: rast::Environment, headers: HashMap<&str, &str>,
    body: R, err: W) -> rast::Response {

    // do something clever...

    rast::Response {
      status: rast::http::StatusCode(some_status),
      headers: a_few_headers,
      body: a_number_of_bytes
    }
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
