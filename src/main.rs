#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

// This is the assembly that OpenSBI hits first
global_asm!(
    r#"
    .section .text.entry
    .globl _start
    _start:
        # Set up a stack pointer (sp)
        la sp, stack_top

        # Jump to our Rust function
        j rust_main
    "#
);

// The UART (Serial Port) address for QEMU 'virt' machine
const UART0: *mut u8 = 0x1000_0000 as *mut u8;

// A simple function to print a string to the console
fn print(s: &str) {
    for c in s.chars() {
        unsafe {
            // Write the character to the UART data register
            core::ptr::write_volatile(UART0, c as u8);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    // Hypervisor starts!
    print("\n\r--- Hypervisor Starting ---\n\r");
    print("Mode: HS-Mode (Hypervisor-Supervisor)\n\r");
    print("Status: Successfully booted from OpenSBI\n\r");
    print("---------------------------\n\r");

    // For now, let's just loop forever.
    // In a real hypervisor, we would set up G-stage paging here.
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print("\n\r!!! HYPERVISOR PANIC !!!\n\r");
    loop {}
}

// Create a small stack in the BSS section
#[unsafe(no_mangle)]
#[unsafe(link_section = ".bss.stack")]
static mut STACK: [u8; 4096] = [0; 4096];

#[unsafe(no_mangle)]
static stack_top: u8 = 0; // This is a simplification for the assembly
