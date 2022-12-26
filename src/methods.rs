use crate::vga_buffer::{Buffer, Color, ColorCode, Writer};
use core::fmt::Write;

pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello! ");

    write!(writer, "The numbers are {} and {}", 42, 1.0 / 3.0).unwrap();
}
