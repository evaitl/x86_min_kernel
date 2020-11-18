#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cpuio::{inb, outb};

const PORT: u16 = 0x3f8;

static HELLO: &[u8] = b"Hello world from the kernel\n";
unsafe fn is_transmit_empty() -> bool {
    inb(PORT + 5) & 0x20 != 0
}
unsafe fn write_serial(ch: u8) {
    while !is_transmit_empty() {}
    outb(ch, PORT);
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    outb(0x00, PORT + 1); // Disable all interrupts
    outb(0x80, PORT + 3); // Enable DLAB (set baud rate divisor)
    outb(0x03, PORT + 0); // Set divisor to 3 (lo byte) 38400 baud
    outb(0x00, PORT + 1); //                  (hi byte)
    outb(0x03, PORT + 3); // 8 bits, no parity, one stop bit
    outb(0xc7, PORT + 2); // Enable FIFO, clear them, with 14-byte threshold
    outb(0x0b, PORT + 4); // IRQs enabled, RTS/DSR set

    loop {
        for (_i, &byte) in HELLO.iter().enumerate() {
            write_serial(byte);
        }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
