fn main() {
    println!("cargo:rustc-link-search=native=/home/namse/ffi-test/libpart/target/debug");
    println!("cargo:rustc-link-lib=static=libpart");
}
