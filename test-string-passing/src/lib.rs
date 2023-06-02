// extern "C" {
//     pub fn send_string_to_host(ptr: u32, len: u32);
// }

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub extern "C" fn foo() -> u32 {
    // let x = "Hello world".as_bytes();
    // let ptr = unsafe { *x.as_ptr() };

    // following: https://radu-matei.com/blog/practical-guide-to-wasm-memory/
    // we don't want the destructor for vec<> to be called, and apparently this will make Rust forget about this guy.
    // but the memory is still allocated.
    // std::mem::forget(x);
    
    // (ptr.try_into().unwrap(), x.len().try_into().unwrap())

    // "Hello world".to_string()
    1
}
