use core::fmt::Arguments;

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3f8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    use core::fmt::Write;
    SERIAL
        .lock()
        .write_fmt(args)
        .expect("Failed to write to serial");
}

#[macro_export]
macro_rules! sprint {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! sprintln {
    () => {
        ($crate::sprint!("\n"));
    };
    ($fmt:expr) => {
        ($crate::sprint!(concat!($fmt, '\n')));
    };
    ($fmt:expr, $($arg:tt)*) => {
        ($crate::sprint!(concat!($fmt, '\n'), $($arg)*));
    }
}
