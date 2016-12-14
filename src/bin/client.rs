extern crate rvpn;
extern crate tun;

fn real_main() -> rvpn::Result<()> {
    Ok(())
}

fn main() {
    rvpn::quick_main(real_main);
}
