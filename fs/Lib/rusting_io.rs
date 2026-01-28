/* src/rusting_io.rs - by KhanMarauder aka RustyHoodie323
 */

#![allow(unused_mut)]
#![allow(dead_code)]




pub unsafe fn io_init() -> u8 {
	0
}


pub mod uart {
	use volatile::Volatile;
	
	pub const BUFFER_HEIGHT: usize = 25;
	pub const BUFFER_WIDTH: usize = 80;
	

	
	#[macro_export]
	macro_rules! io_print {
		() => {}; // Don't do anything if no arguments are supplied
		("") => {}; // Don't do anything if an empty string is supplied
		($fmt:expr) => {$crate::rusting_io::uart::io_print_string(concat!($fmt, "\0"), None);};
		($fmt:expr, $($arg:tt)*) => {
			let s = alloc::format!($fmt, $($arg)*);
			$crate::rusting_io::uart::io_print_string(alloc::format!("{}\0", s).as_str(), None);
		};
	}

	#[macro_export]
	macro_rules! io_println {
		// Print newline if no arguments are supplied
		() => {
			unsafe {$crate::rusting_io::uart::io_print_string("\n\0", None);}
		};
		// Print newline if an empty string is supplied
		("") => {
			unsafe {$crate::rusting_io::uart::io_print_string("\n\0", None);}
		};
		// Print text + newline if arguments are supplied
		($fmt:expr) => {$crate::rusting_io::uart::io_print_string(concat!($fmt, "\n\0"), None);};
		($fmt:expr, $($arg:tt)*) => {
			let s = alloc::format!($fmt, $($arg)*);
			$crate::rusting_io::uart::io_print_string(alloc::format!("{}\n\0", s).as_str(), None);
		};
	}
	

	pub fn io_print_char(char: u8, mut colorcode: Option<ColorCode>) {
		match colorcode {
			Some(_) => {},
			None => {
				colorcode = Some(ColorCode::new(Color::White, Color::Black));
			},
		}

		let mut writer = Writer {
			column_position: 0,
			color_code: colorcode.unwrap(),
			buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
		};

		writer.write_byte(char);
	}

	#[allow(improper_ctypes_definitions)]
	pub fn io_print_string(string: &str, mut colorcode: Option<ColorCode>) {
		match colorcode {
			Some(_) => {},
			None => {
				colorcode = Some(ColorCode::new(Color::White, Color::Black));
			},
		}

		let mut writer = Writer {
			column_position: 0,
			color_code: colorcode.unwrap(),
			buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
		};

		writer.write_string(string);
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	#[repr(C)]
	pub struct ScreenChar {
		ascii_character: u8,
		color_code: ColorCode,
	}

	pub struct Writer {
		column_position: usize,
		color_code: ColorCode,
		buffer: &'static mut Buffer,
	}

	impl Writer {
		pub fn write_byte(&mut self, byte: u8) {
			match byte {
				b'\n' => self.new_line(),
				byte => {
					if self.column_position >= BUFFER_WIDTH {
						self.new_line();
					}

					let row = BUFFER_HEIGHT - 1;
					let col = self.column_position;

					let color_code = self.color_code;
					self.buffer.chars[row][col].write(ScreenChar {
						ascii_character: byte,
						color_code,
					});
					self.column_position += 1;
				}
			}
		}

		pub fn write_string(&mut self, s: &str) {
			for byte in s.bytes() {
				match byte {
					// Printable ASCII byte or newline
					0x20..=0x7e | b'\n' => self.write_byte(byte),
					// Not part of printable ASCII range
					_ => self.write_byte(0xfe),
				}
			}
		}

		fn new_line(&mut self) {
			for row in 1..BUFFER_HEIGHT {
				for col in 0..BUFFER_WIDTH {
					let character = self.buffer.chars[row][col].read();
					self.buffer.chars[row - 1][col].write(character);
				}
			}
			self.clear_row(BUFFER_HEIGHT - 1);
			self.column_position = 0;
		}

		fn clear_row(&mut self, row: usize) {
			let blank = ScreenChar {
				ascii_character: b' ',
				color_code: self.color_code,
			};
			for col in 0..BUFFER_WIDTH {
				self.buffer.chars[row][col].write(blank);
			}
		}
	}
	
	#[repr(transparent)]
	pub struct Buffer {
		chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
	}

	#[allow(dead_code)]
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	#[repr(u8)]
	pub enum Color {
		Black = 0,
		Blue = 1,
		Green = 2,
		Cyan = 3,
		Red = 4,
		Magenta = 5,
		Brown = 6,
		LightGray = 7,
		DarkGray = 8,
		LightBlue = 9,
		LightGreen = 10,
		LightCyan = 11,
		LightRed = 12,
		Pink = 13,
		Yellow = 14,
		White = 15,
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	#[repr(transparent)]
	pub struct ColorCode(u8);

	impl ColorCode {
		fn new(foreground: Color, background: Color) -> ColorCode {
			ColorCode((background as u8) << 4 | (foreground as u8))
		}
	}
}
