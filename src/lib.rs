extern crate failure;
extern crate futures;
extern crate hyper;
extern crate tokio_uds;

use std::io;
use std::path::PathBuf;

use failure::Error;
use futures::Async;
use hyper::client::connect::Connect;
use hyper::client::connect::Connected;
use hyper::client::connect::Destination;
use hyper::rt::Future;
use tokio_uds::ConnectFuture;
use tokio_uds::UnixStream;

pub struct UnixConnector {
    path: PathBuf,
}

pub struct UnixFuture {
    connect: ConnectFuture,
}

impl UnixConnector {
    pub fn new<P: Into<PathBuf>>(path: P) -> UnixConnector {
        UnixConnector { path: path.into() }
    }
}

impl Future for UnixFuture {
    type Item = (UnixStream, Connected);
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<<Self as Future>::Item>, <Self as Future>::Error> {
        self.connect
            .poll()
            .map(|c| c.map(|c| (c, Connected::new())))
    }
}

impl Connect for UnixConnector {
    type Transport = UnixStream;
    type Error = io::Error;
    type Future = UnixFuture;

    fn connect(&self, dst: Destination) -> <Self as Connect>::Future {
        UnixFuture {
            connect: tokio_uds::UnixStream::connect(&self.path),
        }
    }
}
