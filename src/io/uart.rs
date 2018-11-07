//! UART device.

use bare_metal::Mutex;

static UART0: LockedUart = LockedUart::new();

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

pub struct LockedUart(Mutex<Option<Uart>>);

impl LockedUart {
    pub const fn new() -> LockedUart {
        LockedUart(Mutex::new(None))
    }
}