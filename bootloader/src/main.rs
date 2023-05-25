#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]

use alloc::format;
use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr::null;
use uefi::*;

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
) -> Result<EfiStatus, EfiStatus> {
    let header = "Index,\tType,\tType(name),\tPhysicalStart,\tNumberOfPages,\tAttribute\n";
    let len = header.len();

    file.write(len, header)
        .expect("Failed to write memory map header.");

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

        file.write(mem_region_info.len(), &mem_region_info)?;

        index += 1;
        offset += descriptor_size;
    }

    Ok(EfiStatus::Success)
}

/// Open root directory on file system
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

/// Load kernel binary from file system
fn load_kernel(
    base_address: u64,
    boot_service: &EfiBootServices,
    image_handle: EfiHandle,
) -> Result<(), EfiStatus> {
    let file_protocol = open_root_dir(image_handle, boot_service).unwrap();
    let kernel_file =
        file_protocol.open("\\kernel", EfiFileOpenMode::Read, EfiFileAttribute::None)?;

    let kernel_file_info = kernel_file.get_info().unwrap();
    let kernel_file_size = kernel_file_info.file_size.try_into().unwrap();

    let base_address = boot_service.allocate_pages(
        EfiAllocateType::AllocateAddress,
        EfiMemoryType::EfiLoaderData,
        kernel_file_size as usize,
        base_address,
    )?;

    kernel_file.read(kernel_file_size, base_address)?;

    println!("[DEBUG] kernel loaded at 0x{:x}", base_address);

    Ok(())
}

/// Prepare kernel and jump to kernel
fn run_kernel(boot_service: &EfiBootServices, image_handle: EfiHandle) -> ! {
    load_kernel(KERNEL_BASE_ADDRESS, boot_service, image_handle).expect("Failed to load kernel");

    let monitor_frame_buffer = get_monitor_config(image_handle, boot_service).unwrap();

    match boot_service.exit_boot_service(image_handle) {
        Ok(_) => goto_kernel(monitor_frame_buffer),
        Err(res) => {
            panic!("Failed to exit boot service. {:?}", res)
        }
    };
}

/// Jump to kernel
#[allow(unreachable_code)]
fn goto_kernel(frame_buffer_config: FrameBufferConfig) -> ! {
    unsafe {
        // Get entrypoint address of kernel from elf header
        let entry_point = ((KERNEL_BASE_ADDRESS + 24) as *const u64).as_ref().unwrap();
        let kernel_main_ptr = *entry_point as *const ();

        // Define kernel_main function type
        // Kernel binary is compiled with sysv64 calling convention
        let kernel_main = core::mem::transmute::<
            *const (),
            unsafe extern "sysv64" fn(FrameBufferConfig) -> !,
        >(kernel_main_ptr);

        kernel_main(frame_buffer_config);

        loop {
            asm!("hlt");
        }
    };
}

/// Open Graphic Output Protocol
fn open_gop(
    image_handle: EfiHandle,
    boot_service: &EfiBootServices,
) -> Result<&EfiGraphicsOutputProtocol, EfiStatus> {
    let (_, gop_handles, buffer_ptr) = boot_service
        .locate_handle_buffer(
            EfiLocateSearchType::ByProtocol,
            &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
            null(),
        )
        .unwrap();

    let gop_ptr = boot_service.open_protocol(
        gop_handles[0],
        &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
        image_handle,
        null(),
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    )? as *const EfiGraphicsOutputProtocol;

    if gop_ptr.is_null() {
        return Err(EfiStatus::NotFound);
    }

    let gop = unsafe { gop_ptr.as_ref() }.expect("Failed to reference gop_ptr");

    boot_service.free_pool(buffer_ptr).unwrap();

    Ok(gop)
}

#[repr(C)]
enum PixelFormat {
    RGB = 0,
    BGR = 1,
}

#[repr(C)]
struct FrameBufferConfig {
    frame_buffer: *mut u64,
    pixels_per_scan_line: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixel_format: PixelFormat,
}

/// Get the framebuffer address and size
fn get_monitor_config(
    image_handle: EfiHandle,
    boot_service: &EfiBootServices,
) -> Result<FrameBufferConfig, EfiStatus> {
    match open_gop(image_handle, boot_service) {
        Ok(gop) => Ok(FrameBufferConfig {
            frame_buffer: gop.mode.frame_buffer_base as *mut u64,
            pixels_per_scan_line: gop.mode.info.pixels_per_scan_line,
            horizontal_resolution: gop.mode.info.horizontal_resolution,
            vertical_resolution: gop.mode.info.vertical_resolution,
            pixel_format: match gop.mode.info.pixel_format {
                EfiGraphicsPixelFormat::PixelRedGreenBlueReserved8BitPerColor => PixelFormat::RGB,
                EfiGraphicsPixelFormat::PixelBlueGreenRedReserved8BitPerColor => PixelFormat::BGR,
                _ => panic!("Unsupported pixel format"),
            },
        }),
        Err(err) => Err(err),
    }
}

#[no_mangle]
#[allow(unreachable_code)]
/// The entry point of the bootloader
pub extern "C" fn efi_main(image_handle: EfiHandle, system_table: &EfiSystemTable) -> EfiStatus {
    uefi_alloc::init(system_table.boot_services(), system_table.con_out());
    console::init(system_table.con_out());
    println!("---- bootloader ----");

    let mut memory_map: [u8; 8192] = [0; 8192];

    let (map_size, _, descriptor_size, _) = system_table
        .boot_services()
        .get_memory_map(&mut memory_map)
        .unwrap();

    let efi_file_proto = open_root_dir(image_handle, system_table.boot_services()).unwrap();

    let opened_handle = efi_file_proto
        .open(
            "\\memmap",
            EfiFileOpenMode::CreateReadWrite,
            EfiFileAttribute::None,
        )
        .unwrap();

    match save_memory_map(&memory_map, &opened_handle, descriptor_size, map_size) {
        Ok(_) => println!("Saved memory map"),
        Err(err) => println!("Failed to save memory map: {:?}", err),
    }

    opened_handle.close().unwrap();
    efi_file_proto.close().unwrap();

    println!("---- run kernel ----");

    run_kernel(system_table.boot_services(), image_handle);

    loop {
        unsafe {
            asm!("hlt");
        }
    }

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
