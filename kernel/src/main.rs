#![no_std]
#![no_main]

use core::{panic::PanicInfo, arch::asm};

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    // println!("{}", _panic);
    loop {}
}

#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn kernel_main() {
    // for i in 0..buffer_size {
    //     unsafe {*(buffer_base.offset(i.try_into().unwrap())) = (i / 255).try_into().unwrap();}
    // }
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
