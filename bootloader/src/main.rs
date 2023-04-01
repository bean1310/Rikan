#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]

// extern crate alloc;

use alloc::format;
use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr::{self, null, null_mut};
use uefi::*;
use utf16_literal::utf16;

extern crate alloc;
mod uefi_alloc;

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

const KERNEL_BASE_ADDRESS: u64 = 0x0010_0000;

fn save_memory_map(
    map: &[u8],
    file: &EfiFileProtocol,
    descriptor_size: usize,
    map_size: usize,
) -> EfiStatus {
    let header = "Index,\tType,\tType(name),\tPhysicalStart,\tNumberOfPages,\tAttribute\n";
    let len = header.len();

    let written_size = file.write(len, header).unwrap();

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

        let loaded_image = ((_loaded_image as *const _) as *const EfiLoadedImageProtocol)
            .as_ref()
            .unwrap();

        let _fs = bs
            .open_protocol(
                loaded_image.device_handle,
                &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
                image_handle,
                null(),
                EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
            )
            .unwrap();

        let fs = ((_fs as *const _) as *const EfiSimpleFileSystemProtocol)
            .as_ref()
            .unwrap();

        fs.open_volume()
    }
}

fn run_kernel(
    boot_service: &EfiBootServices,
    efi_file_proto: &EfiFileProtocol,
    image_handle: EfiHandle,
    map_key: usize,
    buffer_info: (*mut u64, u64)
) -> ! {
    let kernel_file = efi_file_proto
        .open("\\kernel", EfiFileOpenMode::Read, EfiFileAttribute::None)
        .unwrap();

    let kernel_file_info = kernel_file.get_info("\\kernel").unwrap();
    let kernel_file_size = kernel_file_info.file_size.try_into().unwrap();

    let addr = boot_service
        .allocate_pages(
            EfiAllocateType::AllocateAddress,
            EfiMemoryType::EfiLoaderData,
            kernel_file_size as usize,
            KERNEL_BASE_ADDRESS,
        )
        .unwrap();

    println!("addr is {}", addr);

    kernel_file
        .read(kernel_file_size, KERNEL_BASE_ADDRESS)
        .unwrap();

    let (buffer_base, buffer_size) = (buffer_info.0, buffer_info.1);
    
    let mut memory_map: [u8; 8192] = [0; 8192];
    let (map_size, map_key, descriptor_size, _) =
    boot_service.get_memory_map(&mut memory_map).unwrap();

    match boot_service.exit_boot_service(image_handle, map_key) {
        Ok(_) => unsafe { goto_kernel(buffer_base,buffer_size) },
        Err(res) => {
            println!("Failed to EXIT_BOOT_SERVICE because {:?}", res);
            let mut memory_map: [u8; 8192] = [0; 8192];
            // re-generate map key
            let (map_size, new_map_key, descriptor_size, _) =
                boot_service.get_memory_map(&mut memory_map).unwrap();
            match boot_service.exit_boot_service(image_handle, new_map_key) {
                Ok(_) => unsafe {
                    goto_kernel(buffer_base,buffer_size) },
                Err(status) => {
                    panic!("{:?}", status)
                }
            }
        }
    };
}

unsafe fn goto_kernel(buffer_base: *mut u64, buffer_size: u64) -> ! {
    let kernel_main_ptr = (KERNEL_BASE_ADDRESS + 0x0120) as *const ();
    let kernel_main = core::mem::transmute::<*const (), fn()>(kernel_main_ptr);
    kernel_main();
    loop {
        asm!("hlt");
    }
}

// unsafe fn open_gop(image_handle: EfiHandle, boot_service: &EfiBootServices) -> &EfiGraphicsOutputProtocol {
unsafe fn open_gop(image_handle: EfiHandle, boot_service: &EfiBootServices) -> Result<&EfiGraphicsOutputProtocol, EfiStatus>{
    let (num_gop_handles, gop_handles) = boot_service.locate_handle_buffer(EfiLocateSearchType::ByProtocol, &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID, null()).unwrap();
    println!("handles: {}", num_gop_handles);
    let _res = (boot_service.open_protocol(gop_handles[0], &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID, image_handle, null(), EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL).unwrap() as *const EfiGraphicsOutputProtocol).as_ref().unwrap();
    boot_service.free_pool(gop_handles[0]).unwrap();
    return Ok(_res);
}

fn get_buffer_info(image_handle: EfiHandle, boot_service: &EfiBootServices) -> Result<(*mut u64, u64), EfiStatus> {

    match unsafe {open_gop(image_handle, boot_service)} {
        Ok(gop) => {
            Ok((gop.mode.frame_buffer_base as *mut u64, gop.mode.frame_buffer_size as u64))
        },
        Err(err) => Err(err)
    }
}

#[no_mangle]
#[allow(unreachable_code)]
pub extern "C" fn efi_main(image_handle: EfiHandle, system_table: &EfiSystemTable) -> EfiStatus {
    uefi_alloc::init(system_table.boot_services(), system_table.con_out());
    console::init(system_table.con_out());
    println!("---- bootloader ----");

    let _ = utf16!("tetetete").as_ptr();

    let mut memory_map: [u8; 8192] = [0; 8192];

    let (map_size, map_key, descriptor_size, _) = system_table
        .boot_services()
        .get_memory_map(&mut memory_map)
        .unwrap();

    let mut root_dir: *mut EfiFileProtocol = ptr::null_mut();
    let efi_file_proto = open_root_dir(image_handle, system_table.boot_services()).unwrap();

    let opened_handle = efi_file_proto
        .open(
            "\\memmap",
            EfiFileOpenMode::CreateReadWrite,
            EfiFileAttribute::None,
        )
        .unwrap();

    save_memory_map(&memory_map, &opened_handle, descriptor_size, map_size);

    opened_handle.close().unwrap();// こいつ、save_memory_map関数内部でやったら良いのでは。

    // unsafe {
    //     let gop = open_gop(image_handle, system_table.boot_services()).unwrap();
    //     // Pixel Format: {}, {} pixels/line
    //     println!("Resolution: {}x{}",
    //         gop.mode.info.horizontal_resolution,
    //         gop.mode.info.vertical_resolution,
    //     );

    //     println!("Frame Buffer: 0x{:x} - 0x{:x}, Size: {} bytes",
    //         gop.mode.frame_buffer_base,
    //         gop.mode.frame_buffer_base + gop.mode.frame_buffer_size as u64,
    //         gop.mode.frame_buffer_size        
    //     );

    //     let frame_buffer = gop.mode.frame_buffer_base as *mut u64;
    //     for i in 0..gop.mode.frame_buffer_size {
    //         *(frame_buffer.offset(i.try_into().unwrap())) = (i / 255).try_into().expect(format!("i is {}", i).as_str());
    //     }
    // }
    

    println!("---- run kernel ----");

    run_kernel(
        system_table.boot_services(),
        efi_file_proto,
        image_handle,
        map_key,
        get_buffer_info(image_handle, system_table.boot_services()).unwrap()
    );

    loop {}

    EfiStatus::Success
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    println!("{}", _panic);
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
