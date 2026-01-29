/* src/rusting_io.rs - by KhanMarauder aka RustyHoodie323
 */

#![allow(unused_mut)]
#![allow(dead_code)]




pub unsafe fn io_init() -> u8 {
	0
}


pub mod uart {
	use string::String;
	
	static mut UART_ADDR: *mut u8 = 0x09000000 as *mut u8;

	pub fn print_ps1(username: &str, hostname: &str, location: &str) {
		let uname_str = String::from(username);
		let hname_str = String::from(hostname);
		let loc_str = String::from(location);

		let uname_color = ANSIColor::BrightOrange.as_string();
		let hname_color = ANSIColor::BrightRed.as_string();
		let loc_color = ANSIColor::Blue.as_string();

		let defaultcolor = ANSIColor::White.as_string();

		print(
			// Username
			uname_color + uname_str + defaultcolor + String::from("@") +
			// Hostname
			hname_color + hname_str + defaultcolor + String::from(": ") +
			// Location
			loc_color + loc_str + defaultcolor + String::from("$ ")
		)
	}

	pub unsafe fn print(string: &str) {
		for c in string.bytes() {UART_ADDR.write_volatile(c);}
	}

	#[derive(Debug, Clone, Copy)]
	pub enum ANSIColor {
		DarkRed,
		Red,
		BrightRed,

		DarkOrange,
		Orange,
		BrightOrange,

		DarkYellow,
		Yellow,
		BrightYellow,

		DarkGreen,
		Green,
		BrightGreen,

		DarkBlue,
		Blue,
		BrightBlue,

		DarkIndigo,
		Indigo,
		BrightIndigo,

		DarkViolet,
		Violet,
		BrightViolet,
	}
	
	impl ANSIColor {
		pub fn as_str(self) -> &'static str {
			match self {
				// Red
				ANSIColor::DarkRed    => "\x1b[31m",
				ANSIColor::Red        => "\x1b[91m",
				ANSIColor::BrightRed  => "\x1b[38;5;196m",

				// Orange (not in basic ANSI, so 256‑color approximations)
				ANSIColor::DarkOrange   => "\x1b[38;5;130m",
				ANSIColor::Orange       => "\x1b[38;5;208m",
				ANSIColor::BrightOrange => "\x1b[38;5;214m",

				// Yellow
				ANSIColor::DarkYellow   => "\x1b[33m",
				ANSIColor::Yellow       => "\x1b[93m",
				ANSIColor::BrightYellow => "\x1b[38;5;226m",

				// Green
				ANSIColor::DarkGreen    => "\x1b[32m",
				ANSIColor::Green        => "\x1b[92m",
				ANSIColor::BrightGreen  => "\x1b[38;5;46m",

				// Blue
				ANSIColor::DarkBlue     => "\x1b[34m",
				ANSIColor::Blue         => "\x1b[94m",
				ANSIColor::BrightBlue   => "\x1b[38;5;27m",

				// Indigo (again, 256‑color approximations)
				ANSIColor::DarkIndigo   => "\x1b[38;5;54m",
				ANSIColor::Indigo       => "\x1b[38;5;55m",
				ANSIColor::BrightIndigo => "\x1b[38;5;57m",

				// Violet
				ANSIColor::DarkViolet   => "\x1b[35m",
				ANSIColor::Violet       => "\x1b[95m",
				ANSIColor::BrightViolet => "\x1b[38;5;201m",
			}
		}

		pub fn as_string(self) -> String {
			String::from(self.as_str())
		}

		pub fn name(self) -> &'static str {
			match self {
				ANSIColor::DarkRed    => "dark red",
				ANSIColor::Red        => "red",
				ANSIColor::BrightRed  => "bright red",

				ANSIColor::DarkOrange   => "dark orange",
				ANSIColor::Orange       => "orange",
				ANSIColor::BrightOrange => "bright orange",

				ANSIColor::DarkYellow   => "dark yellow",
				ANSIColor::Yellow       => "yellow",
				ANSIColor::BrightYellow => "bright yellow",

				ANSIColor::DarkGreen    => "dark green",
				ANSIColor::Green        => "green",
				ANSIColor::BrightGreen  => "bright green",

				ANSIColor::DarkBlue     => "dark blue",
				ANSIColor::Blue         => "blue",
				ANSIColor::BrightBlue   => "bright blue",

				ANSIColor::DarkIndigo   => "dark indigo",
				ANSIColor::Indigo       => "indigo",
				ANSIColor::BrightIndigo => "bright indigo",

				ANSIColor::DarkViolet   => "dark violet",
				ANSIColor::Violet       => "violet",
				ANSIColor::BrightViolet => "bright violet",
			}
		}

		pub fn name_string(self) -> String {
			String::from(self.name())
		}
	}
}
