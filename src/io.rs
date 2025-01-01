//Thank you rp2040-panic-usb-boot!
//https://github.com/jannic/rp2040-panic-usb-boot/blob/b223f148dfeca700a9a800bbbcfd8ac33c1ca123/src/lib.rs#L7

pub struct Cursor<'a> {
    pub buf: &'a mut [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(buf: &'a mut [u8]) -> Cursor<'a> {
        Cursor { buf, pos: 0 }
    }

    pub fn clear(&mut self) {
        for i in 0..self.buf.len() {
            self.buf[i] = 0;
        }
        self.pos = 0;
    }
}

impl<'a> core::fmt::Write for Cursor<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let len = s.as_bytes().len();
        if len < self.buf.len() - self.pos {
            self.buf[self.pos..self.pos + len].clone_from_slice(s.as_bytes());
            self.pos += len;
            Ok(())
        } else {
            Err(core::fmt::Error)
        }
    }
}
