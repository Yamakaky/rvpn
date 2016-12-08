use std::io;

use core::io::{Codec, EasyBuf};

pub struct Parser;

impl Codec for Parser {
    type In = Vec<u8>;
    type Out = Vec<u8>;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Vec<u8>>> {
        Ok(Some(buf.as_slice().into()))
    }

    fn encode(&mut self, msg: Vec<u8>, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.extend_from_slice(msg.as_slice());
        Ok(())
    }
}
