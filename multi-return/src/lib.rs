#[no_mangle]
pub fn flip(a: u32, b: u32) -> (u32, u32) {
    (b, a)
}