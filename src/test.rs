mod rusting_encrypt;

fn main() {
	if rusting_encrypt::cryptography::crypt_init() != 0 {
		panic!("couldn't init cryptography");
	}

	let string = b"Hello, world!";
	let hashed_string = rusting_encrypt::cryptography::crypt_xor_hash_u8(string.to_vec());
	println!("Original string: {}, original array: {:?}.", str::from_utf8(string).unwrap(), string);
	println!("Encrypted array: {:?}.", hashed_string);
}
