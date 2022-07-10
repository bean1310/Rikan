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
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
