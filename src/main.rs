#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

static HELLO: &[u8] = b"Hello World!";

use core::fmt::Write;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("Hello world{}", "!");

    panic!("FUCKh");

    loop {}
}
