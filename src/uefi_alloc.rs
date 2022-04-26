use core::alloc::{GlobalAlloc, Layout};
use crate::uefi::{EfiBootServices, EfiMemoryType};
use core::{ptr, panic};

pub struct Allocator;

static mut efi_boot_services: Option<&EfiBootServices<'static>> = None;

// メモ：Uefiが呼ばれたときにinit関数をcallして
// ここにBootserviceのアドレスを届ける必要がある
pub fn init(boot_services: &'static EfiBootServices) {
    unsafe {efi_boot_services = Some(boot_services);}
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let memoryType = EfiMemoryType::EfiLoaderData;
        let size = layout.size();
        let align = layout.align();

        if align > 8 {
            // ここに入る場合ってどういう時だろう？
            // わからんので無視。まずければ考える。
            panic!()
        } else {
            efi_boot_services
                .unwrap()
                .allocate_pool(memoryType, size)
                .unwrap_or(ptr::null_mut())
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if layout.align() > 8 {
            // alloc同様に不明
            panic!()
        } else {
            efi_boot_services.unwrap().free_pool(ptr);
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