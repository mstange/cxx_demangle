#![feature(link_args)]

#[link_args = "-s EXPORTED_FUNCTIONS=['_demangle'] -s MODULARIZE=1 -Oz -s NO_FILESYSTEM=1 -s NO_BROWSER=1  --memory-init-file 0 -s EXPORTED_RUNTIME_METHODS=['Pointer_stringify','stringToUTF8'] --closure 0 --llvm-lto 1 -s TOTAL_STACK=65536 -s TOTAL_MEMORY=16777216"]
extern {}

extern crate cpp_demangle;
use cpp_demangle::Symbol;
use std::os::raw::c_char;
use std::ptr;
use std::mem::forget;
use std::ffi::CStr;
use std::ffi::CString;

#[no_mangle]
pub unsafe extern "C" fn demangle(mangled: *const c_char) -> *const c_char {
    let mangled_s: &CStr = CStr::from_ptr(mangled);
    if let Ok(sym) = Symbol::new(mangled_s.to_bytes()) {
        let demangled = CString::new(format!("{}", sym)).unwrap();
        let demangled_c = demangled.as_ptr();
        forget(demangled);
        demangled_c
    } else {
        ptr::null()
    }
}

fn main() {
    /* Intentionally left blank */
}
