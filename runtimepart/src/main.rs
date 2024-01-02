extern "Rust" {
    fn addd(left: usize, right: usize) -> usize;
}

fn main() {
    unsafe {
        println!("addd(1,1) = {}", addd(1, 1));

        println!("Before loading library");
        let lib =
            libloading::Library::new("/home/namse/ffi-test/userpart/target/debug/libuserpart.so")
                .unwrap();
        println!("I got a library: {:?}", lib);

        type AdddType = unsafe extern "Rust" fn(usize, usize) -> usize;

        let func: libloading::Symbol<unsafe extern "Rust" fn(AdddType)> =
            lib.get(b"set_addd").unwrap();

        func(addd);

        let func: libloading::Symbol<unsafe extern "Rust" fn(usize, usize) -> usize> =
            lib.get(b"add").unwrap();
        println!("I got a symbol: {:?}", func);
        println!("func(1,1) = {}", func(1, 1));
    }
}
