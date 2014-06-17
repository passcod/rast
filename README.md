Rast
====

_A Rust Webserver Interface_

Inspired by [Rack](https://rack.github.io/).

## Build

Currently tracks Rust nightlies.

```bash
$ git clone git://github.com/passcod/rast
$ cd rast
$ rustc rast.rs
```

## Use

```rust
extern crate rast;
extern crate server;
mod your_own;
mod another;

fn main() {
  let it = rast::Stack::new(&server::Handler { port: 8080 });

  it.uses(your_own::Middleware { auth: "secret" });
  it.uses(your_own::Application { });
  it.uses(another::Middleware { gzip: true });

  it.starts();
}
```

## Servers

See [handlers/rast_http.rs](handlers/rast_http.rs) for a working
implementation. Generally:


- You need to `extern crate` both `rast`, `url`, and the server library.
- You need to `use std::collections::HashMap;`.
- You need to define a `struct SomeServerHandler { ... }`.
  Usually that will contain the configuration for the server.
- You need to implement [rast::Server](https://passcod.name/rast/rast/trait.Handler.html) for that struct.

## Apps (and middleware)

See [examples/hello-app.rs](examples/hello-app.rs) for a working example.
Generally:

- You need to `extern crate` both `rast` and `url`.
- You need to `use std::collections::HashMap;`.
- You need to create a `struct SomeApp;`.
- You need to implement [rast::App](https://passcod.name/rast/rast/trait.App.html) for that struct.

## Community

- Inspired from [Rack](https://rack.github.io).
- As well as [rust-http](https://github.com/chris-morgan/rust-http).
- [Public Domain](https://passcod.name/PUBLIC.txt).
