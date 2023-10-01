use crate::EfiSimpleTextOutputProtocol;
use alloc::string::String;
use alloc::vec::Vec;
use core::{
    fmt::{self, Write},
};


#[macro_export]
#[allow_internal_unsafe]
macro_rules! print {
    ($($arg:tt)*) => (unsafe{$crate::_print(format_args!($($arg)*))});
}

#[macro_export]
#[allow_internal_unsafe]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\r\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\r\n"), $($arg)*));
}

pub unsafe fn _print(args: fmt::Arguments) {
    CONSOLE.unwrap().write_fmt(args).unwrap();
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _text = String::from(s);
        let _full_text = _text + "\0";
        let _u16_str: Vec<u16> = _full_text.encode_utf16().into_iter().collect();
        unsafe {
            self.protocol
                .as_ref()
                .unwrap()
                .output_string(_u16_str.as_ptr());
        }
        Ok(())
    }
}

static mut CONSOLE: Option<Console> = None;

#[derive(Clone, Copy)]
pub struct Console {
    protocol: *const EfiSimpleTextOutputProtocol,
}

pub fn init(simple_text_output_proto: &mut EfiSimpleTextOutputProtocol) {
    simple_text_output_proto.reset(false);
        unsafe {
            CONSOLE = Some(Console {
                protocol: simple_text_output_proto as *const _,
            });
        }
}

