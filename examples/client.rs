//! Clone of https://github.com/hyperium/hyper/blob/master/examples/client.rs

#[macro_use]
extern crate failure;
extern crate futures;
extern crate hyper;
extern crate hyperuds;

use std::env;
use std::io;
use std::io::Write;

use failure::Error;
use futures::Future;
use futures::Stream;
use hyper::rt;
use hyper::Body;
use hyper::Client;
use hyperuds::UnixConnector;

fn main() -> Result<(), Error> {
    ensure!(
        3 == env::args().count(),
        "usage: path/to/socket /request/path"
    );
    let socket_path = env::args().nth(1).unwrap();
    let req = env::args().nth(2).unwrap();

    ensure!(
        req.starts_with('/'),
        "request path must be absolute (start with a /)"
    );

    let client: Client<UnixConnector, Body> =
        hyper::Client::builder().build(UnixConnector::new(socket_path));
    rt::run(
        client
            .get(format!("unix://ignored{}", req).parse()?)
            .and_then(|res| {
                println!("resp: {}", res.status());
                res.into_body().for_each(|chunk| {
                    io::stdout()
                        .write_all(&chunk)
                        .map_err(|e| panic!("example expects stdout is open, error={}", e))
                })
            })
            .map(|_| println!("done"))
            .map_err(|err| eprintln!("{}", err)),
    );

    Ok(())
}
