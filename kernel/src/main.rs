// Copyright (c) 2023 MATSUSHITA Isato
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#![no_std]
#![no_main]

use core::{arch::asm, ffi::c_void, panic::PanicInfo};
use heapless::String;
use core::fmt::Write;

mod font;
mod graphics;

#[repr(C)]
pub struct MemoryMap {
    buffer_size: u64,
    buffer: *mut c_void,
    map_size: u64,
    map_key: u64,
    descriptor_size: u64,
    descriptor_version: u32,
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    // println!("{}", _panic);
    loop {
        unsafe {asm!("hlt")}
    }
}

fn write_string(s: &str, x: u32, y: u32, frame_buffer_config: &graphics::FrameBufferConfig) {
    let mut i = 0;
    for c in s.chars() {
        font::write_ascii_at(x + i * 8, y, c, frame_buffer_config);
        i += 1;
    }
}

#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn kernel_main(frame_buffer_config: graphics::FrameBufferConfig, memory_map: MemoryMap) {

    graphics::fill_background(graphics::basic_color::WHITE, &frame_buffer_config);
    graphics::fill_rectangle(0, 0, 200, 100, graphics::basic_color::CYAN, &frame_buffer_config);

    let mut i = 1;
    for c in '!'..'~' {
        font::write_ascii_at(i * 8, 50, c, &frame_buffer_config);
        i += 1;
    }

    write_string("Hello World", 10, 70, &frame_buffer_config);

    let mut s: String<64> = String::new();
    write!(s, "{} + {} = {}", 1, 1, 1+1).unwrap();
    write_string(s.as_str(), 10, 90, &frame_buffer_config);

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
