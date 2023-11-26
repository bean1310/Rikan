// Copyright (c) 2023 MATSUSHITA Isato
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::graphics::*;

extern "C" {
    static _binary_hankaku_bin_start: u8;
    static _binary_hankaku_bin_end: u8;
    static _binary_hankaku_bin_size: u8;
}

pub fn get_font(c: char) -> Result<[u8; 16], ()> {
    let index = c as usize * 16;
    unsafe {
        if index >= &_binary_hankaku_bin_size as *const u8 as usize {
            return Err(());
        }
        let char_ptr =  (&_binary_hankaku_bin_start as *const u8).add(index as usize) as *const [u8; 16];
        Ok(*char_ptr)
    }
}

pub fn write_ascii_at(x: u32, y: u32, c: char, frame_config: &FrameBufferConfig) {

    let font_data = get_font(c).expect("[ERROR] failed to get font");

    for dy in 0..16 {
        for dx in 0..8 {
            if (font_data[dy] << dx) & 0x80 > 0 {
                unsafe {write_pixel(x + dx as u32, y + dy as u32, PixelColor {red: 0, green: 0, blue: 0}, frame_config);}
            }
        }
    }
}