extern "C" {
    // I think u32s passed from host code should still be the correct types. We'll see.
    pub fn hello_from_host() -> (*mut u8, usize);
}

/// This function allocates some memory.
/// It returns a raw pointer to the allocated memory.
#[no_mangle]
pub extern "C" fn wasm_alloc(size: u32) -> *mut u8 {
    let mut buf: Vec<u8> = Vec::with_capacity(size.try_into().unwrap());
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn main() -> (*const u8, usize) {
    // Call our host function, turn the result into a String, 
    let hello_upper = {
        let (offset, len) = unsafe { hello_from_host() };
        let v: Vec<u8> = unsafe { Vec::from_raw_parts(offset, len, len) };
        let hello = String::from_utf8(v).unwrap();

        // Make a new string from the parts
        // The original result should be free'd appropriately. 
        hello.to_uppercase()
    };

    // Now make a pointer and return it.
    // We call forget to make super sure that Rust doesn't deallocate our funcion beforehand.
    let buf = hello_upper.as_bytes();
    let len = buf.len();
    let ptr = buf.as_ptr();
    std::mem::forget(buf);
    (ptr, len)
}