Rast
====

__(Doesn't work, WIP.)__

_A Rust Webserver Interface_

Based on [Rack](https://rack.github.io/), implements a subset of its features.

## Build

Currently tracks Rust nightlies.

```bash
$ git clone git://github.com/passcod/rast
$ cd rast
$ rustc rast.rs
```

## Use

See [handlers/rast_http.rs](handlers/rast_http.rs) for a working
implementation. Generally:


- You need to `extern crate` both `rast` and the server library.
- You need to `use std::collections::HashMap;`.
- You need to define a `struct SomeServerHandler { ... }`.
  Usually that will contain the configuration for the server.
- You need to implement [rast::Handler](https://passcod.name/rast/rast/trait.Handler.html) for that struct.
- When a server runs an app, it needs to call [app.call](https://passcod.name/rast/rast/trait.App.html) with the proper arguments.
- Once that function returns, it needs to send the response through.

### For apps:

See [examples/hello-app.rs](examples/hello-app.rs) for a working example.
Generally:

- You need to `extern crate` both `rast` and the handler of your choice.
- You need to `use std::collections::HashMap;`.
- You need to create a `struct SomeApp;`.
- You need to implement [rast::App](https://passcod.name/rast/rast/trait.App.html) for that struct.
- You should initialise the handler and use it to `.run(&App)` your app.

## Community

- Adapted and inspired from [Rack](https://rack.github.io).
- [Public Domain](https://passcod.name/PUBLIC.txt).
