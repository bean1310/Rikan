#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::panic::PanicInfo;
use utf16_literal::utf16;

mod uefi;

struct MemoryMap<'a> {
    buffer_size: usize,
    buffer: &'a mut [u8],
    map_size: usize,
    map_key: usize,
    descriptor_size: usize,
    descriptor_version: u32,
}

fn getMemoryMap(memoryMap: &mut MemoryMap, gBS: &uefi::EfiBootServices) -> uefi::EfiStatus {
    memoryMap.map_size = memoryMap.buffer_size;
    gBS.GetMemoryMap(
        &mut memoryMap.map_size,
        &mut memoryMap.buffer,
        &mut memoryMap.map_key,
        &mut memoryMap.descriptor_size,
        &mut memoryMap.descriptor_version,
    )
}

#[no_mangle]
pub extern "C" fn efi_main(
    ImageHandle: uefi::EfiHandle,
    SystemTable: &uefi::SystemTable,
) -> uefi::EfiStatus {
    let _conout = SystemTable.ConOut();
    _conout.Reset(false);
    _conout.OutputString(utf16!("Hello World\n").as_ptr());

    let mut buffer: [u8; 4096 * 4] = [0; 4096 * 4];
    let mut memoryMap = MemoryMap {
        buffer_size: 4096 * 4,
        buffer: &mut buffer,
        map_size: 0,
        map_key: 0,
        descriptor_size: 0,
        descriptor_version: 0,
    };

    getMemoryMap(&mut memoryMap, SystemTable.BootServices());

    _conout.OutputString(utf16!("hoge").as_ptr());

    loop {}

    uefi::EfiStatus::Success
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
