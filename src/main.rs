#![no_std]
#![no_main]
#![feature(format_args_nl)]

use gb::println;

#[no_mangle]
pub extern fn main() {
    println!("Hello, Rust-GB!");
}

#[panic_handler]
#[allow(unused_variables)]
pub fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    loop {}
}
