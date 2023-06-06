extern "C" {
    pub fn send_string_to_host(ptr: *const u8, len: i32);
}

fn bytes_to_host(x: &[u8]) {
    let ptr = x.as_ptr();
    let len = x.len();

    unsafe {
        send_string_to_host(
            ptr, 
            len.try_into().unwrap()
        );
    }
}

#[no_mangle]
pub extern "C" fn foo() {
    let x = "Hello world".as_bytes();

    println!("From WASM: Starting...");

    // following: https://radu-matei.com/blog/practical-guide-to-wasm-memory/
    // we don't want the destructor for vec<> to be called, and apparently this will make Rust forget about this guy.
    // but the memory is still allocated.
    // std::mem::forget(x);

    bytes_to_host(x);

    // println!("From WASM: All done!");
    // println!("x is: {x:?}, ptr is: {ptr:?}, len is: {len}");
}