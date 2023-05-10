#![no_std]
#![no_main]

use core::{panic::PanicInfo, arch::asm};
use core::ptr::*;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    // println!("{}", _panic);
    loop {
        unsafe {asm!("hlt")}
    }
}


#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PixelFormat {
    RGB = 0,
    BGR = 1,
}

#[repr(C)]
pub struct FrameBufferConfig {
    frame_buffer: *mut u64,
    pixels_per_scan_line: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixel_format: PixelFormat,
}

struct PixelColor {
    red: u8,
    green: u8,
    blue: u8,
}

unsafe fn write_pixel(x: u32, y: u32, color: PixelColor, frame_config: &FrameBufferConfig) {
    let p = (frame_config.frame_buffer as *const u8).add((4 * (frame_config.pixels_per_scan_line * y + x)) as usize) as *mut u8;
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


#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn kernel_main(frame_buffer_config: FrameBufferConfig) {

    for x in 0..frame_buffer_config.horizontal_resolution {
        for y in 0..frame_buffer_config.vertical_resolution {
            unsafe {write_pixel(x, y, PixelColor {red: 255, green: 255, blue: 255}, &frame_buffer_config);}
        }
    }

    for x in 0..200 {
        for y in 0..100 {
            unsafe {write_pixel(x, y, PixelColor {red: 0, green: 255, blue: 0}, &frame_buffer_config);}
        }
    }

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
