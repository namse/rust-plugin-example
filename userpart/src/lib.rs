type AdddType = unsafe extern "C" fn(usize, usize) -> usize;
extern "C" fn init_addd_type(_: usize, _: usize) -> usize {
    panic!("Function pointer not initialized");
}
static mut ADDD: unsafe extern "C" fn(usize, usize) -> usize = init_addd_type;
#[no_mangle]
pub fn set_addd(f: AdddType) {
    unsafe {
        ADDD = f;
    }
}

#[no_mangle]
pub fn add(left: usize, right: usize) -> usize {
    unsafe { ADDD(left, right) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
