#![feature(link_args)]

#[link_args = "-s EXPORTED_FUNCTIONS=['_demangle'] -s MODULARIZE=1 -s NO_EXIT_RUNTIME=1 -Os -s NO_FILESYSTEM=1 -s ELIMINATE_DUPLICATE_FUNCTIONS=1 -s NO_BROWSER=1  --memory-init-file 0 -s EXPORTED_RUNTIME_METHODS=['stringToUTF8'] --closure 1 --llvm-lto 1 -s TOTAL_STACK=65536 -s TOTAL_MEMORY=16777216"]
extern {}

extern crate cpp_demangle;
use cpp_demangle::Symbol;
use std::os::raw::c_char;
use std::slice;
use std::ffi::CStr;
use std::io::Write;

#[no_mangle]
pub unsafe extern "C" fn demangle(mangled: *const c_char,
                                  output_buf: *mut c_char,
                                  output_size: usize) -> usize {
    let mangled_s: &CStr = CStr::from_ptr(mangled);
    if let Ok(sym) = Symbol::new(mangled_s.to_bytes()) {
        let mut oslice = slice::from_raw_parts_mut(output_buf, output_size);
        match write!(oslice, "{}", sym) {
            Ok(()) => oslice.as_ptr() as usize - output_buf as usize,
            Err(_) => 0
        }
    } else {
        0
    }
}

fn main() {
    /* Intentionally left blank */
}
