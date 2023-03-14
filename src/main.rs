#![no_std]
#![no_main]

use core::panic::PanicInfo;

// mod boot {
//     use core::arch::global_asm;

//     global_asm!(".section .text._start");
// }

/// 3F20_0008 = GPIOFSEL2 1<<3 pin21 as output
/// 3F20_001C = 1<<21 pin21 on
/// 3F20_0028 = 1<<21 pin21 off

#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        core::ptr::write_volatile(0x3f20_0008 as *mut u32, 1 << 3);

        loop {
            // Turn ON
            core::ptr::write_volatile(0x3F20_001C as *mut u32, 1 << 21);

            for _ in 1..30000 {
                core::arch::asm!("nop");
            }

            // Turn OFF
            core::ptr::write_volatile(0x3F20_0028 as *mut u32, 1 << 21);
            for _ in 1..30000 {
                core::arch::asm!("nop");
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
