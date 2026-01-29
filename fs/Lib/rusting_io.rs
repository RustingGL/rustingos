/* src/rusting_io.rs - by KhanMarauder aka RustyHoodie323
 */

#![allow(unused_mut)]
#![allow(dead_code)]




pub unsafe fn io_init() -> u8 {
	0
}


pub mod uart {
	static mut UART_ADDR: *mut u8 = 0x09000000 as *mut u8;

	pub unsafe fn print(string: &str) {
		for c in string.bytes() {
			UART_ADDR.write_volatile(c);
		}
	}
}
