use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::{self, Vec};
use core::ops::Index;
use core::panic;
use core::ptr::{null, null_mut};
use core::{ffi::c_void, ptr};

use crate::{print, KERNEL_BASE_ADDRESS};
use crate::println;

#[derive(PartialEq, Debug, Clone, Copy)]
#[repr(C)]
pub enum EfiStatus {
    Success,
    LoadError,
    InvalidParameter,
    Unsupprted,
    BadBufferSize,
    BufferTooSmall,
    NotReady,
    DeviceError,
    WriteProtected,
    OutOfResources,
    VolumeCorrupted,
    VolumeFull,
    NoMedia,
    MediaChanged,
    NotFound,
    AccessDenied,
    NoResponse,
    NoMapping,
    Timeout,
    NotStarted,
    AlreadyStarted,
    Aborted,
    IcmpError,
    TftpError,
    ProtocolError,
    IncompatibleVersion,
    SecurityViolation,
    CrcError,
    EndOfMedia,
    EndOfFile = 31,
    InvalidLanguage,
    CompromisedData,
    IpAddressConflict,
    HttpError,
}

#[repr(C)]
#[derive(Debug)]
pub struct EfiGuid {
    data_1: u32,
    data_2: u16,
    data_3: u16,
    data_4: [u8; 8],
}

pub const EFI_LOADED_IMAGE_PROTOCOL: EfiGuid = EfiGuid {
    data_1: 0x5b1b31a1,
    data_2: 0x9562,
    data_3: 0x11d2,
    data_4: [0x8e, 0x3f, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
};

pub const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID: EfiGuid = EfiGuid {
    data_1: 0x0964e5b22,
    data_2: 0x6459,
    data_3: 0x11d2,
    data_4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
};

pub const EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL: u32 = 0x00000001;

#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub enum EfiFileAttribute {
    // This value is NOT defined on UEFI Spec.
    None = 0x0,
    // These values below are defined on UEFI Spec.
    ReadOnly = 0x1,
    Hidden = 0x2,
    System = 0x4,
    Reserved = 0x8,
    Directory = 0x10,
    Archive = 0x20,
    ValidAttribute = 0x37,
}

type Char16 = u16;

type NotImplemented = usize;

#[repr(C)]
#[derive(Debug)]
pub struct EfiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

pub type EfiHandle = *const c_void;

pub struct EfiSimpleTextOutputProtocol {
    reset: extern "efiapi" fn(
        This: &EfiSimpleTextOutputProtocol,
        ExtendedVerification: bool,
    ) -> EfiStatus,
    output_string:
        extern "efiapi" fn(This: &EfiSimpleTextOutputProtocol, String: *const Char16) -> EfiStatus,
    _unuse0: usize,
    _unuse1: usize,
    _unuse2: usize,
    _unuse3: usize,
    _unuse4: usize,
    _unuse5: usize,
    _unuse6: usize,
    _unuse7: usize,
}

impl EfiSimpleTextOutputProtocol {
    pub fn reset(&self, extended_verification: bool) -> EfiStatus {
        (self.reset)(self, extended_verification)
    }

    pub fn output_string(&self, string: *const Char16) -> EfiStatus {
        (self.output_string)(self, string)
    }
}

pub struct EfiSimpleTextInputProtocol {}
pub struct EfiRuntimeService {}

#[repr(C)]
pub enum EfiAllocateType{
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType
}

#[repr(C)]
pub struct EfiBootServices {
    hdr: EfiTableHeader,
    raise_tpl: NotImplemented,
    restore_tpl: NotImplemented,
    allocate_pages: extern "efiapi" fn(
        allocate_type: EfiAllocateType,
        memory_type: EfiMemoryType,
        pages: usize,
        memory: &EfiPhysicalAddress
    ) -> EfiStatus,
    free_pages: NotImplemented,
    get_memory_map: extern "efiapi" fn(
        MemoryMapSize: &mut usize,
        MemoryMap: *mut u8,
        MapKey: &mut usize,
        DescriptorSize: &mut usize,
        DescriptoraVersion: &mut u32,
    ) -> EfiStatus,
    allocate_pool:
        extern "efiapi" fn(pooltype: EfiMemoryType, size: usize, buffer: &mut *mut u8) -> EfiStatus,
    free_pool: extern "efiapi" fn(address: *mut u8) -> EfiStatus,
    create_event: NotImplemented,
    set_timer: NotImplemented,
    wait_for_event: NotImplemented,
    signal_event: NotImplemented,
    close_event: NotImplemented,
    check_event: NotImplemented,
    install_protocol_interface: NotImplemented,
    reinstall_protocol_interface: NotImplemented,
    uninstall_protocol_interface: NotImplemented,
    handle_protocol: NotImplemented,
    reserved: NotImplemented,
    register_protocol_notify: NotImplemented,
    locate_handle: NotImplemented,
    locate_device_path: NotImplemented,
    install_configuration_table: NotImplemented,
    load_image: NotImplemented,
    start_image: NotImplemented,
    exit: NotImplemented,
    unload_image: NotImplemented,
    exit_boot_service: 
        extern "efiapi" fn(image_handle: EfiHandle, map_key: usize) -> EfiStatus,
    get_next_monotonic_count: NotImplemented,
    stall: NotImplemented,
    set_watchdog_timer: NotImplemented,
    connect_controller: NotImplemented,
    disconnect_controller: NotImplemented,
    open_protocol: extern "efiapi" fn(
        handle: EfiHandle,
        protocol: &EfiGuid,
        interface: &mut *mut c_void,
        agentHandle: EfiHandle,
        controllerHandle: EfiHandle,
        attributes: u32,
    ) -> EfiStatus,
    close_protocol: extern "efiapi" fn(
        handle: EfiHandle,
        protocol: *const EfiGuid,
        agentHandle: EfiHandle,
        cotrollerHandle: EfiHandle,
    ) -> EfiStatus,
    open_protocol_infomation: NotImplemented,
    protocols_per_handle: NotImplemented,
    locate_handle_buffer: NotImplemented,
    locate_protocol: NotImplemented,
    install_multiple_protocol_interface: NotImplemented,
    uninstall_multiple_protocol_interface: NotImplemented,
    calculate_crc32: NotImplemented,
    copy_mem: NotImplemented,
    set_mem: NotImplemented,
    create_event_ex: NotImplemented,
}

impl EfiBootServices {
    /// # Arguments
    /// * `memory_map_buffer` EfiMemoryDescriptor型の書き込まれる先のbuffer
    pub fn get_memory_map(
        &self,
        memory_map_buffer: &mut [u8],
    ) -> Result<(usize, usize, usize, u32), EfiStatus> {
        let mut memory_map_size = core::mem::size_of::<u8>() * memory_map_buffer.len();
        // let buffer_ptr = memory_map_buffer.as_mut_ptr();
        let mut map_key = 0;
        let mut descriptor_size = 0;
        let mut descriptor_version = 0;
        let _res = (self.get_memory_map)(
            &mut memory_map_size,
            memory_map_buffer.as_mut_ptr(),
            &mut map_key,
            &mut descriptor_size,
            &mut descriptor_version,
        );

        if _res == EfiStatus::Success {
            Ok((
                memory_map_size,
                map_key,
                descriptor_size,
                descriptor_version,
            ))
        } else {
            Err(_res)
        }
    }

    pub unsafe fn open_protocol(
        &self,
        handle: EfiHandle,
        protocol: &EfiGuid,
        agent_handle: EfiHandle,
        controller_handle: EfiHandle,
        attributes: u32,
    ) -> Result<&c_void, EfiStatus> {
        let mut _interface: *mut c_void = null_mut();
        let interface_ptr = &mut _interface;

        // println!("{:?}", handle);
        // println!("{:?}", protocol);
        // println!("{:?}", agent_handle);
        // println!("{:?}", controller_handle);
        // println!("{:?}", attributes);
        let _res = (self.open_protocol)(
            handle,
            protocol,
            interface_ptr,
            agent_handle,
            controller_handle,
            attributes,
        );

        if _res == EfiStatus::Success {
            if interface_ptr.is_null() {
                println!("RETURN NULL");
            }
            Ok(interface_ptr.as_ref().unwrap())
        } else {
            Err(_res)
        }
    }

    pub fn close_protocol(
        &self,
        handle: EfiHandle,
        protocol: *const EfiGuid,
        agent_handle: EfiHandle,
        controller_handle: EfiHandle,
    ) -> EfiStatus {
        (self.close_protocol)(handle, protocol, agent_handle, controller_handle)
    }

    pub fn allocate_pool(&self, pooltype: EfiMemoryType, size: usize) -> Result<*mut u8, ()> {
        let mut buffer = ptr::null_mut();
        let buffer_ptr = &mut buffer;
        if (self.allocate_pool)(pooltype, size, buffer_ptr) as i32 == 0 {
            assert!(!((*buffer_ptr).is_null()));
            Ok(*buffer_ptr)
        } else {
            Err(())
        }
    }

    pub fn free_pool(&self, buffer: *mut u8) -> Result<(), ()> {
        if (self.free_pool)(buffer) == EfiStatus::Success {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn allocate_pages(
        &self, 
        allocate_type: EfiAllocateType,
        memory_type: EfiMemoryType,
        mut pages: usize,
        mut memory: EfiPhysicalAddress
    ) -> Result<EfiPhysicalAddress, EfiStatus> {

        if (pages % 0x1000) != 0 {
            // 4KiB alignment
            pages = (pages + 0xfff) / 0x1000
        }

        let _res = (self.allocate_pages)(allocate_type, memory_type, pages, &mut memory);

        if _res == EfiStatus::Success {
            Ok(memory)
        } else {
            Err(_res)
        }
    }

    pub fn exit_boot_service(
        &self, 
        image_handle: EfiHandle,
        map_key: usize
    ) -> Result<EfiStatus, EfiStatus> {
        let _res = (self.exit_boot_service)(image_handle, map_key);

        if _res == EfiStatus::Success {
            Ok(_res)
        } else {
            Err(_res)
        }
    }
}

#[derive(Default, Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiMemoryDescriptor {
    pub memory_type: u32,
    pub physical_start: EfiPhysicalAddress,
    pub virtual_start: EfiVirtualAddress,
    pub number_of_pages: u64,
    pub attribute: u64,
}

pub type EfiPhysicalAddress = u64;
pub type EfiVirtualAddress = u64;
pub struct EfiConfigurationTable {}

#[repr(C)]
#[derive(Debug)]
pub struct EfiSystemTable {
    hdr: EfiTableHeader,
    firmware_vendor: *const Char16,
    firmware_revision: u32,
    console_in_handle: EfiHandle,
    con_in: *mut EfiSimpleTextInputProtocol,
    console_out_handle: EfiHandle,
    con_out: *mut EfiSimpleTextOutputProtocol,
    standard_error_handle: EfiHandle,
    std_err: *mut EfiSimpleTextOutputProtocol,
    runtime_services: *mut EfiRuntimeService,
    boot_services: *mut EfiBootServices,
    number_of_table_entries: usize,
    econfiguration_table: *mut EfiConfigurationTable,
}

pub struct EfiFileIoToken {}

#[repr(C)]
pub struct EfiFileProtocol {
    pub revision: u64,
    open: extern "efiapi" fn(
        this: &EfiFileProtocol,
        newHandle: &mut *mut EfiFileProtocol,
        fileName: *const Char16,
        openMode: u64,
        attribute: u64,
    ) -> EfiStatus,
    close: extern "efiapi" fn(this: &EfiFileProtocol) -> EfiStatus,
    delete: extern "efiapi" fn(this: &EfiFileProtocol) -> EfiStatus,
    read: extern "efiapi" fn(
        this: &EfiFileProtocol,
        bufferSize: &usize,
        buffer: *mut c_void,
    ) -> EfiStatus,
    write: extern "efiapi" fn(
        this: &EfiFileProtocol,
        bufferSize: &mut usize,
        buffer: *const c_void,
    ) -> EfiStatus,
    get_position: extern "efiapi" fn(this: &EfiFileProtocol, position: &u64) -> EfiStatus,
    set_position: extern "efiapi" fn(this: &EfiFileProtocol, position: &u64) -> EfiStatus,
    get_info: extern "efiapi" fn(
        this: &EfiFileProtocol,
        infomationType: &EfiGuid,
        bufferSize: &usize,
        buffer: *mut c_void,
    ) -> EfiStatus,
    set_info: extern "efiapi" fn(
        this: &EfiFileProtocol,
        infomationType: &EfiGuid,
        bufferSize: &usize,
        buffer: &c_void,
    ) -> EfiStatus,
    flash: extern "efiapi" fn(this: &EfiFileProtocol) -> EfiStatus,
    open_ex: extern "efiapi" fn(
        this: &EfiFileProtocol,
        newHandle: &&EfiFileProtocol,
        fileName: &Char16,
        openMode: u64,
        attribute: u64,
        token: &EfiFileIoToken,
    ) -> EfiStatus,
    read_ex: extern "efiapi" fn(this: &EfiFileProtocol, token: &EfiFileIoToken) -> EfiStatus,
    write_ex: extern "efiapi" fn(this: &EfiFileProtocol, token: &EfiFileIoToken) -> EfiStatus,
    flash_ex: extern "efiapi" fn(this: &EfiFileProtocol, token: &EfiFileIoToken) -> EfiStatus,
}

// note:
// uefi-rsでは、使い方を3つに絞ってやってた
// ありっちゃあり。なので採用した。
#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub enum EfiFileOpenMode {
    Read = 0x1,
    ReadWrite = 0x2 | 0x1,
    CreateReadWrite = 0x8000_0000_0000_0000 | 0x1 | 0x2,
}

pub const EFI_FILE_INFO_ID: EfiGuid = EfiGuid {
    data_1: 0x09576e92,
    data_2: 0x6d3f,
    data_3: 0x11d2,
    data_4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiTime {
    year: u64,
    month: u8,
    day: u8, 
    hour: u8,
    minute: u8,
    second: u8,
    _pad1: u8,
    nanosecond: u32,
    time_zone: i16,
    daylight: u8,
    _pad2: u8
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct EfiFileInfo {
    pub size: u64,
    pub file_size: u64,
    pub physical_size: u64,
    pub create_time: EfiTime,
    pub last_access_time: EfiTime, 
    pub modification_time: EfiTime,
    pub attribute: u64
}

impl EfiFileProtocol {
    pub fn close(&self) -> Result<EfiStatus, EfiStatus> {
        let _res = (self.close)(self);
        if _res == EfiStatus::Success {
            Ok(_res)
        } else {
            Err(_res)
        }
    }

    pub fn get_info(&self, file_name: &str) -> Result<EfiFileInfo, EfiStatus> {
        let file_name_len = file_name.len();
        let mut buffer: Box<[u8]> = Box::new([0; 1024]);
        let buffer_ptr = buffer.as_mut_ptr() as *mut c_void;
        let _res = (self.get_info)(self, &EFI_FILE_INFO_ID, &(buffer.len()), buffer_ptr);
        if _res == EfiStatus::Success {
            let file_info = unsafe {(buffer.as_ptr() as *const EfiFileInfo).as_ref().unwrap()};
            Ok(*file_info)
        } else {
            Err(_res)
        }
    }

    pub fn open(
        &self,
        file_name: &str,
        open_mode: EfiFileOpenMode,
        attribute: EfiFileAttribute,
    ) -> Result<&EfiFileProtocol, EfiStatus> {
        let mut new_handle = ptr::null_mut();
        let new_handle_ptr = &mut new_handle;

        let _text = String::from(file_name);
        let _null_terminated_text = _text + "\0";
        let u16_str: Vec<u16> = _null_terminated_text.encode_utf16().into_iter().collect();
        let u16ed_filename_ptr = u16_str.as_ptr();

        let _res = (self.open)(
            &self,
            new_handle_ptr,
            u16ed_filename_ptr,
            open_mode as u64,
            attribute as u64,
        );
        if _res == EfiStatus::Success {
            unsafe { Ok(new_handle.as_ref().unwrap()) }
        } else {
            Err(_res)
        }
    }

    // ead: extern "efiapi" fn(
    //     this: &EfiFileProtocol,
    //     bufferSize: &usize,
    //     buffer: &c_void,
    // ) -> EfiStatus,
    pub fn read(&self, buffer_size: usize) -> Result<EfiStatus, EfiStatus> {
        let _kernel_load_address = KERNEL_BASE_ADDRESS as *mut u64;
        let res = (self.read)(self, &buffer_size, _kernel_load_address as *mut _);

        if res == EfiStatus::Success {
            Ok(res)
        } else {
            Err(res)
        }
    }

    pub fn write(&self, buffer_size: usize, buffer: &str) -> Result<usize, EfiStatus> {
        let mut written_buffer_size = buffer_size;
        let _res = (self.write)(self, &mut written_buffer_size, buffer.as_ptr() as *const _);
        if _res == EfiStatus::Success {
            Ok(written_buffer_size)
        } else {
            Err(_res)
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct EfiDevicePathProtocol {}

#[repr(C)]
pub struct EfiLoadedImageProtocol<'a> {
    revision: u32,
    parent_handle: EfiHandle,
    system_table: &'a EfiSystemTable,
    pub device_handle: EfiHandle,
    file_path: &'a EfiDevicePathProtocol,
    reserved: &'a c_void,
    load_options_size: u32,
    load_options: &'a c_void,
    image_base: &'a c_void,
    image_size: u64,
    image_code_type: EfiMemoryType,
    image_data_type: EfiMemoryType,
    unload: extern "efiapi" fn(imageHandle: EfiHandle) -> EfiStatus,
}

#[repr(C)]
#[derive(Debug)]
pub enum EfiMemoryType {
    EfiReservedMemoryType = 0,
    EfiLoaderCode = 1,
    EfiLoaderData = 2,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiUnacceptedMemoryType,
    EfiMaxMemoryType,
}

#[repr(C)]
pub struct EfiSimpleFileSystemProtocol {
    revision: u64,
    open_volume: extern "efiapi" fn(
        this: &EfiSimpleFileSystemProtocol,
        root: &mut *mut EfiFileProtocol,
    ) -> EfiStatus,
}

impl EfiSimpleFileSystemProtocol {
    pub unsafe fn open_volume(&self) -> Result<&EfiFileProtocol, EfiStatus> {
        let mut efi_file_proto = ptr::null_mut();
        let mut efi_file_proto_ptr = &mut efi_file_proto;
        let _res = (self.open_volume)(self, efi_file_proto_ptr);
        if _res == EfiStatus::Success {
            Ok(efi_file_proto_ptr.as_ref().unwrap())
        } else {
            Err(_res)
        }
    }
}

impl EfiSystemTable {
    pub fn con_out(&self) -> &mut EfiSimpleTextOutputProtocol {
        unsafe { &mut *self.con_out }
    }

    pub fn boot_services(&self) -> &EfiBootServices {
        unsafe { &*self.boot_services }
    }
}

// std環境と違ってなぜか呼び出し側から見るポインタの先の値が変わってしまう
// fn str_to_uefi_utf16(text: &str) -> Vec<u16> {
//     let _text = String::from(text);
//     let _null_terminated_text = _text + "\0";
//     let u16_str: Vec<u16> = _null_terminated_text.encode_utf16().into_iter().collect();
//     println!("DEBUG: {:p}: {:x}", u16_str.as_ptr(), *(u16_str.as_ptr()));
//     u16_str
// }
