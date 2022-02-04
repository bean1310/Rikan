#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::panic::PanicInfo;
use utf16_literal::utf16;

mod uefi;

#[no_mangle]
pub extern "C" fn efi_main(ImageHandle: uefi::EFI_HANDLE, SystemTable: &uefi::SystemTable) -> uefi::EFI_STATUS {
    let _conout = SystemTable.ConOut();
    _conout.Reset(false);
    _conout.OutputString(utf16!("Hello World\r\n").as_ptr());

    loop{}

    uefi::EFI_STATUS::Success
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop{}
}
