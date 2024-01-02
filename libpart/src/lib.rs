// #[no_mangle]
// pub extern "Rust" fn addd(left: usize, right: usize) -> usize {
//     left + right
// }

#[no_mangle]
pub extern "C" fn addd(left: usize, right: usize) -> usize {
    left + right
}

// #[no_mangle]
// pub fn addd(left: usize, right: usize) -> usize {
//     left + right
// }
