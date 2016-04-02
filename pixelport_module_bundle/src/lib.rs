#![feature(alloc_system)]
#![crate_type = "dylib"]

extern crate alloc_system;

#[no_mangle]
pub extern fn pixelport_package_init() {
    println!("Hello world");
}
