// Copyright (c) 2023 MATSUSHITA Isato
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#![no_std]
#![no_main]

use core::{arch::asm, ffi::c_void, panic::PanicInfo};
use heapless::String;
use core::fmt::Write;

mod graphics;
mod console;

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

#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn kernel_main(frame_buffer_config: graphics::FrameBufferConfig, memory_map: MemoryMap) {

    graphics::fill_background(graphics::basic_color::BLACK, &frame_buffer_config);

    let mut console = console::Console::new(&frame_buffer_config);
    console.write_string("Hello World");
    console.write_string(" by Rust\n");
    console.write_string("This is Isat Matsushita");

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
