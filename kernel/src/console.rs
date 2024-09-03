// Copyright (c) 2024 MATSUSHITA Isato
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::graphics::*;

mod font;

const DEFAULT_LINE_SPACE: u32 = 16;
const DEFAULT_WIDTH_BUFFER: u32 = 8;
const DEFAULT_HEIGHT_BUFFER: u32 = 8;

pub struct Console<'a> {
    cursor_x: u32,
    cursor_y: u32,
    frame_buffer_config: &'a FrameBufferConfig,
}

impl<'a> Console<'a> {

    pub fn new(fbc: &'a FrameBufferConfig) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
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
                self.write_ascii_at(DEFAULT_WIDTH_BUFFER + self.cursor_x * 8, DEFAULT_HEIGHT_BUFFER + self.cursor_y, c);
                self.cursor_x += 1;
            }
        }
    }

    pub fn new_line(&mut self) {
        self.cursor_x = 0;
        self.cursor_y += DEFAULT_LINE_SPACE;
    }
}