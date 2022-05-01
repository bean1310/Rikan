use core::alloc::{GlobalAlloc, Layout};
use crate::uefi::{EfiBootServices, EfiMemoryType, EfiSimpleTextOutputProtocol};
use core::panic;
use core::ptr::{self, NonNull};

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
    // unsafe {
    //     COUT.unwrap().as_ref().OutputString(utf16!("inited\0").as_ptr());
    // }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {

        let memoryType = EfiMemoryType::EfiLoaderData;
        let size = layout.size();
        let align = layout.align();

        // unsafe {
        //     COUT.unwrap().as_ref().OutputString(utf16!("alloc\0").as_ptr());
        // }

        if align > 8 {
            // ここに入る場合ってどういう時だろう？
            // わからんので無視。まずければ考える。
            unsafe {
                COUT.unwrap().as_ref().OutputString(utf16!("align g8\0").as_ptr());
            }
            panic!()
            // ptr::null_mut()
        } else {
            // unsafe {
            //     COUT.unwrap().as_ref().OutputString(utf16!("align l8\0").as_ptr());
            // }
            let res = EFI_BOOT_SERVICES
                .unwrap()
                .as_ref()
                .allocate_pool(memoryType, size);

            // if res.is_err() {
            //     unsafe {
            //         COUT.unwrap().as_ref().OutputString(utf16!("ERR\0").as_ptr());
            //     }
            // }

            // if res.unwrap().is_null() {
            //     unsafe {
            //         COUT.unwrap().as_ref().OutputString(utf16!("NULLPTR\0").as_ptr());
            //     }
            // }

            res.unwrap()
        }
        // ptr::null_mut()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if layout.align() > 8 {
            // alloc同様に不明
            unsafe {
                COUT.unwrap().as_ref().OutputString(utf16!("dealloc\0").as_ptr());
            }
            panic!()
        } else {
            EFI_BOOT_SERVICES
            .unwrap()
            .as_ref().free_pool(ptr);
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