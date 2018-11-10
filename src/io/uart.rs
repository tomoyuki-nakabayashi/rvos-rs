//! UART device.

pub struct Uart {
    addr: *mut u8,
}

impl Uart {
    pub fn new(addr: *mut u8) -> Uart {
        Uart {
            addr: addr,
        }
    }

    pub fn write(&self, ascii_code: u8) {
        unsafe {
            *self.addr = ascii_code;
        }
    }
}
