use core::{ffi::c_void};

#[repr(C)]
pub enum EfiStatus {
    Success = 0,
}

type Char16 = u16;

#[repr(C)]
pub struct EfiTableHeader {
    Signature: u64,
    Revision: u32,
    HeaderSize: u32,
    Crc32: u32,
    Reserved: u32,
}

pub struct EfiHandle(*mut c_void);

pub struct EfiSimpleTextOutputProtocol {
    Reset: extern "efiapi" fn(
        This: &EfiSimpleTextOutputProtocol,
        ExtendedVerification: bool,
    ) -> EfiStatus,
    OutputString: extern "efiapi" fn(
        This: &EfiSimpleTextOutputProtocol,
        String: *const Char16,
    ) -> EfiStatus,
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
pub struct EfiBootServices {
    Hdr: EfiTableHeader,
    RaiseTPL: usize,
    RestoreTPL: usize,
    AllocatePages: usize,
    FreePages: usize,
    GetMemoryMap: extern "efiapi" fn(
        MemoryMapSize: *mut usize,
        MemoryMap: *mut [EfiMemoryDescriptor],
        MapKey: *mut usize,
        DescriptorSize: *mut usize,
        DescriptoraVersion: *mut u32,
    ) -> EfiStatus,
    AllocatePool: usize,
    FreePool: usize,
}

impl EfiBootServices {
    pub fn GetMemoryMap(
        &self,
        MemoryMapSize: &mut usize,
        MemoryMap: &mut [u8],
        MapKey: &mut usize,
        DescriptorSize: &mut usize,
        DescriptoraVersion: &mut u32,
    ) -> EfiStatus {
        unsafe {
            (self.GetMemoryMap)(
                MemoryMapSize as *mut usize,
                // MemoryMap as *mut [EfiMemoryDescriptor],
                (MemoryMap as *mut [u8]) as *mut [EfiMemoryDescriptor],
                MapKey as *mut usize,
                DescriptorSize as *mut usize,
                DescriptoraVersion as *mut u32,
            )
        }
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
pub struct SystemTable {
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
    BootServices: *mut EfiBootServices,
    NumberOfTableEntries: usize,
    EConfigurationTable: *mut EfiConfigurationTable,
}

impl SystemTable {
    pub fn ConOut(&self) -> &mut EfiSimpleTextOutputProtocol {
        unsafe { &mut *self.ConOut }
    }

    pub fn BootServices(&self) -> &EfiBootServices {
        unsafe { &*self.BootServices }
    }
}
