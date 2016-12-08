use std::io;
use std::fs;
use std::os::unix::io::AsRawFd;

use libc;

error_chain! {
    errors {
        NameTooLong(len: usize) {
            description("Interface name too long")
            display("Interface name too long ({} >= 16)", len)
        }
        Open {
            description("Error while opening the device")
        }
        Create(code: io::Error) {
            description("Error while creating the device")
            display(me) -> ("{}: {}", me.description(), code)
            cause(code)
        }
    }
}

mod linux {
    use libc;

    ioctl!(write tun_create with b'T', 202; libc::c_int);

    #[repr(C)]
    pub struct ifreq {
        pub name: [libc::c_char; 16],
        pub flags: libc::c_short,
    }

    pub const IFF_TUN: libc::c_short = 0x0001;
}

pub struct Tun {
    pub name: String,
    pub file: fs::File,
}

impl Tun {
    pub fn new(name: &str) -> Result<Tun> {
        if name.as_bytes().len() >= 16 {
            Err(ErrorKind::NameTooLong(name.as_bytes().len()))?;
        }
        let tun = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/net/tun")
            .chain_err(|| ErrorKind::Open)?;
        let mut params = linux::ifreq {
            name: [0; 16],
            flags: linux::IFF_TUN,
        };
        for (from, to) in name.as_bytes().iter().zip(params.name.iter_mut()) {
            *to = *from as libc::c_schar;
        }
        let ret = unsafe {
            linux::tun_create(tun.as_raw_fd(), &params as *const _ as *const libc::c_void as *const i32)
        };
        if ret < 0 {
            Err(ErrorKind::Create(io::Error::last_os_error()))?;
        }
        Ok(Tun {
            name: name.into(),
            file: tun,
        })
    }
}
