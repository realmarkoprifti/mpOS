const HEIGHT: usize = 25;
const WIDTH: usize = 80;

use core::fmt::Write;

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
    White = 15
}

pub struct Vga {
    row: u8,
    column: u8,
    color: u8,
    buffer: *mut u16
}

impl Vga {
    const fn construct_color(background: VgaColor, foreground: VgaColor) -> u8 {
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
            buffer: 0xb8000 as *mut u16
        }
    }

    fn calculate_index(row: u8, column: u8) -> usize {
        (column * WIDTH as u8 + row).into()
    }

    pub fn putchar(&mut self, character: char, color: Option<u8>) {
        if character == '\n' {
            self.row = 0;
            self.column += 1;
        }
        else if self.row as usize == WIDTH {
            self.row = 0;
            self.column += 1;
        }

        else if self.column as usize == HEIGHT {
            self.column = 0;
        }

        else {
            let index = Self::calculate_index(self.row, self.column);

            unsafe {
                let pos: *mut u16 = self.buffer.add(index);
                let computed_color = match color {
                    Some(color) => color,
                    None => self.color
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

pub static mut VGA: Vga = Vga::new();

unsafe impl Sync for Vga {}

