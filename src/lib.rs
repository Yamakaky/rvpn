#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate mio;
extern crate tokio_core;
extern crate tun;

use tokio_core::io::Io;
use tokio_core::reactor;
use futures::Stream;

error_chain! {
    links {
        Tun(tun::Error, tun::ErrorKind);
    }
    foreign_links {
        Io(::std::io::Error);
    }
}

pub const DEFAULT_PORT: u16 = 18424;
pub const DEFAULT_PORT_CLIENT: u16 = 18425;

pub fn quick_main<F>(main: F) where F: Fn() -> Result<()> {
    if let Err(e) = main() {
        println!("{}", e);
        for cause in e.iter().skip(1) {
            println!("  caused by: {}", cause);
        }
        if let Some(b) = e.backtrace() {
            println!("{:?}", b);
        }
    }
    println!("done");
}
