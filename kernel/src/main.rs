// Copyright (c) 2023 MATSUSHITA Isato
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#![no_std]
#![no_main]

use core::{arch::asm, ffi::c_void, panic::PanicInfo};
use heapless::String;
use core::fmt::Write;
use console::*;
use core::mem::MaybeUninit;

mod graphics;
mod console;

const mouse_cursor_height: usize = 24;
const mouse_cursor_width: usize = 15;

static mouse_cursor_shape: [&str; mouse_cursor_height] = [
    "@              ",
    "@@             ",
    "@.@            ",
    "@..@           ",
    "@...@          ",
    "@....@         ",
    "@.....@        ",
    "@......@       ",
    "@.......@      ",
    "@........@     ",
    "@.........@    ",
    "@..........@   ",
    "@...........@  ",
    "@............@ ",
    "@......@@@@@@@@",
    "@......@       ",
    "@....@@.@      ",
    "@...@ @.@      ",
    "@..@   @.@     ",
    "@.@    @.@     ",
    "@@      @.@    ",
    "@       @.@    ",
    "         @.@   ",
    "         @@@   ",
];

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

static mut FRAME_BUFFER_CONFIG: MaybeUninit<graphics::FrameBufferConfig> = MaybeUninit::uninit();

#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn kernel_main(frame_buffer_config: graphics::FrameBufferConfig, memory_map: MemoryMap) {

    unsafe {
        FRAME_BUFFER_CONFIG.write(frame_buffer_config);
        graphics::fill_background(graphics::basic_color::GRAY, FRAME_BUFFER_CONFIG.assume_init_ref());
        console::init(FRAME_BUFFER_CONFIG.assume_init_ref());
    };

    println!("---- Start RikanOS Kernel ----");
    for i in 0..3 {
        println!("Hello, RikanOS! {}", i);
    }

    for i in 0..mouse_cursor_height {
        for j in 0..mouse_cursor_width {
            let mut s = String::<15>::new();
            if mouse_cursor_shape[i].as_bytes()[j] == b'@' {
                unsafe {
                    graphics::write_pixel(100 + j as u32, 100 + i as u32, graphics::basic_color::RED, &frame_buffer_config);}
            } else if mouse_cursor_shape[i].as_bytes()[j] == b'.' {
                unsafe {
                    graphics::write_pixel(100 + j as u32, 100 + i as u32, graphics::basic_color::WHITE, &frame_buffer_config);}
            }
        }
    }

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
