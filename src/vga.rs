const HEIGHT: usize = 50;
const WIDTH: usize = 160;

use core::fmt::{Arguments, Write};
use spin::Mutex;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

pub struct Vga {
    row: u8,
    column: u8,
    color: u8,
    buffer: *mut u16,
}

impl Vga {
    pub const fn construct_color(background: VgaColor, foreground: VgaColor) -> u8 {
        foreground as u8 | (background as u8) << 4
    }

    fn construct_character(color: u8, character: char) -> u16 {
        (color as u16) << 8 | character as u16
    }

    pub const fn new() -> Vga {
        Vga {
            row: 0,
            column: 0,
            color: Self::construct_color(VgaColor::Black, VgaColor::LightGray),
            buffer: 0xb8000 as *mut u16,
        }
    }

    fn calculate_index(row: u8, column: u8) -> usize {
        (column * WIDTH as u8 + row).into()
    }

    pub fn putchar(&mut self, character: char, color: Option<u8>) {
        if character == '\n' {
            self.row = 0;
            self.column += 1;
        } else if self.row as usize == WIDTH {
            self.row = 0;
            self.column += 1;
        } else if self.column as usize == HEIGHT {
            self.column = 0;
        } else {
            let index = Self::calculate_index(self.row, self.column);

            unsafe {
                let pos: *mut u16 = self.buffer.add(index);
                let computed_color = match color {
                    Some(color) => color,
                    None => self.color,
                };

                *pos = Self::construct_character(computed_color, character);
            }

            self.row += 1;
        }
    }

    pub fn write(&mut self, data: &str) {
        for ch in data.chars() {
            Self::putchar(self, ch, None);
        }
    }
}

pub static VGA: Mutex<Vga> = Mutex::new(Vga::new());

unsafe impl Send for Vga {}

impl Write for Vga {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s);

        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        ($crate::vga::_print(format_args!($($arg)*)));
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        VGA.lock().write_fmt(args).unwrap();
    });
}
