use core::ffi::c_void;

#[repr(C)]
pub enum EFI_STATUS {
    Success = 0
}

type CHAR16 = u16;

#[repr(C)]
pub struct EFI_TABLE_HEADER{
    Signature:  u64,
    Revision:   u32,
    HeaderSize: u32,
    CRC32:      u32,
    Reserved:   u32
}

pub struct EFI_HANDLE(*mut c_void);

pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    Reset: extern "efiapi" fn(This: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, ExtendedVerification: bool) -> EFI_STATUS,
    OutputString: extern "efiapi" fn(This: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, String: *const CHAR16) -> EFI_STATUS,
    _Unuse0: u64,
    _Unuse1: u64,
    _Unuse2: u64,
    _Unuse3: u64,
    _Unuse4: u64,
    _Unuse5: u64,
    _Unuse6: u64,
    _Unuse7: u64
}

impl EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub fn Reset(&self, ExtendedVerification: bool) -> EFI_STATUS {
        unsafe {(self.Reset)(self, ExtendedVerification)}
    }

    pub fn OutputString(&self, String: *const CHAR16) -> EFI_STATUS {
        unsafe{(self.OutputString)(self, String)}
    }
}

pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL{}
pub struct EFI_RUNTIME_SERVICES{}
pub struct EFI_BOOT_SERVICES{}
pub struct EFI_CONFIGURATION_TABLE{}

#[repr(C)]
pub struct SystemTable {
    Hdr:                    EFI_TABLE_HEADER,
    FirmwareVendor:         *const CHAR16,
    FirmwareRevision:       u32,
    ConsoleInHandle:        EFI_HANDLE,
    ConIn:                  *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    ConsoleOutHandle:       EFI_HANDLE,
    ConOut:                 *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    StandardErrorHandle:    EFI_HANDLE,
    StdErr:                 *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    RuntimeServices:        *mut EFI_RUNTIME_SERVICES,
    BootServices:           *mut EFI_BOOT_SERVICES,
    NumberOfTableEntries:   usize,
    EConfigurationTable:    *mut EFI_CONFIGURATION_TABLE
}

impl SystemTable {
    pub fn ConOut(&self) -> &mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
        unsafe {&mut *self.ConOut}
    }
}