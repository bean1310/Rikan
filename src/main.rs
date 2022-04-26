#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]

// extern crate alloc;

use core::panic::PanicInfo;
use core::ptr::{null, null_mut, self};
use uefi::*;
use core::ffi::c_void;
use utf16_literal::utf16;

#[macro_use]
extern crate alloc;
mod uefi_alloc;

// extern crate alloc;

mod uefi;

struct MemoryMap<'a> {
    buffer_size: usize,
    buffer: &'a mut [u8],
    map_size: usize,
    map_key: usize,
    descriptor_size: usize,
    descriptor_version: u32,
}

fn getMemoryMap(memory_map: &mut MemoryMap, bs: &EfiBootServices<'static>) -> EfiStatus {
    memory_map.map_size = memory_map.buffer_size;
    bs.get_memory_map(
        &mut memory_map.map_size,
        &mut memory_map.buffer,
        &mut memory_map.map_key,
        &mut memory_map.descriptor_size,
        &mut memory_map.descriptor_version,
    )
}

fn save_memory_map(map: &MemoryMap, file: &EfiFileProtocol, cout: &EfiSimpleTextOutputProtocol) -> EfiStatus {
    let header = "Index, Type, Type(name), PhysicalStart, NumberOfPages, Attribute\n";
    let len = header.len();

    file.write(len, header);
    // こういうところで動的メモリ確保が使いたい
    // let display_data = format!("map->buffer = {:x}, map->map_size = {:x}", map.buffer, map.buffer_size);

    // cout.OutputString(utf16!(display_data).as_ptr())

    EfiStatus::Success
}

fn open_root_dir(
    image_handle: EfiHandle,
    root: &mut *mut EfiFileProtocol,
    bs: &EfiBootServices<'static>,
) -> EfiStatus {
    let mut loaded_image: *mut EfiLoadedImageProtocol = null_mut();
    let mut fs: *mut EfiSimpleFileSystemProtocol = null_mut();

    bs.open_protocol(
        image_handle,
        &EFI_LOADED_IMAGE_PROTOCOL,
        (&mut loaded_image as *mut *mut EfiLoadedImageProtocol) as *mut *mut c_void,
        image_handle,
        null(),
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    );

    bs.open_protocol(
        unsafe{(*loaded_image).device_handle}, 
        &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
        (&mut fs as *mut *mut EfiSimpleFileSystemProtocol) as *mut *mut c_void,
        image_handle,
        null(),
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL
    );

    unsafe{(*fs).open_volume(root)};

    EfiStatus::Success
}

#[no_mangle]
pub extern "C" fn efi_main(
    ImageHandle: EfiHandle,
    SystemTable: &EfiSystemTable<'static>,
) -> EfiStatus {
    let _conout = SystemTable.ConOut();
    _conout.Reset(false);
    _conout.OutputString(utf16!("Hello World\r\n").as_ptr());

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

    _conout.OutputString(utf16!("pass1").as_ptr());

    let mut root_dir: *mut EfiFileProtocol = ptr::null_mut();
    open_root_dir(ImageHandle, &mut root_dir, SystemTable.BootServices());

    let mut memmap_file: *mut EfiFileProtocol = ptr::null_mut();

    unsafe{
        (*root_dir).open(&mut memmap_file, utf16!("memmap").as_ptr(), EFI_FILE_MODE_READ | EFI_FILE_MODE_WRITE | EFI_FILE_MODE_CREATE, 0);
    }

    // save_memory_map(&memoryMap, &memmap_file, _conout);
    //ここから
    _conout.OutputString(utf16!("pass1.0").as_ptr());
    let display_data = format!("{}{}", 1, 2).as_bytes().as_ptr();

    _conout.OutputString(utf16!("pass1.1").as_ptr());
    _conout.OutputString(display_data as *const u16);

    _conout.OutputString(utf16!("pass2").as_ptr());

    loop {}

    EfiStatus::Success
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
