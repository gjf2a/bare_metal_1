#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bare_metal_1::println;
use bare_metal_1::interrupts::PICS;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world!");
    println!("The numbers are {} and {}", 42, 1.0/3.0);

    bare_metal_1::interrupts::init_idt();
    unsafe {PICS.lock().initialize()};
    x86_64::instructions::interrupts::enable();

    println!("It did not crash!");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

