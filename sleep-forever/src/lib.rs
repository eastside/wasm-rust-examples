#[no_mangle]
pub extern "C" fn sleep_forever() -> u32 {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
