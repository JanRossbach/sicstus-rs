//! High Level API for SICStus Prolog.
#![no_std]

extern crate alloc;
extern crate spin;

pub mod error;

pub mod sys;

#[cfg(feature = "allocator")]
mod allocator;

mod atom;
#[macro_use]
mod query;
mod term_ref;
mod util;

use core::fmt::Write;

use alloc::fmt;
use spin::Mutex;
pub use sys::{SP_atom, SP_integer, SP_pred_ref, SP_term_ref};
pub use term_ref::TermRef;

pub use atom::Atom;
pub use error::SicstusRsError;
pub use query::Predicate;


pub static WRITER: Mutex<Writer> = Mutex::new(Writer {});

pub struct Writer {}
impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let err_code = sicstus_sys::SP_printf(s);
        if err_code != 0 {
            return Err(fmt::Error);
        }
        Ok(())
    }
}
pub fn print(args: fmt::Arguments) {
    WRITER
        .lock()
        .write_fmt(args)
        .expect("print should be successful");
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

// #[cfg(not(test))]
// #[panic_handler]
// fn panic(_info: &core::panic::PanicInfo) -> ! {
//     use sicstus_sys::{SP_new_term_ref, SP_put_string, SP_raise_exception};

//     unsafe {
//         let term = SP_new_term_ref();
//         SP_put_string(term, "Panic: ".as_ptr() as *const i8);
//         SP_raise_exception(term);
//     }

//     loop {}
// }
