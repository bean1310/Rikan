use core::{ffi::c_void, ptr};

#[derive(PartialEq)]
#[repr(C)]
pub enum EfiStatus {
    Success = 0,
}

#[repr(C)]
pub struct EfiGuid {
    data_1: u32,
    data_2: u16,
    data_3: u16,
    data_4: [u8; 8],
}

pub const EFI_LOADED_IMAGE_PROTOCOL: EfiGuid = EfiGuid {
    data_1: 0x5b1b31a1,
    data_2: 0x9652,
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
pub const EFI_FILE_MODE_READ: u64 = 0x0000000000000001;
pub const EFI_FILE_MODE_WRITE: u64 = 0x0000000000000002;
pub const EFI_FILE_MODE_CREATE: u64 = 0x8000000000000000;

type Char16 = u16;
type NotImplemented = usize;

#[repr(C)]
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
pub struct EfiBootServices {
    hdr: EfiTableHeader,
    raise_tpl: NotImplemented,
    restore_tpl: NotImplemented,
    allocate_pages: NotImplemented,
    free_pages: NotImplemented,
    get_memory_map: extern "efiapi" fn(
        MemoryMapSize: *mut usize,
        MemoryMap: *mut [EfiMemoryDescriptor],
        MapKey: *mut usize,
        DescriptorSize: *mut usize,
        DescriptoraVersion: *mut u32,
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
    exit_boot_service: NotImplemented,
    get_next_monotonic_count: NotImplemented,
    stall: NotImplemented,
    set_watchdog_timer: NotImplemented,
    connect_controller: NotImplemented,
    disconnect_controller: NotImplemented,
    open_protocol: extern "efiapi" fn(
        handle: EfiHandle,
        protocol: *const EfiGuid,
        interface: *mut *mut c_void,
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
    pub fn get_memory_map(
        &self,
        memory_map_size: &mut usize,
        memory_map: &mut [EfiMemoryDescriptor],
        map_key: &mut usize,
        descriptor_size: &mut usize,
        descriptora_version: &mut u32,
    ) -> EfiStatus {
        (self.get_memory_map)(
            memory_map_size as *mut usize,
            memory_map as *mut [EfiMemoryDescriptor],
            // (MemoryMap as *mut [u8]) as *mut [EfiMemoryDescriptor],
            map_key as *mut usize,
            descriptor_size as *mut usize,
            descriptora_version as *mut u32,
        )
    }

    pub fn open_protocol(
        &self,
        handle: EfiHandle,
        protocol: &EfiGuid,
        interface: *mut *mut c_void,
        agent_handle: EfiHandle,
        controller_handle: EfiHandle,
        attributes: u32,
    ) -> EfiStatus {
        (self.open_protocol)(
            handle,
            protocol as *const EfiGuid,
            interface,
            agent_handle,
            controller_handle,
            attributes,
        )
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
}

#[derive(Default, Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiMemoryDescriptor {
    memory_type: u32,
    physical_start: EfiPhysicalAddress,
    virtual_start: EfiVirtualAddress,
    number_of_pages: u64,
    attribute: u64,
}

pub type EfiPhysicalAddress = u64;
pub type EfiVirtualAddress = u64;
pub struct EfiConfigurationTable {}

#[repr(C)]
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
    revision: u64,
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
        buffer: &c_void,
    ) -> EfiStatus,
    write: extern "efiapi" fn(
        this: &EfiFileProtocol,
        bufferSize: &usize,
        buffer: *const c_void,
    ) -> EfiStatus,
    get_position: extern "efiapi" fn(this: &EfiFileProtocol, position: &u64) -> EfiStatus,
    set_position: extern "efiapi" fn(this: &EfiFileProtocol, position: &u64) -> EfiStatus,
    get_info: extern "efiapi" fn(
        this: &EfiFileProtocol,
        infomationType: &EfiGuid,
        bufferSize: &usize,
        buffer: &c_void,
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

impl EfiFileProtocol {
    pub fn open(
        &self,
        new_handle: &mut *mut Self,
        file_name: *const Char16,
        open_mode: u64,
        attribute: u64,
    ) -> EfiStatus {
        (self.open)(self, new_handle, file_name, open_mode, attribute)
    }

    pub fn write(&self, buffer_size: usize, buffer: &str) -> EfiStatus {
        (self.write)(self, &buffer_size, buffer.as_ptr() as *const c_void)
    }
}

pub struct EfiDevicePathProtocol {}
#[repr(C)]
pub struct EfiLoadedImageProtocol<'a> {
    revision: u32,
    parent_handle: EfiHandle,
    system_table: EfiSystemTable,
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
    pub fn open_volume(&mut self, root: &mut *mut EfiFileProtocol) -> EfiStatus {
        (self.open_volume)(self, root)
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
