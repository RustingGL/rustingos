/* src/kernel.rs - by KhanMarauder aka RustyHoodie323
 * This is the kernel's rust sorce code. pub unsafe
 * extern "C" fn _start() -> ! is the main entry
 * function. It initializes everything and runs the
 * startup processes.
 */
 
#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
#![feature(alloc_error_handler)]

mod rusting_io;
mod rusting_alloc;
use core::arch::*;

// Define allocator
#[global_allocator]
pub static ALLOCATOR: rusting_alloc::Allocator = rusting_alloc::Allocator::new();

// Define stack
pub const STACK_SIZE: usize = 2 * 1024 * 1024;
#[unsafe(no_mangle)]
#[unsafe(link_section = ".stack")]
pub static mut STACK: [u8; STACK_SIZE] = [0_u8; STACK_SIZE]; // Define 2MB stack

// Bootstrap
global_asm!(include_str!("boot.s"), sym STACK, const STACK_SIZE);




/** crate::main() -> !
	### What it is
	This function is the main entry point for
	the kernel and is run when the OS boots.

	###

	### Safety
	This function is unsafe so that it doesn't
	need to have a multitude of unsafe blocks
	to run the various functions that are
	unsafe. Most of these unsafe functions are
	unsafe becasue they need to access memory in
	a dangerous way; like the mailbox functions.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() {
	// Init rusting_io
	assert_eq!(rusting_io::io_init(), 0);

	rusting_io::uart::print(include_str!("../fs/System/boot/splash.txt"));

	#[allow(clippy::empty_loop)]
	loop {}
}


#[alloc_error_handler]
fn alloc_error(_: core::alloc::Layout) -> ! {
	loop {}
}


/** crate::panic(&PanciInfo<'_>) -> !
	### Description
	Simple panic handler for kernel
	that halts the CPU when panicking.
 */
#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
	loop {}
}


#[cfg(test)]
mod test {}
