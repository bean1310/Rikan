use core::{ffi::c_void, ops::Not, task::Context};

#[repr(C)]
pub enum EfiStatus {
    Success = 0,
}

#[repr(C)]
pub struct EfiGuid {
    data_1: u32,
    date_2: u16,
    date_3: u16,
    date_4: [u8;8]
}

pub const EFI_LOADED_IMAGE_PROTOCOL: EfiGuid = EfiGuid {
    data_1: 0x5b1b31a1,
    date_2: 0x9652,
    date_3: 0x11d2,
    date_4: [0x8e, 0x3f, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]
};

pub const EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL: u32 = 0x00000001;

type Char16 = u16;
type NOT_IMPLEMENTED = usize;

#[repr(C)]
pub struct EfiTableHeader {
    Signature: u64,
    Revision: u32,
    HeaderSize: u32,
    Crc32: u32,
    Reserved: u32,
}

pub type EfiHandle = *const c_void;

pub struct EfiSimpleTextOutputProtocol {
    Reset: extern "efiapi" fn(
        This: &EfiSimpleTextOutputProtocol,
        ExtendedVerification: bool,
    ) -> EfiStatus,
    OutputString:
        extern "efiapi" fn(This: &EfiSimpleTextOutputProtocol, String: *const Char16) -> EfiStatus,
    _Unuse0: usize,
    _Unuse1: usize,
    _Unuse2: usize,
    _Unuse3: usize,
    _Unuse4: usize,
    _Unuse5: usize,
    _Unuse6: usize,
    _Unuse7: usize,
}

impl EfiSimpleTextOutputProtocol {
    pub fn Reset(&self, ExtendedVerification: bool) -> EfiStatus {
        unsafe { (self.Reset)(self, ExtendedVerification) }
    }

    pub fn OutputString(&self, String: *const Char16) -> EfiStatus {
        unsafe { (self.OutputString)(self, String) }
    }
}

pub struct EfiSimpleTextInputProtocol {}
pub struct EfiRuntimeService {}
#[repr(C)]
pub struct EfiBootServices<'a> {
    hdr: EfiTableHeader,
    raise_tpl: NOT_IMPLEMENTED,
    restore_tpl: NOT_IMPLEMENTED,
    allocate_pages: NOT_IMPLEMENTED,
    free_pages: NOT_IMPLEMENTED,
    get_memory_map: extern "efiapi" fn(
        MemoryMapSize: *mut usize,
        MemoryMap: *mut [EfiMemoryDescriptor],
        MapKey: *mut usize,
        DescriptorSize: *mut usize,
        DescriptoraVersion: *mut u32,
    ) -> EfiStatus,
    allocate_pool: NOT_IMPLEMENTED,
    free_pool: NOT_IMPLEMENTED,
    create_event: NOT_IMPLEMENTED,
    set_timer: NOT_IMPLEMENTED,
    wait_for_event: NOT_IMPLEMENTED,
    signal_event: NOT_IMPLEMENTED,
    close_event: NOT_IMPLEMENTED,
    check_event: NOT_IMPLEMENTED,
    install_protocol_interface: NOT_IMPLEMENTED,
    reinstall_protocol_interface: NOT_IMPLEMENTED,
    uninstall_protocol_interface: NOT_IMPLEMENTED,
    handle_protocol: NOT_IMPLEMENTED,
    reserved: &'a c_void,
    register_protocol_notify: NOT_IMPLEMENTED,
    locate_handle: NOT_IMPLEMENTED,
    locate_device_path: NOT_IMPLEMENTED,
    install_configuration_table: NOT_IMPLEMENTED,
    load_image: NOT_IMPLEMENTED,
    start_image: NOT_IMPLEMENTED,
    exit: NOT_IMPLEMENTED,
    unload_image: NOT_IMPLEMENTED,
    exit_boot_service: NOT_IMPLEMENTED,
    get_next_monotonic_count: NOT_IMPLEMENTED,
    stall: NOT_IMPLEMENTED,
    set_watchdog_timer: NOT_IMPLEMENTED,
    connect_controller: NOT_IMPLEMENTED,
    disconnect_controller: NOT_IMPLEMENTED,
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
    open_protocol_infomation: NOT_IMPLEMENTED,
    protocols_per_handle: NOT_IMPLEMENTED,
    locate_handle_buffer: NOT_IMPLEMENTED,
    locate_protocol: NOT_IMPLEMENTED,
    install_multiple_protocol_interface: NOT_IMPLEMENTED,
    uninstall_multiple_protocol_interface: NOT_IMPLEMENTED,
    calculate_crc32: NOT_IMPLEMENTED,
    copy_mem: NOT_IMPLEMENTED,
    set_mem: NOT_IMPLEMENTED,
    create_event_ex: NOT_IMPLEMENTED,
}

impl EfiBootServices<'static> {
    pub fn get_memory_map(
        &self,
        MemoryMapSize: &mut usize,
        MemoryMap: &mut [u8],
        MapKey: &mut usize,
        DescriptorSize: &mut usize,
        DescriptoraVersion: &mut u32,
    ) -> EfiStatus {
        unsafe {
            (self.get_memory_map)(
                MemoryMapSize as *mut usize,
                // MemoryMap as *mut [EfiMemoryDescriptor],
                (MemoryMap as *mut [u8]) as *mut [EfiMemoryDescriptor],
                MapKey as *mut usize,
                DescriptorSize as *mut usize,
                DescriptoraVersion as *mut u32,
            )
        }
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
        unsafe {
            (self.open_protocol)(
                handle,
                protocol as *const EfiGuid,
                interface,
                agent_handle,
                controller_handle,
                attributes,
            )
        }
    }

    pub fn close_protocol(
        &self,
        handle: EfiHandle,
        protocol: *const EfiGuid,
        agent_handle: EfiHandle,
        controller_handle: EfiHandle,
    ) -> EfiStatus {
        unsafe { (self.close_protocol)(handle, protocol, agent_handle, controller_handle) }
    }
}

#[repr(C)]
pub struct EfiMemoryDescriptor {
    Type: u32,
    PhysicalStart: EfiPhysicalAddress,
    VirtualStart: EfiVirtualAddress,
    NumberOfPages: u64,
    Attribute: u64,
}

pub type EfiPhysicalAddress = u64;
pub type EfiVirtualAddress = u64;
pub struct EfiConfigurationTable {}

#[repr(C)]
pub struct EfiSystemTable<'a> {
    Hdr: EfiTableHeader,
    FirmwareVendor: *const Char16,
    FirmwareRevision: u32,
    ConsoleInHandle: EfiHandle,
    ConIn: *mut EfiSimpleTextInputProtocol,
    ConsoleOutHandle: EfiHandle,
    ConOut: *mut EfiSimpleTextOutputProtocol,
    StandardErrorHandle: EfiHandle,
    StdErr: *mut EfiSimpleTextOutputProtocol,
    RuntimeServices: *mut EfiRuntimeService,
    BootServices: *mut EfiBootServices<'a>,
    NumberOfTableEntries: usize,
    EConfigurationTable: *mut EfiConfigurationTable,
}

pub struct EfiFileIoToken {}
pub struct EfiFileProtocol {
    revision: u64,
    open: extern "efiapi" fn(
        this: &EfiFileProtocol,
        newHandle: &(&EfiFileProtocol),
        fileName: &Char16,
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
        buffer: &c_void,
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

pub struct efiDevicePathProtocol{}
#[repr(C)]
pub struct EfiLoadedImageProtocol<'a> {
    revision: u32,
    parent_handle: EfiHandle,
    system_table: EfiSystemTable<'a>,
    device_handle: EfiHandle,
    file_path: &'a efiDevicePathProtocol,
    reserved: &'a c_void,
    load_options_size: u32,
    load_options: &'a c_void,
    image_base: &'a c_void,
    image_size: u64,
    image_code_type: EfiMemoryType,
    image_data_type: EfiMemoryType,
    unload: extern "efiapi" fn(imageHandle: EfiHandle) -> EfiStatus,
}

pub enum EfiMemoryType {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
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
        root: &&EfiFileProtocol,
    ) -> EfiStatus,
}

impl EfiSystemTable<'static> {
    pub fn ConOut(&self) -> &mut EfiSimpleTextOutputProtocol {
        unsafe { &mut *self.ConOut }
    }

    pub fn BootServices(&self) -> &EfiBootServices<'static> {
        unsafe { &*self.BootServices }
    }
}
