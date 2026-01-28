/* src/rusting_encrypt.rs - by KhanMarauder aka Rustyhoodie323
 * This is the library for encryption, random number
 * generation, and pseudo random numbers (PRNs).
 */

#![allow(unused_mut)]
#![allow(dead_code)]




pub mod cryptography {
	use alloc::vec::Vec;
	
	pub extern "C" fn crypt_init() -> u8 {
		0
	}


	pub fn crypt_xor_hash_u32(source: Vec<u32>) -> Vec<u32> {
		let mut output = source.clone();
		for (i, &data) in source.iter().enumerate() {
			output[i] ^= 0xABCDEF12 ^ data;
		}
		output
	}

	pub fn crypt_xor_hash_u16(source: Vec<u16>) -> Vec<u16> {
		let mut output = source.clone();
		for (i, &data) in source.iter().enumerate() {
			output[i] ^= 0xDCBA ^ data;
		}
		output
	}

	pub fn crypt_xor_hash_u8(source: Vec<u8>) -> Vec<u8> {
		let mut output = source.clone();
		for (i, &data) in source.iter().enumerate() {
			output[i] ^= 0x90 ^ data;
		}
		output
	}
}


pub mod random {
	pub fn rand_init() -> u8 {
		0
	}
}


pub mod prn {
	pub fn prn_init() -> u8 {
		0
	}
}
