extern crate futures;
extern crate rvpn;
#[macro_use]
extern crate tokio_core;
extern crate tun;

use std::io::{self, Read, Write};
use std::net::SocketAddr;

use futures::{Future, Poll};
use tokio_core::net::UdpSocket;

struct Server {
    tun: tun::Tun,
    socket: UdpSocket,
}

impl Future for Server {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
                println!("sending");
        loop {
            let mut buf = [0; 1500];
            let nb_bytes = try_nb!(self.tun.read(&mut buf));
            if nb_bytes > 0 {
                try_nb!(self.socket.send_to(&buf[..nb_bytes],
                                            &"127.0.0.1:18425".parse().unwrap()));
            }
            let mut buf = [0; 1500];
            let (len, _addr) = try_nb!(self.socket.recv_from(&mut buf));
            if len > 0 {
                try_nb!(self.tun.write(&buf[..len]));
            }


        }
    }
}

fn real_main() -> rvpn::Result<()> {
    let mut core = tokio_core::reactor::Core::new()?;
    let socket = UdpSocket::bind(&SocketAddr::new("127.0.0.1".parse().unwrap(), rvpn::DEFAULT_PORT),
                                   &core.handle())?;
    let tun = tun::Tun::new(String::from("rvpn-server"), &core.handle())?;
    Ok(core.run(Server {
        socket: socket,
        tun: tun,
    })?)
}

fn main() {
    rvpn::quick_main(real_main);
}
