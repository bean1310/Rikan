#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]

// extern crate alloc;

use alloc::format;
use core::ffi::c_void;
use core::panic::PanicInfo;
use core::ptr::{self, null, null_mut};
use uefi::*;

extern crate alloc;
mod uefi_alloc;

// extern crate alloc;

mod console;
mod uefi;

use console::*;

fn get_memory_map_unicode(memory_type_number: u32) -> &'static str {
    match memory_type_number {
        0 => "EfiReservedMemoryType",
        1 => "EfiLoaderCode",
        2 => "EfiLoaderData",
        3 => "EfiBootServicesCode",
        4 => "EfiBootServicesData",
        5 => "EfiRuntimeServiceCode",
        6 => "EfiRuntimeServiceData",
        7 => "EfiConventionalMemory",
        8 => "EfiUnusableMemory",
        9 => "EfiACPIReclaimMemory",
        10 => "EfiACPIMemoryNVS",
        11 => "EfiMemoryMappedIO",
        12 => "EfiMemoryMappedIOPortSpace",
        13 => "EfiPalCode",
        14 => "EfiPersistentMemory",
        15 => "EfiUnacceptedMemoryType",
        16 => "EfiMaxMemoryType",
        _ => "Unknown Memory Type",
    }
}

fn save_memory_map(
    map: &[u8],
    file: &EfiFileProtocol,
    descriptor_size: usize,
    map_size: usize,
) -> EfiStatus {
    let header = "Index,\tType,\tType(name),\tPhysicalStart,\tNumberOfPages,\tAttribute\n";
    let len = header.len();

    let written_size = file.write(len, header).unwrap();

    println!("written_size is {:}", written_size);

    if written_size != len {
        panic!(
            "Failed to write completely. len:{} done:{}",
            len, written_size
        );
    }

    let mut index = 0;
    let mut offset = 0;

    while offset < map_size {
        let memory_descriptor = unsafe {
            (map.as_ptr().add(offset) as *const EfiMemoryDescriptor)
                .as_ref()
                .unwrap()
        };
        let mem_region_info = format!(
            "{:},\t0x{:x},\t{:},\t0x{:x},\t0x{:x},\t0x{:x}\n",
            index,
            memory_descriptor.memory_type,
            get_memory_map_unicode(memory_descriptor.memory_type),
            memory_descriptor.physical_start,
            memory_descriptor.number_of_pages,
            memory_descriptor.attribute
        );
        file.write(mem_region_info.len(), &mem_region_info);

        index += 1;
        offset += descriptor_size;
    }

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

fn run_kernel(boot_service: &EfiBootServices, efi_file_proto: &EfiFileProtocol) {
    let opened_handle = efi_file_proto
        .open(
            "\\kernel",
            EfiFileOpenMode::Read,
            EfiFileAttribute::None,
        )
        .unwrap();

    let kernel_file_info = opened_handle.get_info("\\kernel").unwrap();
    let kernel_file_size = kernel_file_info.size;

    println!("File size is {:}", kernel_file_size);

    // ここから

    opened_handle.close().unwrap();
}

#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn efi_main(image_handle: EfiHandle, system_table: &EfiSystemTable) -> EfiStatus {
    uefi_alloc::init(system_table.boot_services(), system_table.con_out());
    console::init(system_table.con_out());
    println!("---- efi_main -----");
    println!("{} + {} = {}", 10, 20, 10 + 20);

    // let mut memory_map: [EfiMemoryDescriptor; 60] = [Default::default(); 60];
    let mut memory_map: [u8; 4096] = [0; 4096];

    let (map_size, _, descriptor_size, _) = system_table
        .boot_services()
        .get_memory_map(&mut memory_map)
        .unwrap();

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

    save_memory_map(&memory_map, &opened_handle, descriptor_size, map_size);

    opened_handle.close().unwrap();

   run_kernel(system_table.boot_services(), efi_file_proto);

    loop {}

    EfiStatus::Success
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    println!("{}", _panic);
    loop {}
}
