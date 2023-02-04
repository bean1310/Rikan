use crate::uefi::{EfiBootServices, EfiMemoryType, EfiSimpleTextOutputProtocol};
use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use core::panic;
use core::ptr::{NonNull};

use utf16_literal::utf16;

pub struct Allocator;

static mut EFI_BOOT_SERVICES: Option<NonNull<EfiBootServices>> = None;
static mut COUT: Option<NonNull<EfiSimpleTextOutputProtocol>> = None;

// メモ：Uefiが呼ばれたときにinit関数をcallして
// ここにBootserviceのアドレスを届ける必要がある
pub fn init(boot_services: &EfiBootServices, cout: &mut EfiSimpleTextOutputProtocol) {
    unsafe {
        EFI_BOOT_SERVICES = NonNull::new(boot_services as *const _ as *mut _);
        COUT = NonNull::new(cout as *const _ as *mut _);
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let memory_type = EfiMemoryType::EfiLoaderData;
        let size = layout.size();
        let align = layout.align();

        if align > 8 {
            COUT.unwrap()
                .as_ref()
                .output_string(utf16!("align g8\0").as_ptr());
            panic!()
        } else {
            let res = EFI_BOOT_SERVICES
                .unwrap()
                .as_ref()
                .allocate_pool(memory_type, size);

            res.unwrap()
        }
        // ptr::null_mut()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if layout.align() > 8 {
            // alloc同様に不明
            COUT.unwrap()
                .as_ref()
                .output_string(utf16!("dealloc\0").as_ptr());
            panic!()
        } else {
            let _ = EFI_BOOT_SERVICES.unwrap().as_ref().free_pool(ptr as *const c_void);
        }
    }
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

#[alloc_error_handler]
fn out_of_memory(layout: ::core::alloc::Layout) -> ! {
    panic!(
        "Ran out of free memory while trying to allocate {:#?}",
        layout
    );
}
