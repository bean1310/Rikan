// Copyright (c) 2023 MATSUSHITA Isato
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[repr(C)]
pub struct FrameBufferConfig {
    frame_buffer: *mut u64,
    pixels_per_scan_line: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixel_format: PixelFormat,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PixelColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PixelFormat {
    RGB = 0,
    BGR = 1,
}

pub unsafe fn write_pixel(x: u32, y: u32, color: PixelColor, frame_config: &FrameBufferConfig) {
    let p = (frame_config.frame_buffer as *const u8)
        .add((4 * (frame_config.pixels_per_scan_line * y + x)) as usize) as *mut u8;
    if frame_config.pixel_format == PixelFormat::RGB {
        p.write_volatile(color.red.into());
        p.add(1).write_volatile(color.green.into());
        p.add(2).write_volatile(color.blue.into());
    } else if frame_config.pixel_format == PixelFormat::BGR {
        p.write_volatile(color.blue.into());
        p.add(1).write_volatile(color.green.into());
        p.add(2).write_volatile(color.red.into());
    }
}

pub fn fill_background(color: PixelColor, frame_config: &FrameBufferConfig) {
    for x in 0..frame_config.horizontal_resolution {
        for y in 0..frame_config.vertical_resolution {
            unsafe {write_pixel(x, y, color, frame_config);}
        }
    }
}

pub fn fill_rectangle(x: u32, y: u32, width: u32, height: u32, color: PixelColor, frame_config: &FrameBufferConfig) {
    for dx in 0..width {
        for dy in 0..height {
            unsafe {write_pixel(x + dx, y + dy, color, frame_config);}
        }
    }
}

#[allow(dead_code)]
pub mod basic_color {
    use super::PixelColor;
    pub const BLACK: PixelColor = PixelColor {red: 0, green: 0, blue: 0};
    pub const WHITE: PixelColor = PixelColor {red: 255, green: 255, blue: 255};
    pub const RED: PixelColor = PixelColor {red: 255, green: 0, blue: 0};
    pub const GREEN: PixelColor = PixelColor {red: 0, green: 255, blue: 0};
    pub const BLUE: PixelColor = PixelColor {red: 0, green: 0, blue: 255};
    pub const YELLOW: PixelColor = PixelColor {red: 255, green: 255, blue: 0};
    pub const CYAN: PixelColor = PixelColor {red: 0, green: 255, blue: 255};
    pub const MAGENTA: PixelColor = PixelColor {red: 255, green: 0, blue: 255};
}