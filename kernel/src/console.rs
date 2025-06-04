// Copyright (c) 2024 MATSUSHITA Isato
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use heapless::String;
use core::fmt::{self, Write};

use crate::graphics::{self, *};

mod font;

const DEFAULT_LINE_SPACE: u32 = 16;
const DEFAULT_WIDTH_BUFFER: u32 = 8;
const DEFAULT_HEIGHT_BUFFER: u32 = 8;
const MAX_LINE: usize = 30;
const MAX_LINE_WIDTH: usize = 80;

const DEFAULT_STRING: String::<MAX_LINE_WIDTH> = String::<MAX_LINE_WIDTH>::new();

static mut CONSOLE: Option<Console> = None;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn init(fbc: &'static FrameBufferConfig) {
    unsafe {
        if !CONSOLE.is_some() {
            CONSOLE = Some(Console::new(fbc));
        }
    }
}

pub fn _print(args: core::fmt::Arguments) {
    unsafe {
        if let Some(console) = CONSOLE.as_mut() {
            console.write_fmt(args).unwrap();
        }
    }
}

impl Write for Console<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

pub struct Console<'a> {
    cursor_x: usize,
    cursor_y: usize,
    line_buffer: [String<MAX_LINE_WIDTH>; MAX_LINE],
    frame_buffer_config: &'a FrameBufferConfig,
}

impl Console<'_> {

    pub fn new(fbc: &'static FrameBufferConfig) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            line_buffer: [DEFAULT_STRING; MAX_LINE],
            frame_buffer_config: fbc,
        }
    }

    fn write_ascii_at(&self, x: u32, y: u32, c: char) {

        let font_data = font::get_font(c).expect("[ERROR] failed to get font");
    
        for dy in 0..16 {
            for dx in 0..8 {
                if (font_data[dy] << dx) & 0x80 > 0 {
                    unsafe {write_pixel(x + dx as u32, y + dy as u32, basic_color::WHITE, self.frame_buffer_config);}
                }
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for c in s.chars() {
            if c == '\n' {
                self.new_line();
            } else {
                self.write_ascii_at(DEFAULT_WIDTH_BUFFER + (self.cursor_x * 8) as u32, DEFAULT_HEIGHT_BUFFER + (self.cursor_y * 16) as u32, c);
                self.line_buffer[self.cursor_y].push(c).unwrap();
                self.cursor_x += 1;
            }
        }
    }

    fn write_raw_string(&self, s: &str, mut x: usize, mut y: usize) {
        for i in 0..MAX_LINE_WIDTH {
            self.write_fill_ascii(DEFAULT_WIDTH_BUFFER + (i * 8) as u32, DEFAULT_HEIGHT_BUFFER + (y * 16) as u32);
        }
        for c in s.chars() {
            self.write_ascii_at(DEFAULT_WIDTH_BUFFER + (x * 8) as u32, DEFAULT_HEIGHT_BUFFER + (y * 16) as u32, c);
            x = x + 1;
        }
    }

    fn write_fill_ascii(&self, x: u32, y: u32) {

        let fill_font: [u8; 16] = [0b11111111; 16];
    
        for dy in 0..16 {
            for dx in 0..8 {
                if (fill_font[dy] << dx) & 0x80 > 0 {
                    unsafe {write_pixel(x + dx as u32, y + dy as u32, basic_color::GRAY, self.frame_buffer_config);}
                }
            }
        }
    }

    pub fn new_line(&mut self) {
        self.cursor_x = 0;
        self.cursor_y += 1;
        if self.cursor_y > (MAX_LINE - 1) as _ {
            // graphics::fill_background(basic_color::BLACK, self.frame_buffer_config);
            for i in 0..MAX_LINE  {
                self.line_buffer[i].clear();
                // self.line_buffer[i].push_str(self.line_buffer[i + 1].as_str()).unwrap();
                if i + 1 == MAX_LINE {
                    self.line_buffer[i] = DEFAULT_STRING;
                } else {
                    let s = self.line_buffer[i + 1].clone();
                    self.line_buffer[i] = s;
                }
                self.write_raw_string(self.line_buffer[i].as_str(), 0, i);
            }
            self.cursor_y = MAX_LINE - 1;
        }
    }
}
