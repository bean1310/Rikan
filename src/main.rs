#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]

// extern crate alloc;

use core::panic::PanicInfo;
use core::ptr::{null, null_mut, self};
use alloc::boxed::Box;
use alloc::fmt::format;
use alloc::string::{String, self, ToString};
use alloc::vec::Vec;
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
    buffer: &'a mut[EfiMemoryDescriptor],
    map_size: usize,
    map_key: usize,
    descriptor_size: usize,
    descriptor_version: u32,
}

fn getMemoryMap(memory_map: &mut MemoryMap, bs: &EfiBootServices) -> EfiStatus {
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
    bs: &EfiBootServices,
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

struct Console<'a> {
    protocol: &'a EfiSimpleTextOutputProtocol,
}

impl<'a> Console<'a> {
    fn new(simpleTextOutputProto: &'a EfiSimpleTextOutputProtocol) -> Console<'a> {
        simpleTextOutputProto.Reset(false);
        Console {
            protocol: simpleTextOutputProto,
        }
    }
    // 引数のやつに\0自動的につけたいが...
    fn log(&self, text: *const u16) {
        self.protocol.OutputString(utf16!("[log] \0").as_ptr());
        self.protocol.OutputString(text);
        self.protocol.OutputString(utf16!("\r\n\0").as_ptr());
    }

    fn logng(&self, text: &str) {
        // self.protocol.OutputString(utf16!("enter!!\0").as_ptr());
        let prefix = String::from("[log]");
        let full_text = prefix + text + "\r\n\0";
        let u16_str:Vec<u16> = full_text.encode_utf16().into_iter().collect();
        let u16_ptr = u16_str.as_ptr();
        self.protocol.OutputString(u16_ptr);
    }
}

#[no_mangle]
pub extern "C" fn efi_main(
    ImageHandle: EfiHandle,
    SystemTable: &EfiSystemTable,
) -> EfiStatus {
    let console = Console::new(SystemTable.ConOut());
    uefi_alloc::init(SystemTable.BootServices(), SystemTable.ConOut());
    console.log(utf16!("Start efi_main\0").as_ptr());
    console.logng(&(format!("{} + {} = {}", 1, 5, 6)));

    let mut buffer: [EfiMemoryDescriptor; 10] = [Default::default(); 10];
    let mut memoryMap = MemoryMap {
        buffer_size: buffer.len(),
        buffer: &mut buffer,
        map_size: 0,
        map_key: 0,
        descriptor_size: 0,
        descriptor_version: 0,
    };

    let res = getMemoryMap(&mut memoryMap, SystemTable.BootServices());
    if res != EfiStatus::Success {
        panic!();
    }

    console.log(utf16!("getMemoryMap is done\0").as_ptr());
    let mut root_dir: *mut EfiFileProtocol = ptr::null_mut();
    open_root_dir(ImageHandle, &mut root_dir, SystemTable.BootServices());
    console.log(utf16!("open_root_dir is done\0").as_ptr());

    let mut memmap_file: *mut EfiFileProtocol = ptr::null_mut();

    unsafe{
        (*root_dir).open(&mut memmap_file, utf16!("memmap").as_ptr(), EFI_FILE_MODE_READ | EFI_FILE_MODE_WRITE | EFI_FILE_MODE_CREATE, 0);
    }

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
    loop {}
}
