// Copyright (c) 2023 MATSUSHITA Isato
// 
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#![no_std]
#![no_main]

use core::{panic::PanicInfo, arch::asm};

mod font;
mod graphics;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    // println!("{}", _panic);
    loop {
        unsafe {asm!("hlt")}
    }
}

#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn kernel_main(frame_buffer_config: graphics::FrameBufferConfig) {

    graphics::fill_background(graphics::basic_color::WHITE, &frame_buffer_config);
    graphics::fill_rectangle(0, 0, 200, 100, graphics::basic_color::CYAN, &frame_buffer_config);

    let mut i = 1;
    for c in '!'..'~' {
        font::write_ascii_at(i * 8, 50, c, &frame_buffer_config);
        i += 1;
    }

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
