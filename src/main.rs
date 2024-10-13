#![no_std]
#![no_main]
#![feature(used_with_arg)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const ALIGN: i32 = 1 << 0;
const MEMINFO: i32 = 1 << 1;
const MAGIC: i32 = 0x1BADB002;
const FLAGS: i32 = ALIGN | MEMINFO;

#[repr(C)]
struct MultibootHeader {
    magic: i32,
    flags: i32,
    checksum: i32,
    padding: u32,
}

#[used(linker)]
#[link_section = ".multiboot"]
#[no_mangle]
static MULTIBOOT: MultibootHeader = MultibootHeader {
    magic: MAGIC,
    flags: ALIGN | MEMINFO,
    checksum: -(MAGIC + FLAGS),
    padding: 0,
};

#[no_mangle]
extern "C" fn _start() -> ! {
    loop {}
}
