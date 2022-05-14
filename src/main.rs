#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]

// extern crate alloc;

use core::ffi::c_void;
use core::panic::PanicInfo;
use core::ptr::{self, null, null_mut};
use uefi::*;
use utf16_literal::utf16;

extern crate alloc;
mod uefi_alloc;

// extern crate alloc;

mod console;
mod uefi;

use console::*;

fn save_memory_map(map: &EfiMemoryDescriptor, file: &EfiFileProtocol) -> EfiStatus {
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
    bs: &EfiBootServices,
) -> Result<&EfiFileProtocol, EfiStatus> {
    // let mut loaded_image: *mut EfiLoadedImageProtocol = null_mut();
    // let mut fs: *mut EfiSimpleFileSystemProtocol = null_mut();

    unsafe {
        let _loaded_image = bs
            .open_protocol(
                image_handle,
                &EFI_LOADED_IMAGE_PROTOCOL,
                image_handle,
                null(),
                EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
            )
            .unwrap();

        println!("1st done");

        let loaded_image = ((_loaded_image as *const _) as *const EfiLoadedImageProtocol)
            .as_ref()
            .unwrap();

        // println!("{:?}", loaded_image);

        let _fs = bs
            .open_protocol(
                loaded_image.device_handle,
                &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
                image_handle,
                null(),
                EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
            )
            .unwrap();

        println!("2nd done");

        let fs = ((_fs as *const _) as *const EfiSimpleFileSystemProtocol)
            .as_ref()
            .unwrap();

        fs.open_volume()
    }
}

#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn efi_main(image_handle: EfiHandle, system_table: &EfiSystemTable) -> EfiStatus {
    uefi_alloc::init(system_table.boot_services(), system_table.con_out());
    console::init(system_table.con_out());
    println!("---- efi_main -----");
    println!("{} + {} = {}", 10, 20, 10 + 20);

    let mut memory_map: [EfiMemoryDescriptor; 60] = [Default::default(); 60];

    let (_, _, _, _) = system_table
        .boot_services()
        .get_memory_map(&mut memory_map)
        .unwrap();

    println!("{:?}", memory_map);

    println!("get_memory_map() is done !");
    let mut root_dir: *mut EfiFileProtocol = ptr::null_mut();
    let efi_file_proto = open_root_dir(image_handle, system_table.boot_services()).unwrap();
    println!("open_root_dir() is done");

    let opened_handle = efi_file_proto
        .open(
            "\\memmap",
            EfiFileOpenMode::CreateReadWrite,
            EfiFileAttribute::None,
        )
        .unwrap();

    // save_memory_map(&memoryMap, &memmap_file, _conout);
    //ここから
    // _conout.OutputString(utf16!("pass1.0\r\n").as_ptr());
    // let display_data = format!("n");

    // _conout.OutputString(utf16!("pass1.1").as_ptr());
    // _conout.OutputString(display_data as *const u16);

    // _conout.OutputString(utf16!("pass2").as_ptr());

    loop {}

    EfiStatus::Success
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    println!("{}", _panic);
    loop {}
}
