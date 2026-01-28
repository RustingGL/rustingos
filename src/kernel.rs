/* src/kernel.rs - by KhanMarauder aka RustyHoodie323
 * This is the kernel's rust sorce code. pub unsafe
 * extern "C" fn _start() -> ! is the main entry
 * function. It initializes everything and runs the
 * startup processes.
 */
 
#![no_std]
#![no_main]
#![cfg(not(test))]
#![allow(unsafe_op_in_unsafe_fn)]

mod boot;
mod rusting_gl;
mod rusting_io;
mod rusting_usb;
mod rusting_encrypt;
mod rusting_file;
extern crate alloc;
//use core::arch::aarch64;
use core::panic::PanicInfo;
use linked_list_allocator::LockedHeap;
#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();
pub static mut HEAP_START: *mut u8 = core::ptr::null_mut::<u8>();
pub static HEAP_SIZE: usize = 0;




/**
 * crate::_start() -> !
 * 
 * # What it is
 * This function is the main entry point for the kernel and is run when the OS
 * boots.
 * 
 * # Safety
 * This function is unsafe so that it doesn't need to have a multitude of unsafe
 * blocks to run the various functions that are unsafe. Most of these unsafe
 * functions are unsafe becasue they need to unsafely access memory.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rusting_k_start() {
	// ------------------
	// - Init libraries -
	// ------------------
	{
		// Init allocator
		ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
		
		// Init rusting_io
		let mut init_code = rusting_io::io_init();
		assert_eq!(init_code, 0);

		// Init rusting_gl
		init_code = rusting_gl::gl_init();
		assert_eq!(init_code, 0);

		// Init encryption
		init_code = rusting_encrypt::cryptography::crypt_init();
		assert_eq!(init_code, 0);

		// Init random
		init_code = rusting_encrypt::random::rand_init();
		assert_eq!(init_code, 0);

		// Init pseudo-random number (PRN)
		init_code = rusting_encrypt::prn::prn_init();
		assert_eq!(init_code, 0);

		// Init USB driver
		init_code = rusting_usb::usb_init();
		assert_eq!(init_code, 0);
	}
	
	loop {
		rusting_gl::gl_clear_screen(0xFFF5F5F5);
		rusting_gl::gl_set_rect(rusting_gl::FB_WIDTH/3, rusting_gl::FB_HEIGHT/3, rusting_gl::FB_WIDTH/3, rusting_gl::FB_HEIGHT/3, 0xFF000000);
	}
}


/**
 * crate::rusting_k_cprintln(*const u8, usize)
 * This is a simplified version of the
 * println!() macro to be called from C or
 * Assembly.
 * 
 * # Safety
 * This function is unsafe because it has to
 * do dangerous conversions.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rusting_k_cprintln(ptr: *const u8) {
	if ptr.is_null() {return;}
	let mut p = ptr;
	while *p != 0 {
		rusting_io::uart::io_print_char(*p, None);
		p = p.add(1);
	}
}


#[unsafe(no_mangle)]
pub extern "C" fn kernel_failure() {
	io_print!("Kernel failure occurred.");
	panic!("Kernel failure occurred.");
}


#[panic_handler]
fn panic_handler(info: &PanicInfo<'_>) -> ! {
	io_print!("{}", info);
	loop {}
}
