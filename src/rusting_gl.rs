/* src/graphics.rs - by KhanMarauder aka RustyHoodie323
 * This is the graphics driver for RustingOS.
 */

#![allow(unused_mut)]
#![allow(dead_code)]




// Mailbox constants (hardcoded so that QEMU mode is automatic without running init)
pub const MBOX_BASE:   usize = 0xFE00B880;
pub const MBOX_STATUS: usize = MBOX_BASE + 0x18;
pub const MBOX_WRITE:  usize = MBOX_BASE + 0x20;
pub const MBOX_READ:   usize = MBOX_BASE;
pub const MBOX_FULL:   u32 = 0x8000_0000;
pub const MBOX_EMPTY:  u32 = 0x4000_0000;

pub static mut FB_PTR: *mut u32 = core::ptr::null_mut::<u32>();
pub static mut FB_WIDTH: u32 = 0;
pub static mut FB_HEIGHT: u32 = 0;
pub static mut FB_PITCH: u32 = 0;



#[unsafe(no_mangle)]
pub unsafe extern "C" fn gl_init() -> u8 {
	const WIDTH: u32 = 800;
	const HEIGHT: u32 = 600;
	const CHANNEL: u32 = 8;

	// -----------------------------
	// 1. Set physical resolution
	// -----------------------------
	let mut phys = FbLetter {
		size: core::mem::size_of::<FbLetter>() as u32,
		req: 0,
		tag: FbLetterTags::FramebufferResizePhysical as u32,
		buf_size: 8,
		req_size: 8,
		value1: WIDTH,
		value2: HEIGHT,
		end_tag: 0,
	};

	mailbox_write(CHANNEL, phys.address());
	mailbox_read(CHANNEL);
	if phys.req != 0 { return 1; } // Check for errors

	// -----------------------------
	// 2. Set color depth (32-bit)
	// -----------------------------
	let mut depth = FbLetter {
		size: core::mem::size_of::<FbLetter>() as u32,
		req: 0,
		tag: FbLetterTags::FramebufferSetColorDepth as u32,
		buf_size: 4,
		req_size: 4,
		value1: FbRGB::ARGB8888 as u32,
		value2: 0,
		end_tag: 0,
	};

	mailbox_write(CHANNEL, depth.address());
	mailbox_read(CHANNEL);
	if depth.req != 0 { return 1; } // Check for errors

	// -----------------------------
	// 3. Allocate framebuffer
	// -----------------------------
	let mut alloc = FbLetter {
		size: core::mem::size_of::<FbLetter>() as u32,
		req: 0,
		tag: FbLetterTags::FramebufferAllocFb as u32,
		buf_size: 8,
		req_size: 8,
		value1: 16, // alignment
		value2: 0,  // GPU will fill size
		end_tag: 0,
	};

	mailbox_write(CHANNEL, alloc.address());
	mailbox_read(CHANNEL);
	if alloc.req != 0 { return 1; } // Check for errors

	// GPU returns framebuffer pointer in value1
	let fb_ptr = alloc.value1 & 0x3FFF_FFFF; // mask to bus address

	// -----------------------------
	// 4. Get pitch
	// -----------------------------
	let mut pitch = FbLetter {
		size: core::mem::size_of::<FbLetter>() as u32,
		req: 0,
		tag: FbLetterTags::FramebufferGetPitch as u32,
		buf_size: 4,
		req_size: 4,
		value1: 0,
		value2: 0,
		end_tag: 0,
	};

	mailbox_write(CHANNEL, pitch.address());
	if pitch.req != 0 { return 1; } // Check for errors
	mailbox_read(CHANNEL);

	let pitch_bytes = pitch.value1;

	// -----------------------------
	// Store framebuffer info globally
	// -----------------------------
	FB_PTR = fb_ptr as *mut u32;
	FB_WIDTH = WIDTH;
	FB_HEIGHT = HEIGHT;
	FB_PITCH = pitch_bytes;
	
	0
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn gl_clear_screen(color: u32) {
	gl_set_rect(0, 0, FB_WIDTH, FB_HEIGHT, color);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gl_set_pixel(x: u32, y: u32, color: u32) {
	if x >= FB_WIDTH || y >= FB_HEIGHT {return;} // Bounds checking
	if FB_PTR.is_null() || color < 0x01000000 {return;} // If framebuffer isn't allocated or color has 0 alpha, then exit
	let ptr = FB_PTR.add((y as usize * (FB_PITCH as usize / 4)) + x as usize);
	core::ptr::write_volatile(ptr, color);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gl_get_pixel(x: u32, y: u32) -> u32 {
	if FB_PTR.is_null() {return 0x00000000;} // If framebuffer isn't allocated, then return an empty color
	let ptr = FB_PTR.add((y as usize * (FB_PITCH as usize / 4)) + x as usize);
	core::ptr::read_volatile(ptr)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gl_set_rect(x: u32, y: u32, w: u32, h: u32, color: u32) {
	const CHANNEL: u32 = 8;
	let mut msg = FbFillRect {
		size: core::mem::size_of::<FbFillRect>() as u32,
		req: 0,
		tag: FbLetterTags::FramebufferFillRect as u32,
		buf_size: 16,
		req_size: 16,
		x,
		y,
		w,
		h,
		color,
		end_tag: 0,
	};
	
	mailbox_write(CHANNEL, msg.into_addr()); 
	mailbox_read(CHANNEL);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gl_copy_rect(_fx: u32, _fy: u32, _fw: u32, _fh: u32, _tx: u32, _ty: u32, _tw: u32, _th: u32) {
	if FB_PTR.is_null() {return;} // If framebuffer isn't allocated, then exit
	/* TODO */
}


/**
 * rustos_gl::mailbox_write(u8, u32) | rustos_gl::mailbox_read(u8)
 * Channel designations
 *  - 0 | Power Management
 *  - 1 | Framebuffer (old interface)
 *  - 2 | Virtual UART
 *  - 3 | VCHIQ
 *  - 8 | Property Tags (modern interface)
 */
#[inline(always)]
fn mailbox_write(channel: u32, data: u32) {
	let value = (data & !0xF) | (channel & 0xF);
	unsafe {
		while core::ptr::read_volatile(MBOX_STATUS as *const u32) & MBOX_FULL != 0 {}
		core::ptr::write_volatile(MBOX_WRITE as *mut u32, value);
	}
}

#[inline(always)]
fn mailbox_read(channel: u32) -> u32 {
	unsafe {
		loop {
			// Wait until mailbox has 'mail'
			while core::ptr::read_volatile(MBOX_STATUS as *const u32) & MBOX_EMPTY != 0 {}
			let value = core::ptr::read_volatile(MBOX_READ as *const u32);
			if (value & 0xF) == channel {
				return value & !0xF; // return the address part
			}
		}
	}
}


/**
 * rustos_gl::FbLetter
 * This is a struct used to pass instructions to the GPU
 * Tags
 * 
 * rustos_gl::FbLetterTags
 * This is an enum to store command tags to be passed to the GPU
 */
#[repr(u32)]
pub enum FbLetterTags {
	// Framebuffer tags
	FramebufferResizePhysical   = 0x00048003,
	FramebufferResizeVirtual    = 0x00048004,
	FramebufferSetColorDepth    = 0x00048005,
	FramebufferSetColorOrder    = 0x00048006,
	FramebufferSetVirtualOffset = 0x00048009,
	FramebufferSetColorPalette  = 0x0004800F,
	FramebufferGetPitch         = 0x00040008,
	FramebufferGetDisplaySize   = 0x00040003, // FIXED
	FramebufferAllocFb          = 0x00040001,

	// GPU accelerated operations
	FramebufferFillRect         = 0x0004800A,
	FramebufferCopyRect         = 0x0004800B,
}

#[repr(u32)]
pub enum FbRGB {
	// RGB color depth
	ARGB8888 = 32,
	RGB888   = 24,
	RGB565   = 16,

	// RGB color order
	RGB = 0,
	BGR = 1,
}

#[repr(C, align(16))]
struct FbLetter {
	size: u32,
	req: u32,
	
	tag: u32,
	buf_size: u32,
	req_size: u32,
	
	value1: u32,
	value2: u32,
	
	end_tag: u32,
}

impl FbLetter {
	pub fn address(&self) -> u32 {
		self as *const _ as u32
	}
}

#[repr(C, align(16))]
struct FbFillRect {
	size: u32,
	req: u32,
	tag: u32,
	buf_size: u32,
	req_size: u32,
	x: u32,
	y: u32,
	w: u32,
	h: u32,
	color: u32,
	end_tag: u32,
}

impl FbFillRect {
	pub fn into_addr(&self) -> u32 {
		self as *const _ as u32
	}
}
