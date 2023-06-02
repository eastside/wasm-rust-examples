extern "C" {
    fn hello(a: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn lets_go() -> i32 {
    let out = unsafe { hello(1) };
    out + 1
}